use bevy::{
    prelude::*,
    window::{WindowTheme, WindowThemeChanged},
};
use text::{FontColor, FontWeight};

use super::focus::{focus_plugin, FocusedEntity};

pub mod button;
pub mod node;
pub mod text;

// TODO - add run conditions to as many systems as possible!

pub fn theme_plugin(app: &mut App) {
    app.init_resource::<Theme>()
        .add_plugins(focus_plugin)
        .add_systems(Startup, setup_theme)
        .add_systems(
            Update,
            (
                window_theme_system,
                themed_text_init_system,
                themed_container_init_system,
                theme_change_system.run_if(resource_changed::<Theme>),
                themed_button_interaction_system,
                focus_outline_system,
            ),
        );
}

#[derive(Resource)]
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

#[derive(Default, Component, Clone)]
pub(super) enum ThemeComponent<T> {
    #[default]
    Themed,
    Other(T),
}

impl<T> ThemeComponent<T> {
    pub(self) fn other_or(&self, default: T) -> T
    where
        T: Copy,
    {
        match self {
            Self::Themed => default,
            Self::Other(t) => *t,
        }
    }
}

impl<T> From<Option<T>> for ThemeComponent<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            None => ThemeComponent::Themed,
            Some(t) => ThemeComponent::Other(t),
        }
    }
}

fn setup_theme(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create a camera
    commands.spawn(Camera2dBundle::default());

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

fn window_theme_system(
    mut ev_window_theme_changed: EventReader<WindowThemeChanged>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for ev in ev_window_theme_changed.read() {
        let text_font_regular = asset_server.load("fonts/OpenSans-Regular.ttf");
        let text_font_bold = asset_server.load("fonts/OpenSans-Bold.ttf");

        let app_theme: Theme = match ev.theme {
            WindowTheme::Dark => Theme::dark(text_font_regular, text_font_bold),
            WindowTheme::Light => Theme::light(text_font_regular, text_font_bold),
        };

        // Update the app theme
        commands.insert_resource(app_theme);
    }
}

// TODO - Maybe a set of these components would be useful?

#[derive(Component)]
pub struct UseThemeTextColorForBackground;

fn themed_text_init_system(
    theme: Res<Theme>,
    mut themed_text_query: Query<
        (
            &mut Text,
            &ThemeComponent<FontWeight>,
            &ThemeComponent<FontColor>,
        ),
        (
            Added<ThemeComponent<FontWeight>>,
            Added<ThemeComponent<FontColor>>,
        ),
    >,
    mut background_color_query: Query<&mut BackgroundColor, Added<UseThemeTextColorForBackground>>,
) {
    for (mut text, tc_font_weight, tc_font_color) in themed_text_query.iter_mut() {
        let text_style = &mut text.sections[0].style;
        text_style.font = match tc_font_weight.other_or(FontWeight::Regular) {
            FontWeight::Bold => theme.text_font_bold.clone(),
            FontWeight::Regular => theme.text_font_regular.clone(),
        };
        text_style.color = tc_font_color.other_or(FontColor(theme.text_color)).0;
    }
    for mut background_color in background_color_query.iter_mut() {
        background_color.0 = theme.text_color;
    }
}

fn themed_container_init_system(
    theme: Res<Theme>,
    mut themed_button_query: Query<
        (
            &mut Style,
            &mut BorderColor,
            &mut BorderRadius,
            &mut BackgroundColor,
            &ThemeComponent<UiRect>,
            &ThemeComponent<BorderColor>,
            &ThemeComponent<BorderRadius>,
            &ThemeComponent<BackgroundColor>,
        ),
        (
            Added<ThemeComponent<UiRect>>,
            Added<ThemeComponent<BorderColor>>,
            Added<ThemeComponent<BorderRadius>>,
            Added<ThemeComponent<BackgroundColor>>,
        ),
    >,
) {
    for (
        mut style,
        mut border_color,
        mut border_radius,
        mut background_color,
        tc_border_rect,
        tc_border_color,
        tc_border_radius,
        tc_background_color,
    ) in themed_button_query.iter_mut()
    {
        style.border = tc_border_rect.other_or(theme.border_rect);
        *border_color = tc_border_color.other_or(theme.border_color);
        *border_radius = tc_border_radius.other_or(theme.border_radius);
        *background_color = tc_background_color.other_or(theme.button_normal_background);
    }
}

fn theme_change_system(
    theme: Res<Theme>,
    mut clear_color: ResMut<ClearColor>,
    mut themed_button_query: Query<(
        &mut Style,
        &mut BorderColor,
        &mut BorderRadius,
        &mut BackgroundColor,
        &ThemeComponent<UiRect>,
        &ThemeComponent<BorderColor>,
        &ThemeComponent<BorderRadius>,
        &ThemeComponent<BackgroundColor>,
    )>,
    mut themed_text_query: Query<(
        &mut Text,
        &ThemeComponent<FontWeight>,
        &ThemeComponent<FontColor>,
    )>,
    mut background_color_query: Query<
        &mut BackgroundColor,
        (
            With<UseThemeTextColorForBackground>,
            Without<ThemeComponent<BackgroundColor>>,
        ),
    >,
) {
    clear_color.0 = theme.clear_color;
    for (
        mut style,
        mut border_color,
        mut border_radius,
        mut background_color,
        tc_border_rect,
        tc_border_color,
        tc_border_radius,
        tc_background_color,
    ) in themed_button_query.iter_mut()
    {
        style.border = tc_border_rect.other_or(theme.border_rect);
        *border_color = tc_border_color.other_or(theme.border_color);
        *border_radius = tc_border_radius.other_or(theme.border_radius);
        *background_color = tc_background_color.other_or(theme.button_normal_background);
    }
    for (mut text, tc_font_weight, tc_font_color) in themed_text_query.iter_mut() {
        let text_style = &mut text.sections[0].style;
        text_style.font = match tc_font_weight.other_or(FontWeight::Regular) {
            FontWeight::Bold => theme.text_font_bold.clone(),
            FontWeight::Regular => theme.text_font_regular.clone(),
        };
        text_style.color = tc_font_color.other_or(FontColor(theme.text_color)).0;
    }
    for mut background_color in background_color_query.iter_mut() {
        background_color.0 = theme.text_color;
    }
}

fn themed_button_interaction_system(
    theme: Res<Theme>,
    mut themed_button_query: Query<
        (
            &mut BackgroundColor,
            &Interaction,
            &ThemeComponent<BackgroundColor>,
        ),
        (
            Changed<Interaction>,
            With<ThemeComponent<BackgroundColor>>,
            With<Button>,
        ),
    >,
) {
    for (mut background_color, interaction, tc_background_color) in themed_button_query.iter_mut() {
        *background_color = match *interaction {
            Interaction::None => tc_background_color.other_or(theme.button_normal_background),
            Interaction::Hovered => theme.button_hovered_background,
            Interaction::Pressed => theme.button_pressed_background,
        };
    }
}

fn focus_outline_system(
    theme: Res<Theme>,
    focused_entity: Res<FocusedEntity>,
    mut border_query: Query<&mut BorderColor>,
) {
    if let Some(last) = focused_entity.last {
        if let Ok(mut last_border) = border_query.get_mut(last) {
            *last_border = theme.border_color;
        }
    }
    if let Some(current) = focused_entity.current {
        if let Ok(mut current_border) = border_query.get_mut(current) {
            *current_border = BorderColor(theme.button_pressed_background.0);
        }
    }
}
