#[cfg(not(target_family = "wasm"))]
use bevy::window::WindowMode;
use bevy::{prelude::*, window::PresentMode};
#[cfg(debug_assertions)]
use sudoku_machine::plugins::fps;
use sudoku_machine::{
    plugins::{common::theme, nav, screens},
    APP_TITLE,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: APP_TITLE.into(),
                present_mode: PresentMode::AutoVsync,
                #[cfg(not(target_family = "wasm"))]
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
        .add_plugins((
            theme::theme_plugin,
            screens::screen_plugin,
            nav::nav_plugin,
            #[cfg(debug_assertions)]
            fps::fps_plugin,
        ))
        .run();
}
