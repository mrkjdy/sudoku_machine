use bevy::{
    app::AppExit,
    input::{keyboard::Key, keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use strum_macros::Display;

use crate::plugins::screens::ScreenState;

use super::common::theme::{
    node::{ThemedBackgroundColor, ThemedBorderColor, ThemedBorderRadius, ThemedBorderRect},
    text::{ThemedFontWeight, ThemedTextColor},
};

use crate::plugins::common::theme::focus::FocusedEntity;

#[derive(Resource, Default)]
pub struct EscapeNavState {
    pub focus_cleared_this_frame: bool,
}

pub fn nav_plugin(app: &mut App) {
    app.init_state::<NavState>()
        .init_resource::<EscapeNavState>()
        .add_systems(Startup, nav_setup)
        .add_systems(Update, nav_visibility_system)
        .add_systems(Update, nav_button_action)
        .add_systems(Update, nav_escape_system)
        .add_systems(PostUpdate, reset_escape_nav_state);
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States, Display)]
pub enum NavState {
    #[default]
    #[strum(to_string = " ")]
    Hidden,
    #[strum(to_string = "🡠")]
    Back,
    #[strum(to_string = "⏸")]
    Pause,
}

impl From<NavState> for String {
    fn from(value: NavState) -> Self {
        value.to_string()
    }
}

#[derive(Component)]
#[require(
    Button,
    ThemedBackgroundColor,
    ThemedBorderColor,
    ThemedBorderRadius,
    ThemedBorderRect,
    Visibility
)]
pub struct NavButton;

#[derive(Component)]
struct MenuNavButton;

fn nav_setup(mut commands: Commands) {
    commands.spawn((
        MenuNavButton,
        Visibility::Hidden,
        nav_button_bundle(
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(20.0),
                width: Val::Px(80.0),
                height: Val::Px(60.0),
                ..default()
            },
            NavState::Back,
        ),
    ));
}

pub(super) fn nav_button_bundle(node: Node, icon: NavState) -> impl Bundle {
    // TODO - use SVGs instead of text
    let nav_button_icon_bundle = (
        Text::new(icon),
        TextFont::from_font_size(50.0),
        ThemedFontWeight::Symbolic,
        ThemedTextColor,
        TextLayout::new_with_justify(Justify::Center),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            margin: UiRect::top(Val::Px(4.0)),
            ..default()
        },
    );

    (NavButton, node, children![nav_button_icon_bundle])
}

fn nav_button_action(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<NavButton>)>,
    nav_state: Res<State<NavState>>,
    mut screen_state: ResMut<NextState<ScreenState>>,
) {
    for _ in interaction_query
        .iter()
        .filter(|interaction| **interaction == Interaction::Pressed)
    {
        match *nav_state.get() {
            NavState::Back => {
                screen_state.set(ScreenState::Home);
            }
            NavState::Hidden => {}
            NavState::Pause => {
                screen_state.set(ScreenState::Home);
            }
        }
    }
}

fn nav_visibility_system(
    mut nav_button_query: Query<&mut Visibility, With<MenuNavButton>>,
    nav_state: Res<State<NavState>>,
    screen_state: Res<State<ScreenState>>,
) {
    let mut visibility = nav_button_query.single_mut().unwrap();
    *visibility = if *nav_state.get() == NavState::Back && *screen_state.get() != ScreenState::Game
    {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

fn nav_escape_system(
    mut keyboard_input_events: MessageReader<KeyboardInput>,
    focused: Option<Res<FocusedEntity>>,
    escape_state: Res<EscapeNavState>,
    current_screen: Option<Res<State<ScreenState>>>,
    mut next_screen: Option<ResMut<NextState<ScreenState>>>,
    mut commands: Commands,
) {
    let escape_pressed = keyboard_input_events.read().any(|event| {
        event.state == ButtonState::Pressed && matches!(event.logical_key, Key::Escape)
    });

    if !escape_pressed || escape_state.focus_cleared_this_frame {
        return;
    }

    if focused.as_ref().and_then(|f| f.current).is_some() {
        return;
    }

    if let Some(screen_state) = current_screen.as_ref() {
        match screen_state.get() {
            ScreenState::Home => {
                commands.write_message(AppExit::Success);
            }
            _ => {
                if let Some(ref mut next) = next_screen {
                    next.set(ScreenState::Home);
                }
            }
        }
    }
}

fn reset_escape_nav_state(mut escape_state: ResMut<EscapeNavState>) {
    escape_state.focus_cleared_this_frame = false;
}
