use bevy::prelude::*;
use indoc::indoc;
use num_enum::TryFromPrimitive;
use strum_macros::{Display, EnumIter};

use crate::{AppState, PuzzleSettings};

mod classic;
#[cfg(debug_assertions)]
mod full_kropki;
#[cfg(debug_assertions)]
mod knight;

#[derive(Default, EnumIter, Display, TryFromPrimitive, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum PuzzleType {
    #[default]
    Classic,
    #[cfg(debug_assertions)]
    Knight,
    #[cfg(debug_assertions)]
    #[strum(to_string = "Full Kropki")]
    FullKropki,
}

impl PuzzleType {
    #[must_use]
    pub fn description(&self) -> String {
        match self {
            PuzzleType::Classic => indoc! {"
                Fill a 9x9 grid so each row, column, and 3x3 box contains all digits 1-9 without \
                repetition.
            "},
            #[cfg(debug_assertions)]
            PuzzleType::Knight => indoc! {"
                Classic rules plus no identical numbers can be a knight’s move apart, like in chess.
            "},
            #[cfg(debug_assertions)]
            PuzzleType::FullKropki => indoc! {"
                Classic rules plus Kropki dot rules for adjacent cells:
                 • a black dot means one of the numbers is twice the value of the other
               	 • a white dot means the numbers are consecutive
            "},
        }
        .into()
    }
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, States)]
pub enum GameState {
    Playing(PuzzleType),
    #[default]
    Disabled,
}

pub fn game_plugin(app: &mut App) {
    app.init_state::<GameState>()
        .add_systems(OnEnter(AppState::Game), game_setup)
        .add_plugins((
            classic::classic_plugin,
            #[cfg(debug_assertions)]
            full_kropki::full_kropki_plugin,
            #[cfg(debug_assertions)]
            knight::knight_plugin,
        ));
}

fn game_setup(
    mut next_game_state: ResMut<NextState<GameState>>,
    puzzle_settings: ResMut<PuzzleSettings>,
) {
    // Transition the game to the corresponding puzzle
    next_game_state.set(GameState::Playing(puzzle_settings.puzzle_type));
}
