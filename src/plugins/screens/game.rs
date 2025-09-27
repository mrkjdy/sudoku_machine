use bevy::prelude::*;

use crate::{
    plugins::{despawn_component, nav::NavState, puzzles::classic::classic_puzzle_bundle},
    puzzles::{classic::puzzle::ClassicPuzzle, PuzzleType},
};

use super::{PuzzleSettings, ScreenState};

#[derive(Component)]
pub struct GameContainer;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(ScreenState::Game), game_setup)
        .add_systems(
            OnExit(ScreenState::Game),
            despawn_component::<GameContainer>,
        );
}

fn game_setup(
    mut nav_state: ResMut<NextState<NavState>>,
    mut commands: Commands,
    puzzle_settings: Res<PuzzleSettings>,
) {
    nav_state.set(NavState::Pause);

    info!("Setting up {:}!", puzzle_settings.puzzle_type.title());
    info!("Seed is {:}", puzzle_settings.seed);

    let game_container_bundle = (
        GameContainer,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            padding: UiRect::top(Val::Px(80.0)),
            ..default()
        },
    );

    match puzzle_settings.puzzle_type {
        PuzzleType::Classic => {
            let puzzle = ClassicPuzzle::from_seed(&puzzle_settings.seed);
            let puzzle_bundle = classic_puzzle_bundle(puzzle.grid);
            commands.spawn((game_container_bundle, children![puzzle_bundle]));
        }
        PuzzleType::FullKropki => {
            commands.spawn(game_container_bundle);
        }
        PuzzleType::Knight => {
            commands.spawn(game_container_bundle);
        }
    }
}
