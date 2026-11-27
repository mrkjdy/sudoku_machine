use bevy::prelude::*;

use super::{node::ThemedBackgroundColor, Theme};
use crate::plugins::common::bundles::puzzle_cell::PuzzleCellKind;

pub fn themed_button_plugin(app: &mut App) {
    app.add_systems(Update, themed_button_interaction_system);
}

#[allow(clippy::type_complexity)]
fn themed_button_interaction_system(
    theme: Res<Theme>,
    mut themed_button_query: Query<
        (&mut BackgroundColor, &Interaction, Option<&PuzzleCellKind>),
        (
            Changed<Interaction>,
            (With<ThemedBackgroundColor>, With<Button>),
        ),
    >,
) {
    for (mut background_color, interaction, kind) in &mut themed_button_query {
        if kind.is_some_and(|k| matches!(k, PuzzleCellKind::Given)) {
            *background_color = theme.puzzle_given_background;
            continue;
        }
        *background_color = match *interaction {
            Interaction::None => theme.button_normal_background,
            Interaction::Hovered => theme.button_hovered_background,
            Interaction::Pressed => theme.button_pressed_background,
        };
    }
}
