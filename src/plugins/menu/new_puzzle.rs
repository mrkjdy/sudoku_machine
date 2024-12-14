use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    despawn_component,
    plugins::{
        common::{
            theme::{
                button::ThemedButtonBundleBuilder,
                text::{FontWeight, ThemedTextBundleBuilder},
            },
            widgets::{
                dropdown::{self, DropdownData, DropdownWidgetBuilder},
                text_input::{text_input_plugin, TextInputData, TextInputWidgetBuilder},
                Spawnable,
            },
        },
        game::PuzzleType,
        nav::NavState,
    },
    utility::seed::SeedRng,
    AppState, PuzzleSettings,
};

use super::{MenuState, CH};

pub fn new_puzzle_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(MenuState::NewPuzzle), new_puzzle_menu_setup)
        .add_systems(OnExit(MenuState::NewPuzzle), despawn_component::<NewMenu>)
        .add_plugins((dropdown::dropdown_plugin, text_input_plugin))
        .add_systems(
            Update,
            (description_system, start_button_system).run_if(in_state(MenuState::NewPuzzle)),
        );
}

#[derive(Component)]
struct NewMenu;

#[derive(Component)]
struct PuzzleTypeDropdown;

#[derive(Component)]
struct PuzzleTypeDescriptionText;

#[derive(Component)]
struct SeedTextInput;

#[derive(Component)]
struct StartButton;

fn new_puzzle_menu_setup(mut nav_state: ResMut<NextState<NavState>>, mut commands: Commands) {
    nav_state.set(NavState::Back);

    // New puzzle menu container
    let new_menu_container = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            // row_gap: Val::Px(20.0),
            padding: UiRect::top(Val::Px(80.0)),
            ..default()
        },
        ..default()
    };

    // Common style fields
    let width = Val::Percent(96.0);
    let max_width = Val::Px(65.0 * CH);
    let margin = UiRect::bottom(Val::Px(20.0));

    // Title

    let new_puzzle_menu_title = ThemedTextBundleBuilder::default()
        .value("New Puzzle".into())
        .font_size(36.0)
        .font_weight(FontWeight::Bold)
        .style(Style {
            max_width,
            margin: UiRect::all(Val::Px(40.0)),
            ..default()
        })
        .build();

    let body_font_size = 20.0;

    let mut label_builder = ThemedTextBundleBuilder::default();
    label_builder
        .font_size(body_font_size)
        .font_weight(FontWeight::Bold)
        .justify_text(JustifyText::Left)
        .style(Style {
            width,
            max_width,
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        });

    // Type

    let type_label = label_builder.value("Type".into()).build();

    let initial_selected_type = PuzzleType::default();

    let type_dropdown_widget = DropdownWidgetBuilder::default()
        .dropdown(DropdownData {
            selected: initial_selected_type as usize,
            options: PuzzleType::iter().map(|o| o.to_string()).collect(),
        })
        .font_size(body_font_size)
        .container_style(Style {
            width,
            max_width,
            margin,
            ..default()
        })
        .button_style(Style {
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        })
        .build();

    // Description

    let description_label = label_builder.value("Description".into()).build();

    let description_text = ThemedTextBundleBuilder::default()
        .value(initial_selected_type.description())
        .font_size(body_font_size)
        .justify_text(JustifyText::Left)
        .style(Style {
            width,
            max_width,
            height: Val::Vh(20.0),
            ..default()
        })
        .build();

    // Seed

    let seed_label = label_builder.value("Seed".into()).build();

    let seed_text_input_widget = TextInputWidgetBuilder::default()
        .container_style(Style {
            margin: UiRect::bottom(Val::Px(40.0)),
            padding: UiRect::all(Val::Px(5.0)),
            width,
            max_width,
            ..default()
        })
        .font_size(body_font_size)
        .justify_text(JustifyText::Left)
        .text_input_data(TextInputData {
            placeholder_text: "Random...".into(),
            ..default()
        })
        .build();

    // Start

    let start_button = ThemedButtonBundleBuilder::default()
        .style(Style {
            width,
            max_width,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        })
        .build();

    let start_button_text = ThemedTextBundleBuilder::default()
        .value("Start".into())
        .font_size(30.0)
        .build();

    commands
        .spawn((new_menu_container, NewMenu))
        .with_children(|parent| {
            parent.spawn(new_puzzle_menu_title);
            parent.spawn(type_label);
            type_dropdown_widget.spawn_with_components(parent, PuzzleTypeDropdown);
            parent.spawn(description_label);
            parent.spawn((description_text, PuzzleTypeDescriptionText));
            parent.spawn(seed_label);
            seed_text_input_widget.spawn_with_components(parent, SeedTextInput);
            parent
                .spawn((start_button, StartButton))
                .with_children(|parent| {
                    parent.spawn(start_button_text);
                });
        });
}

fn description_system(
    dropdown_query: Query<&DropdownData, (Changed<DropdownData>, With<PuzzleTypeDropdown>)>,
    mut description_text_query: Query<&mut Text, With<PuzzleTypeDescriptionText>>,
) {
    for dropdown in dropdown_query.iter() {
        let mut description_text = description_text_query.get_single_mut().unwrap();
        description_text.sections[0].value = PuzzleType::try_from(dropdown.selected)
            .unwrap()
            .description();
    }
}

// TODO - Optimize when systems run! They should only run if the entities they work on are present

fn start_button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    dropdown_query: Query<&DropdownData, With<PuzzleTypeDropdown>>,
    seed_container_query: Query<(&Children, &TextInputData), With<SeedTextInput>>,
    seed_text_query: Query<&Text>,
    mut puzzle_settings: ResMut<PuzzleSettings>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for &interaction in interaction_query.iter() {
        if interaction == Interaction::Pressed {
            // Read the puzzle settings from the dropdown and the seed input
            let dropdown_data = dropdown_query.single();
            let (seed_container_children, text_input_data) = seed_container_query.single();
            let seed_text = seed_text_query.get(seed_container_children[0]).unwrap();
            // Set the PuzzleSettings resource
            puzzle_settings.puzzle_type = PuzzleType::try_from(dropdown_data.selected).unwrap();
            puzzle_settings.seed = if text_input_data.is_empty {
                // Generate a random seed string if one was not provided
                let mut rng = rand::thread_rng();
                rng.gen_seed()
            } else {
                // Otherwise use the provided value
                seed_text.sections[0].value.clone()
            };
            // Change states
            next_menu_state.set(MenuState::Disabled);
            next_app_state.set(AppState::Game);
        }
    }
}
