use bevy::{input_focus::InputFocus, prelude::*, ui::UiSystems};

#[derive(Component, Clone, Copy)]
pub struct FocusOutline {
    pub normal: Color,
    pub focused: Option<Color>,
}

impl FocusOutline {
    #[must_use]
    pub const fn new(normal: Color, focused: Option<Color>) -> Self {
        Self { normal, focused }
    }

    #[must_use]
    pub const fn transparent() -> Self {
        Self {
            normal: Color::NONE,
            focused: None,
        }
    }
}

impl Default for FocusOutline {
    fn default() -> Self {
        Self::transparent()
    }
}

pub fn focus_plugin(app: &mut App) {
    app.init_resource::<InputFocus>()
        .add_observer(clear_despawned_focus)
        .add_systems(PreUpdate, pointer_focus_system.after(UiSystems::Focus));
}

fn pointer_focus_system(
    buttons: Res<ButtonInput<MouseButton>>,
    interaction_query: Query<(&Interaction, Entity), Changed<Interaction>>,
    mut input_focus: ResMut<InputFocus>,
) {
    let pressed = interaction_query
        .iter()
        .filter(|(interaction, _)| **interaction == Interaction::Pressed)
        .last();
    if let Some((_, entity)) = pressed {
        if input_focus.get() != Some(entity) {
            input_focus.set(entity);
        }
    } else if buttons.get_just_pressed().len() > 0 && input_focus.get().is_some() {
        input_focus.clear();
    }
}

fn clear_despawned_focus(event: On<Despawn, Interaction>, mut input_focus: ResMut<InputFocus>) {
    if input_focus.get() == Some(event.entity) {
        input_focus.clear();
    }
}
