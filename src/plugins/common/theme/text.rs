use bevy::prelude::*;

use crate::plugins::common::bundles::text_input::TextInputCursor;

use super::Theme;

#[derive(Component, Default, Clone, Copy)]
#[require(TextFont)]
pub enum ThemedFontWeight {
    #[default]
    Regular,
    Bold,
    Symbolic,
}

#[derive(Component, Default, Clone, Copy)]
pub struct ThemedTextColor;

/// Sets and changes the text color and font using the theme
pub fn themed_text_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            themed_text_change_system.run_if(resource_changed::<Theme>),
            themed_text_color_added_system,
            themed_font_weight_system,
        ),
    );
}

/// Sets and changes the text color and font using the theme
fn themed_text_change_system(
    theme: Res<Theme>,
    mut themed_text_color_query: Query<&mut TextColor, With<ThemedTextColor>>,
    mut text_cursor_query: Query<&mut BackgroundColor, With<TextInputCursor>>,
    mut themed_font_weight_query: Query<(&mut TextFont, &ThemedFontWeight)>,
) {
    for mut text_color in themed_text_color_query.iter_mut() {
        text_color.0 = theme.text_color;
    }
    for mut text_input_cursor_background_color in text_cursor_query.iter_mut() {
        text_input_cursor_background_color.0 = theme.text_color;
    }
    for (mut text_font, font_weight) in themed_font_weight_query.iter_mut() {
        text_font.font = match font_weight {
            ThemedFontWeight::Regular => theme.text_font_regular.clone(),
            ThemedFontWeight::Bold => theme.text_font_bold.clone(),
            ThemedFontWeight::Symbolic => theme.text_font_symbols.clone(),
        };
    }
}

fn themed_text_color_added_system(
    theme: Res<Theme>,
    mut themed_text_color_query: Query<&mut TextColor, Added<ThemedTextColor>>,
    mut text_cursor_query: Query<&mut BackgroundColor, Added<TextInputCursor>>,
) {
    for mut text_color in themed_text_color_query.iter_mut() {
        text_color.0 = theme.text_color;
    }
    for mut text_cursor_background_color in text_cursor_query.iter_mut() {
        text_cursor_background_color.0 = theme.text_color;
    }
}

fn themed_font_weight_system(
    theme: Res<Theme>,
    mut themed_font_weight_query: Query<
        (&mut TextFont, &ThemedFontWeight),
        Added<ThemedFontWeight>,
    >,
) {
    for (mut text_font, font_weight) in themed_font_weight_query.iter_mut() {
        text_font.font = match font_weight {
            ThemedFontWeight::Regular => theme.text_font_regular.clone(),
            ThemedFontWeight::Bold => theme.text_font_bold.clone(),
            ThemedFontWeight::Symbolic => theme.text_font_symbols.clone(),
        };
    }
}
