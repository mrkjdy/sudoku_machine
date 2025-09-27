use bevy::{ecs::spawn::SpawnIter, prelude::*};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{
    plugins::{
        common::theme::{
            node::{
                ThemedBackgroundColor, ThemedBorderColor, ThemedBorderRadius, ThemedBorderRect,
            },
            text::{ThemedFontWeight, ThemedTextColor},
        },
        despawn_component,
        nav::NavState,
    },
    APP_TITLE,
};

use super::{ScreenState, PIXELS_PER_CH};

pub fn home_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(ScreenState::Home), home_menu_setup)
        .add_systems(
            Update,
            (home_menu_action_system).run_if(in_state(ScreenState::Home)),
        )
        .add_systems(
            OnExit(ScreenState::Home),
            despawn_component::<HomeMenuContainer>,
        );
}

#[derive(Component)]
#[require(Node)]
struct HomeMenuContainer;

#[derive(Component, EnumIter, Display)]
#[require(
    Button,
    ThemedBackgroundColor,
    ThemedBorderColor,
    ThemedBorderRadius,
    ThemedBorderRect
)]
enum HomeMenuButton {
    Continue,
    #[strum(to_string = "New Puzzle")]
    NewPuzzle,
    History,
}

fn home_menu_setup(mut nav_state: ResMut<NextState<NavState>>, mut commands: Commands) {
    nav_state.set(NavState::Hidden);

    let title_bundle = (
        Text::new(APP_TITLE),
        TextFont::from_font_size(80.0),
        Node {
            margin: UiRect::all(Val::Px(50.0)),
            ..default()
        },
        ThemedFontWeight::Bold,
        ThemedTextColor,
    );

    let button_bundles = HomeMenuButton::iter().map(|home_menu_button| {
        let button_text_bundle = (
            ThemedFontWeight::Regular,
            ThemedTextColor,
            Text::new(home_menu_button.to_string()),
            TextFont::from_font_size(40.0),
        );

        (
            home_menu_button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::vertical(Val::Px(5.0)),
                width: Val::Px(32.0 * PIXELS_PER_CH),
                ..default()
            },
            children![button_text_bundle],
        )
    });

    commands.spawn((
        HomeMenuContainer,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        Children::spawn((Spawn(title_bundle), SpawnIter(button_bundles))),
    ));
}

fn home_menu_action_system(
    interaction_query: Query<(&Interaction, &HomeMenuButton), Changed<Interaction>>,
    mut screen_state: ResMut<NextState<ScreenState>>,
) {
    for (_, menu_button) in interaction_query
        .iter()
        .filter(|(interaction, _)| **interaction == Interaction::Pressed)
    {
        match menu_button {
            HomeMenuButton::Continue => {
                screen_state.set(ScreenState::Game);
            }
            HomeMenuButton::History => {
                screen_state.set(ScreenState::History);
            }
            HomeMenuButton::NewPuzzle => {
                screen_state.set(ScreenState::NewPuzzle);
            }
        }
    }
}
