use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{
    despawn_component,
    plugins::{
        common::theme::{
            button::ThemedButtonBundleBuilder,
            text::{FontWeight, ThemedTextBundleBuilder},
        },
        nav::NavState,
    },
    AppState, APP_TITLE,
};

use super::MenuState;

pub fn home_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(MenuState::Home), home_menu_setup)
        .add_systems(
            Update,
            (home_menu_action_system).run_if(in_state(MenuState::Home)),
        )
        .add_systems(OnExit(MenuState::Home), despawn_component::<HomeMenu>);
}

#[derive(Component)]
struct HomeMenu;

#[derive(Component, EnumIter, Display)]
enum HomeMenuButton {
    Continue,
    #[strum(to_string = "New Puzzle")]
    NewPuzzle,
    History,
}

fn home_menu_setup(mut nav_state: ResMut<NextState<NavState>>, mut commands: Commands) {
    nav_state.set(NavState::Exit);

    let home_menu_container = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        ..default()
    };

    let game_title = ThemedTextBundleBuilder::default()
        .value(APP_TITLE.into())
        .font_size(80.0)
        .font_weight(FontWeight::Bold)
        .style(Style {
            margin: UiRect::all(Val::Px(50.0)),
            ..default()
        })
        .build();

    let button_style = Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::vertical(Val::Px(5.0)),
        width: Val::Px(240.0),
        ..default()
    };

    let home_menu_button = ThemedButtonBundleBuilder::default()
        .style(button_style)
        .build();

    commands
        .spawn((home_menu_container, HomeMenu))
        .with_children(|parent| {
            parent.spawn(game_title);
            for button in HomeMenuButton::iter() {
                let button_text = button.to_string();
                parent
                    .spawn((home_menu_button.clone(), button))
                    .with_children(|parent| {
                        let button_text_bundle = ThemedTextBundleBuilder::default()
                            .value(button_text)
                            .font_size(40.0)
                            .build();
                        parent.spawn(button_text_bundle);
                    });
            }
        });
}

fn home_menu_action_system(
    interaction_query: Query<(&Interaction, &HomeMenuButton), Changed<Interaction>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
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
}
