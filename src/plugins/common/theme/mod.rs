use bevy::{
    prelude::*,
    window::{WindowTheme, WindowThemeChanged},
};
use button::themed_button_plugin;
use node::themed_node_plugin;
use text::themed_text_plugin;

use focus::focus_plugin;

pub mod button;
pub mod focus;
pub mod node;
pub mod text;

#[derive(Resource, Clone)]
struct Theme {
    clear_color: Color,
    text_font_regular: Handle<Font>,
    text_font_bold: Handle<Font>,
    text_color: Color,
    border_rect: UiRect,
    border_color: BorderColor,
    border_radius: BorderRadius,
    button_normal_background: BackgroundColor, // (not hovered or pressed)
    button_hovered_background: BackgroundColor,
    button_pressed_background: BackgroundColor,
}

impl Theme {
    fn dark(text_font_regular: Handle<Font>, text_font_bold: Handle<Font>) -> Self {
        Self {
            clear_color: Color::srgb_u8(13, 17, 23), // #0D1117
            text_font_regular,
            text_font_bold,
            text_color: Color::srgb(1.0, 1.0, 1.0),
            border_rect: UiRect::all(Val::Px(2.0)),
            border_color: BorderColor(Color::srgb_u8(48, 54, 61)), // #30363D
            border_radius: BorderRadius::all(Val::Px(6.0)),
            button_normal_background: BackgroundColor(Color::srgb_u8(21, 26, 35)), // #151A23
            button_hovered_background: BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            button_pressed_background: BackgroundColor(Color::srgb(0.35, 0.35, 0.85)),
        }
    }

    fn light(text_font_regular: Handle<Font>, text_font_bold: Handle<Font>) -> Self {
        Self {
            clear_color: Color::srgb(1.0, 1.0, 1.0),
            text_font_regular,
            text_font_bold,
            text_color: Color::srgb(0.0, 0.0, 0.0),
            border_rect: UiRect::all(Val::Px(2.0)),
            border_color: BorderColor(Color::srgb(0.1, 0.1, 0.1)),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            button_normal_background: BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
            button_hovered_background: BackgroundColor(Color::srgb(0.75, 0.75, 0.75)),
            button_pressed_background: BackgroundColor(Color::srgb(0.35, 0.35, 0.85)),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light(default(), default())
    }
}

#[derive(Component, Default)]
pub struct Themed;

pub fn theme_plugin(app: &mut App) {
    app.init_resource::<Theme>()
        .add_systems(Startup, theme_init_system)
        .add_systems(Update, (theme_change_system, clear_color_system))
        .add_plugins((
            themed_text_plugin,
            themed_node_plugin,
            themed_button_plugin,
            focus_plugin,
        ));
}

fn theme_init_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create a camera
    commands.spawn(Camera2d);

    let text_font_regular = asset_server.load("fonts/OpenSans-Regular.ttf");
    let text_font_bold = asset_server.load("fonts/OpenSans-Bold.ttf");

    // Use system theme to set initial app theme
    let app_theme: Theme = match dark_light::detect() {
        dark_light::Mode::Dark => Theme::dark(text_font_regular, text_font_bold),
        dark_light::Mode::Default => Theme::light(text_font_regular, text_font_bold),
        dark_light::Mode::Light => Theme::light(text_font_regular, text_font_bold),
    };

    // Set the theme as a resource for use across the app
    commands.insert_resource(app_theme);
}

fn theme_change_system(
    mut ev_window_theme_changed: EventReader<WindowThemeChanged>,
    current_theme: Res<Theme>,
    mut commands: Commands,
) {
    for ev in ev_window_theme_changed.read() {
        let text_font_regular = current_theme.text_font_regular.clone();
        let text_font_bold = current_theme.text_font_bold.clone();

        let app_theme: Theme = match ev.theme {
            WindowTheme::Dark => Theme::dark(text_font_regular, text_font_bold),
            WindowTheme::Light => Theme::light(text_font_regular, text_font_bold),
        };

        // Update the app theme
        commands.insert_resource(app_theme);
    }
}

fn clear_color_system(mut clear_color: ResMut<ClearColor>, theme: Res<Theme>) {
    clear_color.0 = theme.clear_color;
}
