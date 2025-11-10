use bevy::prelude::*;

use crate::puzzles::PuzzleType;

mod game;
mod history;
mod home;
mod new_puzzle;

#[derive(Default, Resource)]
pub struct PuzzleSettings {
    pub puzzle_type: PuzzleType,
    pub seed: String,
}

pub fn screen_plugin(app: &mut App) {
    app.init_state::<ScreenState>()
        .init_resource::<PuzzleSettings>()
        .add_plugins((
            home::home_menu_plugin,
            new_puzzle::new_puzzle_menu_plugin,
            history::history_menu_plugin,
            game::game_plugin,
        ));
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum ScreenState {
    #[default]
    Home,
    NewPuzzle,
    History,
    Game,
}

// Measured the width of the character "0" on my mac when it was 16px tall.
pub const PIXELS_PER_CH: f32 = 10.5;
