use bevy::prelude::*;

use super::{Theme, Themed};

pub fn themed_button_plugin(app: &mut App) {
    app.add_systems(Update, themed_button_interaction_system);
}

#[allow(clippy::type_complexity)]
fn themed_button_interaction_system(
    theme: Res<Theme>,
    mut themed_button_query: Query<
        (&mut BackgroundColor, &Interaction),
        (Changed<Interaction>, (With<Themed>, With<Button>)),
    >,
) {
    for (mut background_color, interaction) in themed_button_query.iter_mut() {
        *background_color = match *interaction {
            Interaction::None => theme.button_normal_background,
            Interaction::Hovered => theme.button_hovered_background,
            Interaction::Pressed => theme.button_pressed_background,
        };
    }
}
