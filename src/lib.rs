use bevy::prelude::*;

pub mod plugins {
    pub mod common;
    pub mod fps;
    pub mod nav;
    pub mod screens;
}

pub mod puzzles;

pub mod utility {
    pub mod bitset;
    pub mod element_set;
    pub mod priority_queue;
    pub mod seed;
}

pub const APP_TITLE: &str = "Sudoku Machine";

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_component<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}
