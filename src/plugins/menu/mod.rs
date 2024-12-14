use bevy::prelude::*;

use crate::AppState;

use super::game::GameState;

mod history;
mod home;
mod new_puzzle;

pub fn menu_plugin(app: &mut App) {
    app.init_state::<MenuState>()
        .add_systems(OnEnter(AppState::Menu), menu_setup)
        .add_plugins((
            home::home_menu_plugin,
            new_puzzle::new_puzzle_menu_plugin,
            history::history_menu_plugin,
        ));
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum MenuState {
    #[default]
    Home,
    NewPuzzle,
    History,
    Disabled,
}

// Measured the width of the character "0" on my mac when it was 16px tall.
pub const CH: f32 = 10.5;

fn menu_setup(
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    game_state.set(GameState::Disabled);
    menu_state.set(MenuState::Home);
}
