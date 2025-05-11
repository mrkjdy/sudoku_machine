use bevy::prelude::*;

use crate::plugins::common::bundles::text_input::TextInputCursor;

use super::{Theme, Themed};

#[derive(Component, Default, Clone, Copy)]
#[require(Themed, TextFont)]
pub enum ThemedFontWeight {
    #[default]
    Regular,
    Bold,
}

/// Sets and changes the text color and font using the theme
pub fn themed_text_plugin(app: &mut App) {
    app.add_systems(Update, themed_text_system);
}

/// Sets and changes the text color and font using the theme
fn themed_text_system(
    theme: Res<Theme>,
    mut themed_text_color_query: Query<&mut TextColor, With<Themed>>,
    mut themed_font_weight_query: Query<(&mut TextFont, &ThemedFontWeight)>,
    mut text_cusor_query: Query<&mut BackgroundColor, With<TextInputCursor>>,
) {
    for mut text_color in themed_text_color_query
        .iter_mut()
        .filter(|text_color| theme.is_changed() || text_color.is_added())
    {
        text_color.0 = theme.text_color;
    }
    for (mut text_font, font_weight) in themed_font_weight_query
        .iter_mut()
        .filter(|(text_font, _)| theme.is_changed() || text_font.is_added())
    {
        text_font.font = match font_weight {
            ThemedFontWeight::Regular => theme.text_font_regular.clone(),
            ThemedFontWeight::Bold => theme.text_font_bold.clone(),
        };
    }
    for mut text_input_cursor_background_color in
        text_cusor_query
            .iter_mut()
            .filter(|text_input_cursor_background_color| {
                theme.is_changed() || text_input_cursor_background_color.is_added()
            })
    {
        text_input_cursor_background_color.0 = theme.text_color;
    }
}
