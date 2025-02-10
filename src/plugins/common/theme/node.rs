use bevy::prelude::*;

use super::{Theme, Themed};

#[derive(Component, Default, Clone, Copy)]
#[require(Themed, Button)]
pub struct ListItemButton;

pub fn themed_node_plugin(app: &mut App) {
    app.add_systems(Update, themed_node_system);
}

fn themed_node_system(
    theme: Res<Theme>,
    mut background_color_query: Query<&mut BackgroundColor, (With<Themed>, Without<Text>)>,
    mut border_color_query: Query<
        &mut BorderColor,
        (With<Themed>, (Without<Text>, Without<ListItemButton>)),
    >,
    mut border_radius_query: Query<&mut BorderRadius, (With<Themed>, Without<Text>)>,
    mut node_query: Query<&mut Node, (With<Themed>, Without<Text>)>,
) {
    for mut background_color in background_color_query
        .iter_mut()
        .filter(|background_color| theme.is_changed() || background_color.is_added())
    {
        *background_color = theme.button_normal_background;
    }
    for mut border_color in border_color_query
        .iter_mut()
        .filter(|border_color| theme.is_changed() || border_color.is_added())
    {
        *border_color = theme.border_color;
    }
    for mut border_radius in border_radius_query
        .iter_mut()
        .filter(|border_radius| theme.is_changed() || border_radius.is_added())
    {
        *border_radius = theme.border_radius;
    }
    for mut node in node_query
        .iter_mut()
        .filter(|node| theme.is_changed() || node.is_added())
    {
        node.border = theme.border_rect;
    }
}
