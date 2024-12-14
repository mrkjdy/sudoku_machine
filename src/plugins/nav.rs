use bevy::prelude::*;
use strum_macros::Display;

use crate::{plugins::menu::MenuState, AppState};

use super::common::theme::{button::ThemedButtonBundleBuilder, text::ThemedTextBundleBuilder};

pub fn nav_plugin(app: &mut App) {
    app.init_state::<NavState>()
        .add_systems(Startup, nav_setup)
        .add_systems(Update, nav_icon_system)
        .add_systems(Update, nav_button_action);
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States, Display)]
pub enum NavState {
    #[default]
    #[strum(to_string = "X")]
    Exit,
    #[strum(to_string = "<")]
    Back,
    #[strum(to_string = "||")]
    Pause,
}

impl From<NavState> for String {
    fn from(value: NavState) -> Self {
        value.to_string()
    }
}

#[derive(Component)]
struct NavButton;

#[derive(Component)]
struct NavButtonIcon;

fn nav_setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
) {
    // Setup the nav component
    let nav_button = ThemedButtonBundleBuilder::default()
        .style(Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(5.0)),
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            ..default()
        })
        .build();

    // Add an initial icon to the button
    // TODO - Use an SVG. Consult bevy_ui to figure out how to make a styled UI svg bundle.
    let nav_button_icon = ThemedTextBundleBuilder::default()
        .value(NavState::default().into())
        .font_size(60.0)
        .build();

    commands
        .spawn((nav_button, NavButton))
        .with_children(|parent| {
            parent.spawn((nav_button_icon, NavButtonIcon));
        });
}

fn nav_icon_system(
    mut nav_button_icon_query: Query<&mut Text, With<NavButtonIcon>>,
    nav_state: Res<State<NavState>>,
    // asset_server: Res<AssetServer>,
) {
    if nav_state.is_changed() {
        let mut text = nav_button_icon_query.single_mut();
        // *svg_handle = match *nav_state.get() {
        //     NavState::Back => asset_server.load("icons/arrow-uturn-left.svg"),
        //     NavState::Exit => asset_server.load("icons/x-mark.svg"),
        //     NavState::Pause => asset_server.load("icons/pause.svg"),
        // };
        text.sections[0].value = nav_state.get().to_string();
    }
}

fn nav_button_action(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<NavButton>)>,
    nav_state: Res<State<NavState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            match *nav_state.get() {
                NavState::Back => {
                    menu_state.set(MenuState::Home);
                }
                NavState::Exit => {
                    app_exit_events.send(AppExit::Success);
                }
                NavState::Pause => {
                    app_state.set(AppState::Menu);
                }
            }
        }
    }
}
