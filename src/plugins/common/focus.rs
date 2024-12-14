use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct FocusedEntity {
    pub last: Option<Entity>,
    pub current: Option<Entity>,
}

pub fn focus_plugin(app: &mut App) {
    app.init_resource::<FocusedEntity>()
        .add_systems(Update, (focus_system, unfocus_system));
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
