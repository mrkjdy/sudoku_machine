use bevy::prelude::*;

use crate::despawn_component;

use super::{GameState, PuzzleType};

pub fn full_kropki_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Playing(PuzzleType::FullKropki)),
        full_kropki_setup,
    )
    .add_systems(
        OnExit(GameState::Playing(PuzzleType::FullKropki)),
        despawn_component::<FullKropkiContainer>,
    )
    .add_systems(
        Update,
        (full_kropki_action_system).run_if(in_state(GameState::Playing(PuzzleType::FullKropki))),
    );
}

#[derive(Component)]
struct FullKropkiContainer;

// Generate and spawn the board
fn full_kropki_setup() {}

fn full_kropki_action_system() {}
