use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};
use sudoku_machine::{
    plugins::{common::theme, game, menu, nav},
    AppState, PuzzleSettings, APP_TITLE,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: APP_TITLE.into(),
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                // Tells bevy to use the system theme
                window_theme: None,
                // Tells Wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .init_resource::<PuzzleSettings>()
        .add_plugins((
            theme::theme_plugin,
            menu::menu_plugin,
            nav::nav_plugin,
            game::game_plugin,
        ))
        .run();
}
