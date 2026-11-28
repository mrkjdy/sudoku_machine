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

#[cfg(all(test, target_family = "wasm"))]
mod tests {
    use super::*;
    use crate::plugins::{
        nav::{nav_plugin, NavButton},
        puzzles::classic::ClassicGridState,
    };
    use bevy::state::app::StatesPlugin;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn screen_transitions_clean_up_ui_and_keep_navigation() {
        let mut app = App::new();
        app.add_plugins((StatesPlugin, screen_plugin, nav_plugin));
        let world = app.world_mut();
        world.run_schedule(Startup);
        world.run_schedule(StateTransition);

        let nav = world
            .query_filtered::<Entity, With<NavButton>>()
            .single(world)
            .unwrap();
        let nav_icon = world.get::<Children>(nav).unwrap()[0];
        let mut game_nav = world.query_filtered::<Entity, (With<NavButton>, With<ChildOf>)>();
        let mut screen_roots =
            world.query_filtered::<Entity, (With<Node>, Without<ChildOf>, Without<NavButton>)>();

        for screen in [ScreenState::NewPuzzle, ScreenState::Game, ScreenState::Home] {
            let old_root = screen_roots.single(world).unwrap();
            let old_child = world.get::<Children>(old_root).unwrap()[0];
            world
                .resource_mut::<NextState<ScreenState>>()
                .set(screen.clone());
            world.run_schedule(StateTransition);

            assert_eq!(world.resource::<State<ScreenState>>().get(), &screen);
            assert!(world.get_entity(old_root).is_err());
            assert!(world.get_entity(old_child).is_err());
            assert!(world.get::<NavButton>(nav).is_some());
            assert_eq!(world.get::<ChildOf>(nav_icon).unwrap().parent(), nav);
            assert!(world.get::<ChildOf>(nav).is_none());
            if screen == ScreenState::Game {
                let button = game_nav.single(world).unwrap();
                let header = world.get::<ChildOf>(button).unwrap().parent();
                assert_eq!(
                    world.get::<ChildOf>(header).unwrap().parent(),
                    screen_roots.single(world).unwrap()
                );
                assert!(world.contains_resource::<ClassicGridState>());
            } else {
                assert!(game_nav.iter(world).next().is_none());
                assert!(!world.contains_resource::<ClassicGridState>());
            }
        }
        assert!(screen_roots.single(world).is_ok());
    }
}
