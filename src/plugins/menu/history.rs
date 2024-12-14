use bevy::prelude::*;

use crate::{despawn_component, plugins::nav::NavState};

use super::MenuState;

pub fn history_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(MenuState::History), history_menu_setup)
        .add_systems(OnExit(MenuState::History), despawn_component::<HistoryMenu>);
}

#[derive(Component)]
struct HistoryMenu;

fn history_menu_setup(mut nav_state: ResMut<NextState<NavState>>, mut _commands: Commands) {
    nav_state.set(NavState::Back);
}
