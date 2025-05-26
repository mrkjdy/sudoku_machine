use bevy::prelude::*;

use super::Theme;

#[derive(Component, Default)]
pub struct ThemedBackgroundColor;

#[derive(Component, Default)]
pub struct ThemedBorderColor;

#[derive(Component, Default)]
pub struct ThemedBorderRadius;

#[derive(Component, Default)]
pub struct ThemedBorderRect;

pub fn themed_node_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            theme_changed_system.run_if(resource_changed::<Theme>),
            themed_background_color_added_system,
            themed_border_color_added_system,
            themed_border_radius_added_system,
            themed_border_rect_added_system,
        ),
    );
}

fn theme_changed_system(
    theme: Res<Theme>,
    mut background_color_query: Query<&mut BackgroundColor, With<ThemedBackgroundColor>>,
    mut border_color_query: Query<&mut BorderColor, With<ThemedBorderColor>>,
    mut border_radius_query: Query<&mut BorderRadius, With<ThemedBorderRadius>>,
    mut node_query: Query<&mut Node, With<ThemedBorderRect>>,
) {
    for mut background_color in background_color_query.iter_mut() {
        *background_color = theme.button_normal_background;
    }

    for mut border_color in border_color_query.iter_mut() {
        *border_color = theme.border_color;
    }

    for mut border_radius in border_radius_query.iter_mut() {
        *border_radius = theme.border_radius;
    }

    for mut node in node_query.iter_mut() {
        node.border = theme.border_rect;
    }
}

fn themed_background_color_added_system(
    theme: Res<Theme>,
    mut background_color_query: Query<&mut BackgroundColor, Added<ThemedBackgroundColor>>,
) {
    for mut background_color in background_color_query.iter_mut() {
        *background_color = theme.button_normal_background;
    }
}

fn themed_border_color_added_system(
    theme: Res<Theme>,
    mut border_color_query: Query<&mut BorderColor, Added<ThemedBorderColor>>,
) {
    for mut border_color in border_color_query.iter_mut() {
        *border_color = theme.border_color;
    }
}

fn themed_border_radius_added_system(
    theme: Res<Theme>,
    mut border_radius_query: Query<&mut BorderRadius, Added<ThemedBorderRadius>>,
) {
    for mut border_radius in border_radius_query.iter_mut() {
        *border_radius = theme.border_radius;
    }
}

fn themed_border_rect_added_system(
    theme: Res<Theme>,
    mut node_query: Query<&mut Node, Added<ThemedBorderRect>>,
) {
    for mut node in node_query.iter_mut() {
        node.border = theme.border_rect;
    }
}
