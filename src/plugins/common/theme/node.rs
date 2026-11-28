use bevy::{input_focus::InputFocus, prelude::*};

use super::{focus::FocusOutline, Theme};
use crate::plugins::common::bundles::puzzle_cell::PuzzleCellKind;

#[derive(Component, Default)]
pub struct ThemedBackgroundColor;

#[derive(Component, Default)]
pub struct ThemedBorderColor;

#[derive(Component, Default)]
pub struct ThemedBorderRadius;

#[derive(Component, Default)]
pub struct ThemedBorderRect;

type ThemedBorderQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Node,
        Has<ThemedBorderRadius>,
        Has<ThemedBorderRect>,
    ),
    Or<(With<ThemedBorderRadius>, With<ThemedBorderRect>)>,
>;

pub fn themed_node_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            theme_changed_system.run_if(resource_changed::<Theme>),
            themed_background_color_added_system,
            themed_border_radius_added_system,
            themed_border_rect_added_system,
        ),
    )
    .add_systems(PostUpdate, themed_border_color_system);
}

fn theme_changed_system(
    theme: Res<Theme>,
    mut background_color_query: Query<
        (&mut BackgroundColor, Option<&PuzzleCellKind>),
        With<ThemedBackgroundColor>,
    >,
    mut node_query: ThemedBorderQuery,
) {
    for (mut background_color, kind) in &mut background_color_query {
        let is_given = kind.is_some_and(|k| matches!(k, PuzzleCellKind::Given));
        *background_color = if is_given {
            theme.puzzle_given_background
        } else {
            theme.button_normal_background
        };
    }

    for (mut node, themed_radius, themed_rect) in &mut node_query {
        if themed_radius {
            node.border_radius = theme.border_radius;
        }
        if themed_rect {
            node.border = theme.border_rect;
        }
    }
}

fn themed_background_color_added_system(
    theme: Res<Theme>,
    mut background_color_query: Query<
        (&mut BackgroundColor, Option<&PuzzleCellKind>),
        Added<ThemedBackgroundColor>,
    >,
) {
    for (mut background_color, kind) in &mut background_color_query {
        let is_given = kind.is_some_and(|k| matches!(k, PuzzleCellKind::Given));
        *background_color = if is_given {
            theme.puzzle_given_background
        } else {
            theme.button_normal_background
        };
    }
}

fn themed_border_color_system(
    theme: Res<Theme>,
    input_focus: Res<InputFocus>,
    mut border_color_query: Query<(
        Entity,
        &mut BorderColor,
        Option<&FocusOutline>,
        Ref<ThemedBorderColor>,
    )>,
) {
    let refresh_all = theme.is_changed() || input_focus.is_changed();
    for (entity, mut border_color, focus_outline, themed_border) in &mut border_color_query {
        if refresh_all || themed_border.is_added() {
            let color = if input_focus.get() == Some(entity) {
                focus_outline
                    .and_then(|outline| outline.focused)
                    .unwrap_or(theme.button_pressed_background.0)
            } else {
                focus_outline.map_or(theme.border_color, |outline| outline.normal)
            };
            *border_color = BorderColor::all(color);
        }
    }
}

fn themed_border_radius_added_system(
    theme: Res<Theme>,
    mut node_query: Query<&mut Node, Added<ThemedBorderRadius>>,
) {
    for mut node in &mut node_query {
        node.border_radius = theme.border_radius;
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
