use bevy::prelude::*;
use plugins::game::PuzzleType;

pub mod plugins {
    pub mod common;
    pub mod fps;
    pub mod game;
    pub mod menu;
    pub mod nav;
}

pub mod puzzles {
    pub mod classic;
}

pub mod grids {
    pub mod classic;
}

pub mod utility {
    pub mod bitset;
    pub mod element_set;
    pub mod priority_queue;
    pub mod seed;
}

pub const APP_TITLE: &str = "Sudoku Machine";

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum AppState {
    #[default]
    Menu,
    Game,
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_component<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

#[derive(Default, Resource)]
pub struct PuzzleSettings {
    pub puzzle_type: PuzzleType,
    pub seed: String,
}
