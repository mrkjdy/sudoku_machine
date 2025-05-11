use bevy::{ecs::spawn::SpawnIter, prelude::*};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{
    despawn_component,
    plugins::{
        common::theme::{text::ThemedFontWeight, Themed},
        nav::NavState,
    },
    AppState, APP_TITLE,
};

use super::{MenuState, PIXELS_PER_CH};

pub fn home_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(MenuState::Home), home_menu_setup)
        .add_systems(
            Update,
            (home_menu_action_system).run_if(in_state(MenuState::Home)),
        )
        .add_systems(
            OnExit(MenuState::Home),
            despawn_component::<HomeMenuContainer>,
        );
}

#[derive(Component)]
#[require(Node)]
struct HomeMenuContainer;

#[derive(Component, EnumIter, Display)]
#[require(Themed, Button)]
enum HomeMenuButton {
    Continue,
    #[strum(to_string = "New Puzzle")]
    NewPuzzle,
    History,
}

fn home_menu_setup(mut nav_state: ResMut<NextState<NavState>>, mut commands: Commands) {
    nav_state.set(NavState::Exit);

    let title_bundle = (
        Text::new(APP_TITLE),
        TextFont::from_font_size(80.0),
        Node {
            margin: UiRect::all(Val::Px(50.0)),
            ..default()
        },
        ThemedFontWeight::Bold,
    );

    let button_bundles = HomeMenuButton::iter().map(|home_menu_button| {
        let button_text_bundle = (
            Themed,
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
        Children::spawn(Spawn(title_bundle)),
        Children::spawn(SpawnIter(button_bundles)),
    ));
}

fn home_menu_action_system(
    interaction_query: Query<(&Interaction, &HomeMenuButton), Changed<Interaction>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (_, menu_button) in interaction_query
        .iter()
        .filter(|(interaction, _)| **interaction == Interaction::Pressed)
    {
        match menu_button {
            HomeMenuButton::Continue => {
                app_state.set(AppState::Game);
                menu_state.set(MenuState::Disabled);
            }
            HomeMenuButton::History => {
                menu_state.set(MenuState::History);
            }
            HomeMenuButton::NewPuzzle => {
                menu_state.set(MenuState::NewPuzzle);
            }
        }
    }
}
