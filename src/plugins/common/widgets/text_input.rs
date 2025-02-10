use arboard::Clipboard;
use bevy::input::keyboard::Key;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use derive_builder::Builder;

use crate::plugins::common::theme::focus::FocusedEntity;
use crate::plugins::common::theme::Themed;

use super::Spawnable;

#[derive(Resource)]
struct ClipboardResource(Clipboard);

impl Default for ClipboardResource {
    fn default() -> Self {
        Self(Clipboard::new().unwrap())
    }
}

pub fn text_input_plugin(app: &mut App) {
    app.init_resource::<ClipboardResource>()
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
#[require(Themed, Node, Interaction)]
pub struct TextInputContainer {
    pub placeholder_text: String,
    pub is_empty: bool,
}

impl Default for TextInputContainer {
    fn default() -> Self {
        Self {
            placeholder_text: default(),
            is_empty: true,
        }
    }
}

#[derive(Builder)]
#[builder(build_fn(skip), default, public)]
pub struct TextInputWidget {
    text_input_container: TextInputContainer,
    text_font: TextFont,
    container_node: Node,
    text_node: Node,
    // justify_text: JustifyText,
    // background_color: ThemeComponent<BackgroundColor>,
}

impl TextInputWidgetBuilder {
    pub fn build(&self) -> TextInputWidget {
        let TextInputWidgetBuilder {
            text_input_container,
            text_font,
            container_node,
            text_node,
            // justify_text,
            // background_color,
        } = self;
        TextInputWidget {
            text_input_container: text_input_container.clone().unwrap_or_default(),
            text_font: text_font.clone().unwrap_or_default(),
            container_node: container_node.clone().unwrap_or_default(),
            text_node: text_node.clone().unwrap_or_default(),
            // justify_text: justify_text.clone().unwrap_or_default(),
            // background_color: background_color.clone().unwrap_or_default(),
        }
    }
}

#[derive(Component)]
#[require(Themed, Text)]
struct TextInputText;

#[derive(Component)]
#[require(Node)]
pub struct TextInputCursor;

impl Spawnable for TextInputWidget {
    fn spawn_with_components<'a, S: super::Spawn>(
        &self,
        spawner: &'a mut S,
        components: impl Bundle,
    ) -> EntityCommands<'a> {
        let TextInputWidget {
            text_input_container,
            text_font,
            container_node,
            text_node,
            // justify_text,
            // no_cursor,
            // background_color,
        } = self;

        let container_bundle = (
            text_input_container.clone(),
            Node {
                overflow: Overflow {
                    x: OverflowAxis::Hidden,
                    y: OverflowAxis::Hidden,
                },
                align_items: AlignItems::Center,
                ..container_node.clone()
            },
            // BackgroundColor(background_color.clone())
        );

        let text_bundle = (
            TextInputText,
            Text::new(text_input_container.placeholder_text.clone()),
            Node {
                height: Val::Px(text_font.font_size),
                margin: UiRect::vertical(Val::Px(8.0)),
                justify_content: JustifyContent::Center,
                ..text_node.clone()
            },
            text_font.clone(),
        );

        let cursor_bundle = (
            TextInputCursor,
            Node {
                width: Val::Px(1.0),
                height: Val::Px(text_font.font_size + 4.0),
                ..default()
            },
            Visibility::Hidden,
        );

        let mut ec = spawner.spawn((container_bundle, components));
        ec.with_children(|parent| {
            parent.spawn(text_bundle);
            parent.spawn(cursor_bundle);
        });
        return ec;
    }
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

impl ClipboardResource {
    fn copy(&mut self, val: impl Into<String>) {
        self.0.set_text(val.into()).unwrap();
    }

    fn paste(&mut self, destination: &mut String) {
        destination.push_str(
            &self
                .0
                .get_text()
                .unwrap()
                .chars()
                .filter(|&c| c != '\n' && c != '\r')
                .collect::<String>(),
        );
    }
}

fn typing_system(
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
    let mut text = text_query.get_mut(container_children[0]).unwrap();
    let text_input_value = &mut text.0;

    // Handle the keyboard event
    for keyboard_input_event in keyboard_input_events.read() {
        // We don't care about key releases, only key presses
        if keyboard_input_event.state == ButtonState::Released {
            continue;
        }

        #[cfg(target_os = "macos")]
        let control_keys = [KeyCode::SuperLeft, KeyCode::SuperRight];
        #[cfg(not(target_os = "macos"))]
        let control_keys = [KeyCode::ControlLeft, KeyCode::ControlRight];

        // Handle the key press
        match &keyboard_input_event.logical_key {
            Key::Backspace if keys.any_pressed(control_keys) => {
                text_input_value.clear();
            }
            Key::Backspace => {
                text_input_value.pop();
            }
            // // Ignore any input that contains control (special) characters?
            // Key::Character(input) if input.chars().any(|c| c.is_control()) => {
            //     continue;
            // }
            Key::Character(input) if keys.any_pressed(control_keys) => {
                match input.as_str() {
                    "c" => {
                        clipboard_resource.copy(text_input_value.clone());
                    }
                    "v" => {
                        clipboard_resource.paste(text_input_value);
                    }
                    _ => {}
                };
            }
            Key::Character(input) => {
                text_input_value.push_str(&input);
            }
            Key::Space => {
                text_input_value.push(' ');
            }
            Key::Copy => {
                clipboard_resource.copy(text_input_value.clone());
            }
            Key::Paste => {
                clipboard_resource.paste(text_input_value);
            }
            _ => {}
        };

        // Finally, update the is_empty flag for the text input
        text_input_data.is_empty = text_input_value.is_empty();
    }
}
