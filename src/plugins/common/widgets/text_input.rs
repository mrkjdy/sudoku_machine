use arboard::Clipboard;
use bevy::input::keyboard::Key;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::{ecs::system::EntityCommands, input::keyboard::KeyboardInput};
use derive_builder::Builder;

use crate::plugins::common::{
    focus::FocusedEntity,
    theme::{
        node::ThemedNodeBundleBuilder, text::ThemedTextBundleBuilder, ThemeComponent,
        UseThemeTextColorForBackground,
    },
};

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
pub struct TextInputData {
    pub placeholder_text: String,
    pub is_empty: bool,
}

impl Default for TextInputData {
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
    text_input_data: TextInputData,
    font_size: f32,
    container_style: Style,
    text_style: Style,
    justify_text: JustifyText,
    // no_cursor: bool,
    background_color: ThemeComponent<BackgroundColor>,
}

impl TextInputWidgetBuilder {
    pub fn build(&self) -> TextInputWidget {
        let TextInputWidgetBuilder {
            text_input_data,
            font_size,
            container_style,
            text_style,
            justify_text,
            background_color,
        } = self;
        TextInputWidget {
            text_input_data: text_input_data.clone().unwrap_or_default(),
            font_size: font_size.unwrap_or_default(),
            container_style: container_style.clone().unwrap_or_default(),
            text_style: text_style.clone().unwrap_or_default(),
            justify_text: justify_text.clone().unwrap_or_default(),
            background_color: background_color.clone().unwrap_or_default(),
        }
    }
}

#[derive(Component)]
struct TextInputContainer;

#[derive(Component)]
struct TextInputText;

#[derive(Component)]
struct TextInputCursor;

impl Spawnable for TextInputWidget {
    fn spawn_with_components<'a, S: super::Spawn>(
        &self,
        spawner: &'a mut S,
        components: impl Bundle,
    ) -> EntityCommands<'a> {
        let TextInputWidget {
            text_input_data,
            font_size,
            container_style,
            text_style,
            justify_text,
            // no_cursor,
            background_color,
        } = self;

        let container = ThemedNodeBundleBuilder::default()
            .style(Style {
                overflow: Overflow {
                    x: OverflowAxis::Hidden,
                    y: OverflowAxis::Hidden,
                },
                ..container_style.clone()
            })
            .background_color(background_color.clone())
            .build();

        let text = ThemedTextBundleBuilder::default()
            .value(text_input_data.placeholder_text.clone())
            .style(text_style.clone())
            .font_size(*font_size)
            .justify_text(*justify_text)
            .line_break_behavior(bevy::text::BreakLineOn::NoWrap)
            .build();

        let text_cursor = NodeBundle {
            style: Style {
                width: Val::Px(1.0),
                height: Val::Px(*font_size),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        };

        let mut ec = spawner.spawn((
            container,
            text_input_data.clone(),
            Interaction::default(),
            TextInputContainer,
        ));
        ec.insert(components);
        ec.with_children(|parent| {
            parent.spawn((text, TextInputText));
            parent.spawn((text_cursor, TextInputCursor, UseThemeTextColorForBackground));
        });
        return ec;
    }
}

fn text_input_focus_system(
    focused_entity: Res<FocusedEntity>,
    container_query: Query<(&TextInputData, &Children), With<TextInputContainer>>,
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
                text.sections[0].value = text_input_data.placeholder_text.clone();
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
                text.sections[0].value = "".into();
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
    mut container_query: Query<(&mut TextInputData, &Children), With<TextInputContainer>>,
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
    let text_input_value = &mut text.sections[0].value;

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
