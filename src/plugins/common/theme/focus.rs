use bevy::prelude::*;

use super::{node::ListItemButton, Theme};

#[derive(Resource, Default)]
pub struct FocusedEntity {
    pub last: Option<Entity>,
    pub current: Option<Entity>,
}

pub fn focus_plugin(app: &mut App) {
    app.init_resource::<FocusedEntity>().add_systems(
        Update,
        (
            focus_system,
            unfocus_system,
            focus_outline_system.run_if(resource_exists_and_changed::<FocusedEntity>),
        ),
    );
}

fn focus_system(
    interaction_query: Query<(&Interaction, Entity), Changed<Interaction>>,
    mut focused_entity: ResMut<FocusedEntity>,
) {
    for (&interaction, interacted_entity) in interaction_query.iter() {
        if interaction == Interaction::Pressed {
            focused_entity.last = focused_entity.current;
            focused_entity.current = Some(interacted_entity);
        }
    }
}

fn unfocus_system(
    buttons: Res<ButtonInput<MouseButton>>,
    interaction_query: Query<&Interaction, Changed<Interaction>>,
    mut focused_entity: ResMut<FocusedEntity>,
) {
    if buttons.get_just_pressed().len() > 0
        && interaction_query.iter().all(|&i| i != Interaction::Pressed)
    {
        focused_entity.last = focused_entity.current;
        focused_entity.current = None;
    }
}

fn focus_outline_system(
    theme: Res<Theme>,
    focused_entity: Res<FocusedEntity>,
    mut border_query: Query<&mut BorderColor, Without<ListItemButton>>,
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
