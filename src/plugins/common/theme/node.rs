use bevy::prelude::*;

use super::{focus::FocusOutline, Theme};

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
    mut border_color_query: Query<
        (&mut BorderColor, Option<&FocusOutline>),
        With<ThemedBorderColor>,
    >,
    mut border_radius_query: Query<&mut BorderRadius, With<ThemedBorderRadius>>,
    mut node_query: Query<&mut Node, With<ThemedBorderRect>>,
) {
    for mut background_color in &mut background_color_query {
        *background_color = theme.button_normal_background;
    }

    for (mut border_color, focus_outline) in &mut border_color_query {
        let default_color = focus_outline
            .map(|outline| outline.normal)
            .unwrap_or(theme.border_color.0);
        *border_color = BorderColor(default_color);
    }

    for mut border_radius in &mut border_radius_query {
        *border_radius = theme.border_radius;
    }

    for mut node in &mut node_query {
        node.border = theme.border_rect;
    }
}

fn themed_background_color_added_system(
    theme: Res<Theme>,
    mut background_color_query: Query<&mut BackgroundColor, Added<ThemedBackgroundColor>>,
) {
    for mut background_color in &mut background_color_query {
        *background_color = theme.button_normal_background;
    }
}

fn themed_border_color_added_system(
    theme: Res<Theme>,
    mut border_color_query: Query<
        (&mut BorderColor, Option<&FocusOutline>),
        Added<ThemedBorderColor>,
    >,
) {
    for (mut border_color, focus_outline) in &mut border_color_query {
        let default_color = focus_outline
            .map(|outline| outline.normal)
            .unwrap_or(theme.border_color.0);
        *border_color = BorderColor(default_color);
    }
}

fn themed_border_radius_added_system(
    theme: Res<Theme>,
    mut border_radius_query: Query<&mut BorderRadius, Added<ThemedBorderRadius>>,
) {
    for mut border_radius in &mut border_radius_query {
        *border_radius = theme.border_radius;
    }
}

fn themed_border_rect_added_system(
    theme: Res<Theme>,
    mut node_query: Query<&mut Node, Added<ThemedBorderRect>>,
) {
    for mut node in &mut node_query {
        node.border = theme.border_rect;
    }
}
