use bevy::input::keyboard::Key;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;

use crate::plugins::common::clipboard::clipboard_plugin;
use crate::plugins::common::clipboard::ClipboardResource;
use crate::plugins::common::theme::focus::FocusedEntity;
use crate::plugins::common::theme::node::ThemedBackgroundColor;
use crate::plugins::common::theme::node::ThemedBorderColor;
use crate::plugins::common::theme::node::ThemedBorderRadius;
use crate::plugins::common::theme::node::ThemedBorderRect;
use crate::plugins::common::theme::text::ThemedFontWeight;
use crate::plugins::common::theme::text::ThemedTextColor;

pub fn text_input_plugin(app: &mut App) {
    app.add_plugins(clipboard_plugin)
        .insert_resource(BlinkTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .add_systems(
            Update,
            (
                text_input_focus_system,
                text_input_cursor_blink_system,
                typing_system,
            ),
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
    pub is_empty: bool,
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
            is_empty: true,
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

fn text_input_focus_system(
    focused_entity: Res<FocusedEntity>,
    container_query: Query<(&TextInputContainer, &Children), With<TextInputContainer>>,
    mut text_query: Query<&mut Text, With<TextInputText>>,
    mut text_cursor_query: Query<&mut Visibility, With<TextInputCursor>>,
) {
    // Unfocus the last focused entity if it is a text input
    if let Some(last_focused_entity) = focused_entity.last {
        let last_container_children_result = container_query.get(last_focused_entity);
        if let Ok((text_input_data, container_children)) = last_container_children_result {
            // Show the placeholder if the text input is empty
            if text_input_data.is_empty {
                let mut text = text_query.get_mut(container_children[0]).unwrap();
                text.0 = text_input_data.placeholder_text.clone();
            }
            // Hide the cursor
            let mut text_cursor_visibility =
                text_cursor_query.get_mut(container_children[1]).unwrap();
            *text_cursor_visibility = Visibility::Hidden;
        }
    }

    // Focus the current focused entity if it is a text input
    if let Some(current_focused_entity) = focused_entity.current {
        let current_container_children_result = container_query.get(current_focused_entity);
        if let Ok((text_input_data, container_children)) = current_container_children_result {
            // Hide the placeholder if the text input is empty
            if text_input_data.is_empty {
                let mut text = text_query.get_mut(container_children[0]).unwrap();
                text.0 = "".into();
            }
            // Show the cursor
            let mut text_cursor_visibility =
                text_cursor_query.get_mut(container_children[1]).unwrap();
            *text_cursor_visibility = Visibility::Visible;
        }
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
        for mut text_cursor_background_color in text_cursor_query.iter_mut() {
            let color = &mut text_cursor_background_color.0;
            let new_alpha = if color.alpha() >= 1.0 { 0.0 } else { 1.0 };
            color.set_alpha(new_alpha);
        }
    }
}

fn typing_system(
    #[cfg(target_family = "wasm")] mut commands: Commands,
    focused_entity: Res<FocusedEntity>,
    mut container_query: Query<(&mut TextInputContainer, &Children), With<TextInputContainer>>,
    mut text_query: Query<&mut Text, With<TextInputText>>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut clipboard_resource: ResMut<ClipboardResource>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    // Get the current focused entity
    if focused_entity.current.is_none() {
        return;
    }
    let current_focused_entity = focused_entity.current.unwrap();

    // Check if it's a text input container
    let container_result = container_query.get_mut(current_focused_entity);
    if container_result.is_err() {
        return;
    }
    let (mut text_input_data, container_children) = container_result.unwrap();

    // Get the text input val
    let text_entity = container_children[0];
    let mut text = text_query.get_mut(text_entity).unwrap();
    let text_input_value = &mut text.0;

    // Handle the keyboard event
    for keyboard_input_event in keyboard_input_events.read() {
        // We don't care about key releases, only key presses
        if keyboard_input_event.state == ButtonState::Released {
            continue;
        }

        let control_keys = [
            KeyCode::SuperLeft,
            KeyCode::SuperRight,
            KeyCode::ControlLeft,
            KeyCode::ControlRight,
        ];

        let mut is_empty = false;

        // Handle the key press
        match &keyboard_input_event.logical_key {
            Key::Backspace if keys.any_pressed(control_keys) => {
                text_input_value.clear();
                is_empty = text_input_value.is_empty();
            }
            Key::Backspace => {
                text_input_value.pop();
                is_empty = text_input_value.is_empty();
            }
            Key::Character(input) if keys.any_pressed(control_keys) => {
                match input.as_str() {
                    "c" => {
                        clipboard_resource.copy(text_input_value.clone());
                    }
                    "v" => {
                        #[cfg(not(target_family = "wasm"))]
                        clipboard_resource.native_paste(text_input_value);
                        #[cfg(target_family = "wasm")]
                        clipboard_resource.wasm_paste(&mut commands, text_entity);
                    }
                    _ => {}
                };
            }
            Key::Character(input) => {
                text_input_value.push_str(input);
            }
            Key::Space => {
                text_input_value.push(' ');
            }
            Key::Copy => {
                clipboard_resource.copy(text_input_value.clone());
            }
            Key::Paste => {
                #[cfg(not(target_family = "wasm"))]
                clipboard_resource.native_paste(text_input_value);
                #[cfg(target_family = "wasm")]
                clipboard_resource.wasm_paste(&mut commands, text_entity);
            }
            _ => {}
        };

        // Finally, update the is_empty flag for the text input
        text_input_data.is_empty = is_empty;
    }
}
