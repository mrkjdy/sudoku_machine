use crate::plugins::screens::ScreenState;
use bevy::{
    app::AppExit,
    input::{keyboard::Key, keyboard::KeyboardInput, ButtonState},
    input_focus::InputFocus,
    prelude::*,
};

use super::common::{
    bundles::puzzle_cell::{puzzle_cell_input_system, PuzzleCellFocusCleared},
    theme::{
        node::{ThemedBackgroundColor, ThemedBorderColor, ThemedBorderRadius, ThemedBorderRect},
        text::{ThemedFontWeight, ThemedTextColor},
    },
};

pub fn nav_plugin(app: &mut App) {
    app.add_message::<PuzzleCellFocusCleared>()
        .add_systems(Startup, nav_setup)
        .add_systems(Update, (nav_visibility_system, nav_button_action).chain())
        .add_systems(Update, nav_escape_system.after(puzzle_cell_input_system));
}

#[derive(Component)]
#[require(
    Button,
    ThemedBackgroundColor,
    ThemedBorderColor,
    ThemedBorderRadius,
    ThemedBorderRect,
    Visibility
)]
pub struct NavButton;

#[derive(Component)]
struct MenuNavButton;

fn nav_setup(mut commands: Commands) {
    commands.spawn((
        MenuNavButton,
        Visibility::Hidden,
        nav_button_bundle(
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(20.0),
                width: Val::Px(80.0),
                height: Val::Px(60.0),
                ..default()
            },
            "🡠",
        ),
    ));
}

pub(super) fn nav_button_bundle(node: Node, icon: &str) -> impl Bundle {
    // TODO - use SVGs instead of text
    let nav_button_icon_bundle = (
        Text::new(icon),
        TextFont::from_font_size(50.0),
        ThemedFontWeight::Symbolic,
        ThemedTextColor,
        TextLayout::new_with_justify(Justify::Center),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            margin: UiRect::top(Val::Px(4.0)),
            ..default()
        },
    );

    (NavButton, node, children![nav_button_icon_bundle])
}

type NavButtonInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static Visibility),
    (Changed<Interaction>, With<NavButton>),
>;

fn nav_button_action(
    interaction_query: NavButtonInteractionQuery,
    mut next_screen: ResMut<NextState<ScreenState>>,
) {
    if interaction_query.iter().any(|(interaction, visibility)| {
        *interaction == Interaction::Pressed && *visibility != Visibility::Hidden
    }) {
        next_screen.set(ScreenState::Home);
    }
}

fn nav_visibility_system(
    mut nav_button_query: Query<&mut Visibility, With<MenuNavButton>>,
    screen_state: Res<State<ScreenState>>,
) {
    let visibility = match screen_state.get() {
        ScreenState::NewPuzzle | ScreenState::History => Visibility::Visible,
        ScreenState::Game | ScreenState::Home => Visibility::Hidden,
    };
    nav_button_query
        .single_mut()
        .unwrap()
        .set_if_neq(visibility);
}

fn nav_escape_system(
    mut keyboard_input_events: MessageReader<KeyboardInput>,
    input_focus: Option<Res<InputFocus>>,
    mut focus_cleared_events: MessageReader<PuzzleCellFocusCleared>,
    current_screen: Option<Res<State<ScreenState>>>,
    mut next_screen: Option<ResMut<NextState<ScreenState>>>,
    mut commands: Commands,
) {
    let focus_cleared = !focus_cleared_events.is_empty();
    focus_cleared_events.clear();

    let mut escape_pressed = false;
    for event in keyboard_input_events.read() {
        escape_pressed |= event.state == ButtonState::Pressed
            && !event.repeat
            && matches!(event.logical_key, Key::Escape);
    }

    if !escape_pressed || focus_cleared {
        return;
    }

    if input_focus.as_deref().and_then(InputFocus::get).is_some() {
        return;
    }

    if let Some(screen_state) = current_screen.as_ref() {
        match screen_state.get() {
            ScreenState::Home => {
                commands.write_message(AppExit::Success);
            }
            _ => {
                if let Some(ref mut next) = next_screen {
                    next.set(ScreenState::Home);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::state::app::StatesPlugin;

    #[cfg_attr(not(target_family = "wasm"), test)]
    #[cfg_attr(target_family = "wasm", wasm_bindgen_test::wasm_bindgen_test)]
    fn navigation_buttons_follow_the_current_screen() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<ScreenState>()
            .add_message::<KeyboardInput>()
            .add_plugins(nav_plugin);
        app.update();

        let world = app.world_mut();
        let menu_button = world
            .query_filtered::<Entity, With<MenuNavButton>>()
            .single(world)
            .unwrap();
        for (screen, expected_visibility) in [
            (ScreenState::Home, Visibility::Hidden),
            (ScreenState::Game, Visibility::Hidden),
            (ScreenState::NewPuzzle, Visibility::Visible),
            (ScreenState::History, Visibility::Visible),
        ] {
            app.world_mut()
                .resource_mut::<NextState<ScreenState>>()
                .set(screen);
            app.update();
            let world = app.world_mut();
            assert_eq!(
                world.get::<Visibility>(menu_button),
                Some(&expected_visibility)
            );
            *world.get_mut::<Interaction>(menu_button).unwrap() = Interaction::Pressed;
            app.update();
            let world = app.world_mut();
            let returns_home = matches!(
                world.resource::<NextState<ScreenState>>(),
                NextState::Pending(ScreenState::Home)
            );
            assert_eq!(returns_home, expected_visibility == Visibility::Visible);
            *world.get_mut::<Interaction>(menu_button).unwrap() = Interaction::None;
        }
    }
}
