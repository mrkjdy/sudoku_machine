use bevy::prelude::*;
use strum_macros::Display;

use crate::{plugins::menu::MenuState, AppState};

use super::common::theme::{
    node::{ThemedBackgroundColor, ThemedBorderColor, ThemedBorderRadius, ThemedBorderRect},
    text::{ThemedFontWeight, ThemedTextColor},
};

pub fn nav_plugin(app: &mut App) {
    app.init_state::<NavState>()
        .add_systems(Startup, nav_setup)
        .add_systems(Update, nav_icon_system.run_if(state_changed::<NavState>))
        .add_systems(Update, nav_button_action);
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States, Display)]
pub enum NavState {
    #[default]
    #[strum(to_string = "🞨")]
    Exit,
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
    ThemedBorderRect
)]
struct NavButton;

#[derive(Component)]
#[require(Text, ThemedFontWeight::Regular, ThemedTextColor)]
struct NavButtonIcon;

fn nav_setup(mut commands: Commands) {
    // TODO - use SVGs instead of text
    let nav_button_icon_bundle = (
        NavButtonIcon,
        Text::new(NavState::default()),
        TextFont::from_font_size(50.0),
        ThemedFontWeight::Symbolic,
        ThemedTextColor,
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            margin: UiRect::top(Val::Px(4.0)),
            ..default()
        },
    );

    commands.spawn((
        NavButton,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            width: Val::Px(80.0),
            height: Val::Px(60.0),
            ..default()
        },
        children![nav_button_icon_bundle],
    ));
}

fn nav_icon_system(
    mut nav_button_icon_query: Query<&mut Text, With<NavButtonIcon>>,
    nav_state: Res<State<NavState>>,
) {
    let mut text = nav_button_icon_query.single_mut().unwrap();
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
                app_exit_events.write(AppExit::Success);
            }
            NavState::Pause => {
                app_state.set(AppState::Menu);
            }
        }
    }
}
