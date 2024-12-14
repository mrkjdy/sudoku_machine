use bevy::prelude::*;

use crate::despawn_component;

use super::{GameState, PuzzleType};

pub fn knight_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Playing(PuzzleType::Knight)),
        knight_setup,
    )
    .add_systems(
        OnExit(GameState::Playing(PuzzleType::Knight)),
        despawn_component::<KnightContainer>,
    )
    .add_systems(
        Update,
        (knight_action_system).run_if(in_state(GameState::Playing(PuzzleType::Knight))),
    );
}

#[derive(Component)]
struct KnightContainer;

// Generate and spawn the board
fn knight_setup() {}

fn knight_action_system() {}
