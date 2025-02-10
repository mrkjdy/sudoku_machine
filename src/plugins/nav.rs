use bevy::prelude::*;
use strum_macros::Display;

use crate::{plugins::menu::MenuState, AppState};

use super::common::theme::Themed;

pub fn nav_plugin(app: &mut App) {
    app.init_state::<NavState>()
        .add_systems(Startup, nav_setup)
        .add_systems(Update, nav_icon_system.run_if(state_changed::<NavState>))
        .add_systems(Update, nav_button_action);
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States, Display)]
pub enum NavState {
    #[default]
    #[strum(to_string = "X")]
    Exit,
    #[strum(to_string = "<")]
    Back,
    #[strum(to_string = "||")]
    Pause,
}

impl From<NavState> for String {
    fn from(value: NavState) -> Self {
        value.to_string()
    }
}

#[derive(Component)]
#[require(Themed, Button)]
struct NavButton;

#[derive(Component)]
#[require(Themed, Text)]
struct NavButtonIcon;

fn nav_setup(mut commands: Commands) {
    let nav_button_bundle = (
        NavButton,
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(5.0)),
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            ..default()
        },
    );

    // TODO - use SVGs instead of text
    let nav_button_icon_bundle = (
        NavButtonIcon,
        Text::new(NavState::default()),
        TextFont::from_font_size(60.0),
    );

    commands
        .spawn(nav_button_bundle)
        .with_child(nav_button_icon_bundle);
}

fn nav_icon_system(
    mut nav_button_icon_query: Query<&mut Text, With<NavButtonIcon>>,
    nav_state: Res<State<NavState>>,
) {
    let mut text = nav_button_icon_query.single_mut();
    text.0 = nav_state.get().to_string();
}

fn nav_button_action(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<NavButton>)>,
    nav_state: Res<State<NavState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for _ in interaction_query
        .iter()
        .filter(|interaction| **interaction == Interaction::Pressed)
    {
        match *nav_state.get() {
            NavState::Back => {
                menu_state.set(MenuState::Home);
            }
            NavState::Exit => {
                app_exit_events.send(AppExit::Success);
            }
            NavState::Pause => {
                app_state.set(AppState::Menu);
            }
        }
    }
}
