use bevy::prelude::*;

use crate::{
    despawn_component, plugins::nav::NavState, puzzles::classic::ClassicPuzzle, PuzzleSettings,
};

use super::{GameState, PuzzleType};

pub fn classic_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Playing(PuzzleType::Classic)),
        classic_puzzle_setup,
    )
    .add_systems(
        OnExit(GameState::Playing(PuzzleType::Classic)),
        despawn_component::<ClassicContainer>,
    )
    .add_systems(
        Update,
        (classic_game_action_system).run_if(in_state(GameState::Playing(PuzzleType::Classic))),
    );
}

#[derive(Component)]
struct ClassicContainer;

// Generate and spawn the board
fn classic_puzzle_setup(
    mut nav_state: ResMut<NextState<NavState>>,
    puzzle_settings: Res<PuzzleSettings>,
) {
    nav_state.set(NavState::Pause);
    println!("Setting up classic puzzle!");
    println!("Seed is {:}", puzzle_settings.seed);
    let puzzle = ClassicPuzzle::from_seed(puzzle_settings.seed.clone());
    println!("Finished:");
    println!("{puzzle:}");
}

fn classic_game_action_system() {}
