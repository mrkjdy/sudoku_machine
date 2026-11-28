use bevy::prelude::*;

use crate::plugins::nav::NavState;

use super::ScreenState;

pub fn history_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(ScreenState::History), history_menu_setup);
}

fn history_menu_setup(mut nav_state: ResMut<NextState<NavState>>) {
    nav_state.set(NavState::Back);
}
