use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    plugins::common::{
        bundles::{
            dropdown::{self, dropdown_bundle, DropdownBundleOptions, DropdownContainer},
            text_input::{
                text_input_bundle, text_input_plugin, TextInputBundleOptions, TextInputContainer,
            },
        },
        theme::{
            node::{
                ThemedBackgroundColor, ThemedBorderColor, ThemedBorderRadius, ThemedBorderRect,
            },
            text::{ThemedFontWeight, ThemedTextColor},
        },
    },
    puzzles::PuzzleType,
    utility::seed::SeedRng,
};

use super::{PuzzleSettings, ScreenState, PIXELS_PER_CH};

pub fn new_puzzle_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(ScreenState::NewPuzzle), new_puzzle_menu_setup)
        .add_plugins((dropdown::dropdown_plugin, text_input_plugin))
        .add_systems(
            Update,
            (description_system, start_button_system).run_if(in_state(ScreenState::NewPuzzle)),
        );
}

#[derive(Component)]
struct PuzzleTypeDropdown;

#[derive(Component)]
#[require(Text, ThemedFontWeight::Regular, ThemedTextColor)]
struct PuzzleTypeDescriptionText;

#[derive(Component)]
struct SeedTextInput;

#[derive(Component)]
#[require(
    Button,
    ThemedBackgroundColor,
    ThemedBorderColor,
    ThemedBorderRadius,
    ThemedBorderRect
)]
struct StartButton;

fn new_puzzle_menu_setup(mut commands: Commands) {
    // Common node values
    let width = Val::Percent(96.0);
    let max_width = Val::Px(65.0 * PIXELS_PER_CH);
    let margin = UiRect::bottom(Val::Px(20.0));
    let body_font_size = 20.0;

    let title_bundle = (
        Text::new("New Puzzle"),
        TextFont::from_font_size(36.0),
        Node {
            max_width,
            margin: UiRect::all(Val::Px(40.0)),
            ..default()
        },
        ThemedFontWeight::Bold,
    );

    let base_heading_bundle = (
        TextFont::from_font_size(body_font_size),
        Node {
            width,
            max_width,
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
        ThemedFontWeight::Bold,
        ThemedTextColor,
    );

    let initial_selected_type = PuzzleType::default();

    let type_input_heading_bundle = (Text::new("Type"), base_heading_bundle.clone());

    let type_dropdown_bundle = (
        PuzzleTypeDropdown,
        dropdown_bundle(DropdownBundleOptions {
            selected: initial_selected_type as usize,
            options: PuzzleType::iter().map(|pt| pt.title().into()).collect(),
            text_font: TextFont::from_font_size(body_font_size),
            container_node: Node {
                width,
                max_width,
                margin,
                ..default()
            },
            button_node: Node {
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            ..Default::default()
        }),
    );

    let description_heading_bundle = (Text::new("Description"), base_heading_bundle.clone());

    let description_bundle = (
        PuzzleTypeDescriptionText,
        Text::new(initial_selected_type.description()),
        TextFont::from_font_size(body_font_size),
        Node {
            width,
            max_width,
            height: Val::Vh(20.0),
            ..default()
        },
    );

    let seed_input_heading_bundle = (Text::new("Seed"), base_heading_bundle);

    let seed_text_input_bundle = (
        SeedTextInput,
        text_input_bundle(TextInputBundleOptions {
            placeholder_text: "Random...".into(),
            text_font: TextFont::from_font_size(body_font_size),
            container_node: Node {
                margin: UiRect::bottom(Val::Px(40.0)),
                padding: UiRect::horizontal(Val::Px(5.0)),
                width,
                max_width,
                ..default()
            },
            ..Default::default()
        }),
    );

    let start_button_bundle = (
        StartButton,
        Node {
            width,
            max_width,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        children![(
            Text::new("Start"),
            TextFont::from_font_size(30.0),
            ThemedFontWeight::Bold,
            ThemedTextColor
        )],
    );

    commands.spawn((
        DespawnOnExit(ScreenState::NewPuzzle),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            padding: UiRect::top(Val::Px(80.0)),
            ..default()
        },
        children![
            title_bundle,
            type_input_heading_bundle,
            type_dropdown_bundle,
            description_heading_bundle,
            description_bundle,
            seed_input_heading_bundle,
            seed_text_input_bundle,
            start_button_bundle
        ],
    ));
}

fn description_system(
    dropdown_query: Query<
        &DropdownContainer,
        (Changed<DropdownContainer>, With<PuzzleTypeDropdown>),
    >,
    mut description_text_query: Query<&mut Text, With<PuzzleTypeDescriptionText>>,
) {
    for dropdown in dropdown_query.iter() {
        let mut description_text = description_text_query.single_mut().unwrap();
        description_text.0 = PuzzleType::try_from(dropdown.selected)
            .unwrap()
            .description()
            .into();
    }
}

fn start_button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    dropdown_query: Query<&DropdownContainer, With<PuzzleTypeDropdown>>,
    seed_container_query: Query<(&Children, &TextInputContainer), With<SeedTextInput>>,
    seed_text_query: Query<&Text>,
    mut puzzle_settings: ResMut<PuzzleSettings>,
    mut next_screen_state: ResMut<NextState<ScreenState>>,
) {
    for _ in interaction_query
        .iter()
        .filter(|interaction| **interaction == Interaction::Pressed)
    {
        // Read the puzzle settings from the dropdown and the seed input
        let dropdown_data = dropdown_query.single().unwrap();
        let (seed_container_children, text_input_data) = seed_container_query.single().unwrap();
        let seed_text = seed_text_query.get(seed_container_children[0]).unwrap();
        // Set the PuzzleSettings resource
        puzzle_settings.puzzle_type = PuzzleType::try_from(dropdown_data.selected).unwrap();
        puzzle_settings.seed = if text_input_data.is_empty {
            // Generate a random seed string if one was not provided
            rand::rng().gen_seed()
        } else {
            // Otherwise use the provided value
            seed_text.0.clone()
        };
        // Change states
        next_screen_state.set(ScreenState::Game);
    }
}

#[cfg(all(test, target_family = "wasm"))]
mod tests {
    use super::*;
    use crate::puzzles::classic::puzzle::ClassicPuzzle;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn start_without_seed_generates_playable_puzzle() {
        let mut app = App::new();
        app.init_resource::<PuzzleSettings>()
            .init_resource::<NextState<ScreenState>>()
            .add_systems(Update, start_button_system);
        let world = app.world_mut();
        world.spawn((StartButton, Interaction::Pressed));
        world.spawn((
            PuzzleTypeDropdown,
            DropdownContainer {
                selected: PuzzleType::Classic as usize,
                options: vec![PuzzleType::Classic.title().into()],
            },
        ));
        let seed_input = world
            .spawn((
                SeedTextInput,
                TextInputContainer {
                    placeholder_text: "Random".into(),
                    is_empty: true,
                },
            ))
            .id();
        world.spawn((Text::new("Random"), ChildOf(seed_input)));

        app.update();

        let seed = &app.world().resource::<PuzzleSettings>().seed;
        assert_eq!(seed.len(), 16);
        assert!(seed.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert!(matches!(
            app.world().resource::<NextState<ScreenState>>(),
            NextState::Pending(ScreenState::Game)
        ));
        let puzzle = ClassicPuzzle::from_seed(seed);
        assert_eq!(
            ClassicPuzzle::count_solutions_bounded_iterative(puzzle, 2),
            1
        );
    }
}
