use bevy::input::keyboard::Key;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::input_focus::InputFocus;
use bevy::prelude::*;
use bevy::ui::UiSystems;
#[cfg(target_family = "wasm")]
use bevy_defer::AsyncCommandsExtension;

use crate::plugins::common::clipboard::clipboard_plugin;
use crate::plugins::common::clipboard::ClipboardResource;
use crate::plugins::common::theme::node::ThemedBackgroundColor;
use crate::plugins::common::theme::node::ThemedBorderColor;
use crate::plugins::common::theme::node::ThemedBorderRadius;
use crate::plugins::common::theme::node::ThemedBorderRect;
use crate::plugins::common::theme::text::ThemedFontWeight;
use crate::plugins::common::theme::text::ThemedTextColor;

pub fn text_input_plugin(app: &mut App) {
    app.add_plugins(clipboard_plugin)
        .insert_resource(BlinkTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .add_systems(Update, (text_input_cursor_blink_system, typing_system))
        .add_systems(
            PostUpdate,
            update_text_input_display.before(UiSystems::Content),
        );
}

#[derive(Component, Clone)]
#[require(
    Node,
    Interaction,
    ThemedBackgroundColor,
    ThemedBorderColor,
    ThemedBorderRadius,
    ThemedBorderRect
)]
pub struct TextInputContainer {
    pub placeholder_text: String,
    pub value: String,
}

#[derive(Component)]
#[require(Text, ThemedFontWeight::Regular, ThemedTextColor)]
struct TextInputText;

#[derive(Component)]
#[require(Node)]
pub struct TextInputCursor;

#[derive(Default)]
pub struct TextInputBundleOptions {
    pub placeholder_text: String,
    pub text_font: TextFont,
    pub container_node: Node,
    pub text_node: Node,
}

#[must_use]
pub fn text_input_bundle(options: TextInputBundleOptions) -> impl Bundle {
    let TextInputBundleOptions {
        placeholder_text,
        text_font,
        container_node,
        text_node,
    } = options;

    let font_size = text_font.font_size;

    let text_input_text_bundle = (
        TextInputText,
        Text::new(placeholder_text.clone()),
        Node {
            height: Val::Px(text_font.font_size),
            margin: UiRect::vertical(Val::Px(8.0)),
            justify_content: JustifyContent::Center,
            ..text_node
        },
        text_font,
    );

    let text_input_cursor_bundle = (
        TextInputCursor,
        Node {
            width: Val::Px(1.0),
            height: Val::Px(font_size + 4.0),
            ..default()
        },
        Visibility::Hidden,
    );

    (
        TextInputContainer {
            placeholder_text,
            value: String::new(),
        },
        Node {
            overflow: Overflow {
                x: OverflowAxis::Hidden,
                y: OverflowAxis::Hidden,
            },
            align_items: AlignItems::Center,
            ..container_node
        },
        children![text_input_text_bundle, text_input_cursor_bundle],
    )
}

fn update_text_input_display(
    input_focus: Res<InputFocus>,
    container_query: Query<(Entity, &TextInputContainer, &Children)>,
    mut text_query: Query<&mut Text, With<TextInputText>>,
    mut text_cursor_query: Query<&mut Visibility, With<TextInputCursor>>,
) {
    for (entity, text_input_data, container_children) in &container_query {
        let is_focused = input_focus.get() == Some(entity);
        let display = if text_input_data.value.is_empty() && !is_focused {
            &text_input_data.placeholder_text
        } else {
            &text_input_data.value
        };
        let mut text = text_query.get_mut(container_children[0]).unwrap();
        if text.0 != *display {
            text.0.clone_from(display);
        }
        let cursor_visibility = if is_focused {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        text_cursor_query
            .get_mut(container_children[1])
            .unwrap()
            .set_if_neq(cursor_visibility);
    }
}

#[derive(Resource)]
struct BlinkTimer(Timer);

fn text_input_cursor_blink_system(
    mut blink_timer: ResMut<BlinkTimer>,
    time: Res<Time>,
    mut text_cursor_query: Query<&mut BackgroundColor, With<TextInputCursor>>,
) {
    if blink_timer.0.tick(time.delta()).just_finished() {
        // Toggle the cursors color alpha
        for mut text_cursor_background_color in &mut text_cursor_query {
            let color = &mut text_cursor_background_color.0;
            let new_alpha = if color.alpha() >= 1.0 { 0.0 } else { 1.0 };
            color.set_alpha(new_alpha);
        }
    }
}

pub(crate) fn typing_system(
    #[cfg(target_family = "wasm")] mut commands: Commands,
    input_focus: Res<InputFocus>,
    mut container_query: Query<&mut TextInputContainer>,
    mut keyboard_input_events: MessageReader<KeyboardInput>,
    mut clipboard_resource: ResMut<ClipboardResource>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Some(entity) = input_focus.get() else {
        keyboard_input_events.clear();
        return;
    };
    let Ok(mut text_input) = container_query.get_mut(entity) else {
        keyboard_input_events.clear();
        return;
    };

    let control_pressed = keys.any_pressed([
        KeyCode::SuperLeft,
        KeyCode::SuperRight,
        KeyCode::ControlLeft,
        KeyCode::ControlRight,
    ]);

    for event in keyboard_input_events.read() {
        if event.state == ButtonState::Released {
            continue;
        }

        let key = match &event.logical_key {
            Key::Character(input) if control_pressed => match input.as_str() {
                "c" => &Key::Copy,
                "v" => &Key::Paste,
                _ => continue,
            },
            key => key,
        };
        match key {
            Key::Backspace if control_pressed => {
                text_input.value.clear();
            }
            Key::Backspace => {
                text_input.value.pop();
            }
            Key::Character(input) => {
                text_input.value.push_str(input);
            }
            Key::Space => {
                text_input.value.push(' ');
            }
            Key::Copy => {
                clipboard_resource.copy(&text_input.value);
            }
            Key::Paste => {
                #[cfg(not(target_family = "wasm"))]
                clipboard_resource.native_paste(&mut text_input.value);
                #[cfg(target_family = "wasm")]
                commands.spawn_task(move || async move {
                    let clipboard_text = ClipboardResource::wasm_read_text().await;
                    bevy_defer::fetch!(entity, &mut TextInputContainer).get_mut(|mut text_input| {
                        text_input.value.push_str(&clipboard_text);
                    })
                });
            }
            _ => {}
        }
    }
}
