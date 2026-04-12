use std::time::Duration;

use bevy::{ecs::schedule::common_conditions::resource_exists, prelude::*};

use crate::{
    plugins::{
        common::{
            bundles::puzzle_cell::{
                puzzle_cell_input_system, PuzzleCell, PuzzleCellBoardSize, PuzzleCellEditEvent,
                PuzzleCellFocusCleared, PuzzleCellKind, PuzzleCellNeighborHighlight,
                PuzzleCellPosition, PUZZLE_CELL_NEIGHBOR_HIGHLIGHT_COLOR,
            },
            clipboard::ClipboardResource,
            theme::{
                focus::FocusedEntity,
                node::{
                    ThemedBackgroundColor, ThemedBorderColor, ThemedBorderRadius, ThemedBorderRect,
                },
                text::{ThemedFontWeight, ThemedTextColor},
                Theme,
            },
        },
        despawn_component,
        nav::{EscapeNavState, NavButton, NavState},
        puzzles::classic::{classic_puzzle_bundle, ClassicGridState},
    },
    puzzles::{
        classic::{
            grid::{ClassicGrid, NUM_COLS, NUM_ROWS},
            puzzle::ClassicPuzzle,
        },
        PuzzleMeta, PuzzleType,
    },
};

use super::{PuzzleSettings, ScreenState};

#[derive(Component)]
pub struct GameContainer;

#[derive(Component)]
struct GamePuzzlePanel;

#[derive(Component)]
struct GameTimerText;

#[derive(Component)]
struct SeedButton;

#[derive(Component)]
struct SeedCopyIcon;

#[derive(Component)]
struct SeedCopyFeedbackTimer(Timer);

#[derive(Component)]
struct NavButtonGameLayout;

#[derive(Component)]
struct GameHeader;

#[derive(Component)]
struct GameTitleSeedGroup;

#[derive(Component)]
struct GameTimerContainer;

#[derive(Resource, Default)]
struct GameTimer {
    elapsed: Duration,
    last_displayed_seconds: u64,
}

type SeedButtonInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static Children),
    (Changed<Interaction>, With<SeedButton>),
>;

type PuzzleCellHighlightQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static PuzzleCellPosition,
        &'static mut BackgroundColor,
        Option<&'static mut PuzzleCellNeighborHighlight>,
        Option<&'static PuzzleCellKind>,
        &'static Interaction,
    ),
    With<PuzzleCell>,
>;

const COPY_ICON: &str = "❐";
const CHECK_ICON: &str = "✔";

pub fn game_plugin(app: &mut App) {
    app.add_event::<PuzzleCellEditEvent>()
        .add_event::<PuzzleCellFocusCleared>()
        .add_systems(OnEnter(ScreenState::Game), game_setup)
        .add_systems(
            OnExit(ScreenState::Game),
            (
                restore_nav_button_layout,
                despawn_component::<GameContainer>,
                clear_classic_grid_state,
                clear_game_timer,
            ),
        )
        .add_systems(
            Update,
            (
                puzzle_cell_input_system
                    .run_if(in_state(ScreenState::Game))
                    .run_if(resource_exists::<ClassicGridState>)
                    .run_if(resource_exists::<PuzzleCellBoardSize>),
                classic_puzzle_cell_apply_edit_system
                    .run_if(in_state(ScreenState::Game))
                    .run_if(resource_exists::<ClassicGridState>),
                classic_puzzle_focus_clear_system.run_if(in_state(ScreenState::Game)),
                classic_puzzle_neighbor_highlight_system
                    .run_if(in_state(ScreenState::Game))
                    .run_if(resource_exists::<ClassicGridState>),
                game_header_layout_system.run_if(in_state(ScreenState::Game)),
                game_timer_system
                    .run_if(in_state(ScreenState::Game))
                    .run_if(resource_exists::<GameTimer>),
                seed_button_interaction_system
                    .run_if(in_state(ScreenState::Game))
                    .run_if(resource_exists::<ClipboardResource>),
                seed_copy_feedback_system.run_if(in_state(ScreenState::Game)),
            ),
        );
}

fn game_setup(
    mut nav_state: ResMut<NextState<NavState>>,
    mut commands: Commands,
    puzzle_settings: Res<PuzzleSettings>,
    mut nav_button_query: Query<(Entity, &mut Node), With<NavButton>>,
) {
    nav_state.set(NavState::Pause);

    commands.insert_resource(GameTimer::default());

    let nav_button_entity = nav_button_query
        .single_mut()
        .map(|mut nav| {
            nav.1.position_type = PositionType::Relative;
            nav.1.left = Val::Auto;
            nav.1.top = Val::Auto;
            nav.1.width = Val::Px(80.0);
            nav.1.height = Val::Px(60.0);
            nav.1.justify_content = JustifyContent::Center;
            nav.1.align_items = AlignItems::Center;
            nav.1.flex_shrink = 0.0;
            nav.1.margin = UiRect::all(Val::Px(0.0));
            commands.entity(nav.0).insert(NavButtonGameLayout);
            nav.0
        })
        .ok();

    let classic_grid = match puzzle_settings.puzzle_type {
        PuzzleType::Classic => {
            let puzzle = ClassicPuzzle::from_seed(&puzzle_settings.seed);
            let grid = puzzle.grid;
            commands.insert_resource(ClassicGridState::new(grid));
            commands.insert_resource(PuzzleCellBoardSize {
                rows: NUM_ROWS,
                cols: NUM_COLS,
            });
            Some(grid)
        }
        #[cfg(debug_assertions)]
        _ => None,
    };

    let container_entity = commands
        .spawn((
            GameContainer,
            Node {
                width: Val::Percent(96.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::vertical(Val::Px(24.0)),
                row_gap: Val::Px(24.0),
                ..default()
            },
        ))
        .id();

    let header_entity = commands
        .spawn((
            GameHeader,
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                flex_wrap: FlexWrap::Wrap,
                row_gap: Val::Px(12.0),
                column_gap: Val::Px(16.0),
                ..default()
            },
            ChildOf(container_entity),
        ))
        .id();

    if let Some(nav_button_entity) = nav_button_entity {
        commands
            .entity(nav_button_entity)
            .insert(ChildOf(header_entity));
    }

    commands.entity(header_entity).with_children(|header| {
        if classic_grid.is_some() {
            header
                .spawn((
                    GameTitleSeedGroup,
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(8.0),
                        margin: UiRect::horizontal(Val::Auto),
                        ..default()
                    },
                ))
                .with_children(|info| {
                    info.spawn((
                        Text::new(ClassicPuzzle::title()),
                        TextFont::from_font_size(30.0),
                        ThemedFontWeight::Bold,
                        ThemedTextColor,
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                    info.spawn((
                        SeedButton,
                        Button,
                        ThemedBackgroundColor,
                        ThemedBorderColor,
                        ThemedBorderRadius,
                        ThemedBorderRect,
                        Node {
                            padding: UiRect::horizontal(Val::Px(16.0)),
                            height: Val::Px(44.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            column_gap: Val::Px(12.0),
                            min_width: Val::Px(220.0),
                            ..default()
                        },
                        children![
                            (
                                ThemedTextColor,
                                ThemedFontWeight::Regular,
                                Text::from(puzzle_settings.seed.clone()),
                                TextFont::from_font_size(20.0),
                                TextLayout::new_with_justify(JustifyText::Left),
                            ),
                            (
                                SeedCopyIcon,
                                ThemedTextColor,
                                ThemedFontWeight::Symbolic,
                                Text::from(COPY_ICON),
                                TextFont::from_font_size(22.0),
                                TextLayout::new_with_justify(JustifyText::Right),
                            )
                        ],
                    ));
                });
        }

        header.spawn((
            GameTimerContainer,
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(4.0),
                flex_shrink: 0.0,
                ..default()
            },
            ThemedBackgroundColor,
            ThemedBorderColor,
            ThemedBorderRadius,
            ThemedBorderRect,
            children![(
                GameTimerText,
                ThemedTextColor,
                ThemedFontWeight::Bold,
                Text::from("0:00"),
                TextFont::from_font_size(28.0),
                TextLayout::new_with_justify(JustifyText::Right),
            )],
        ));
    });

    if let Some(grid) = classic_grid {
        let puzzle_panel = commands
            .spawn((
                GamePuzzlePanel,
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ChildOf(container_entity),
            ))
            .id();

        commands
            .entity(puzzle_panel)
            .with_children(|puzzle_parent| {
                puzzle_parent.spawn(classic_puzzle_bundle(grid));
            });
    }
}

fn clear_classic_grid_state(mut commands: Commands) {
    commands.remove_resource::<ClassicGridState>();
    commands.remove_resource::<PuzzleCellBoardSize>();
}

fn clear_game_timer(mut commands: Commands) {
    commands.remove_resource::<GameTimer>();
}

fn restore_nav_button_layout(
    mut commands: Commands,
    mut nav_query: Query<(Entity, &mut Node), With<NavButtonGameLayout>>,
) {
    if let Ok((entity, mut node)) = nav_query.single_mut() {
        node.position_type = PositionType::Absolute;
        node.left = Val::Px(20.0);
        node.top = Val::Px(20.0);
        node.width = Val::Px(80.0);
        node.height = Val::Px(60.0);
        node.justify_content = JustifyContent::FlexStart;
        node.align_items = AlignItems::FlexStart;
        node.flex_shrink = 1.0;
        node.margin = UiRect::all(Val::Px(0.0));
        commands.entity(entity).remove::<ChildOf>();
        commands.entity(entity).remove::<NavButtonGameLayout>();
    }
}

fn game_header_layout_system(
    mut commands: Commands,
    header_query: Query<(Entity, &ComputedNode, &Children), With<GameHeader>>,
    nav_query: Query<(Entity, &ComputedNode), With<NavButtonGameLayout>>,
    title_query: Query<(Entity, &ComputedNode), With<GameTitleSeedGroup>>,
    timer_query: Query<(Entity, &ComputedNode), With<GameTimerContainer>>,
) {
    let Ok((header_entity, header_node, header_children)) = header_query.single() else {
        return;
    };
    let Ok((nav_entity, nav_node)) = nav_query.single() else {
        return;
    };
    let Ok((title_entity, title_node)) = title_query.single() else {
        return;
    };
    let Ok((timer_entity, timer_node)) = timer_query.single() else {
        return;
    };

    let available_width = header_node.size().x;
    if available_width <= 0.0 {
        return;
    }

    let nav_width = nav_node.size().x;
    let title_width = title_node.size().x;
    let timer_width = timer_node.size().x;
    if nav_width <= 0.0 || title_width <= 0.0 || timer_width <= 0.0 {
        return;
    }

    const HEADER_COLUMN_GAP: f32 = 16.0;
    let total_required = nav_width + title_width + timer_width + (2.0 * HEADER_COLUMN_GAP);

    let desired_children: [Entity; 3] = if total_required <= available_width {
        [nav_entity, title_entity, timer_entity]
    } else {
        [nav_entity, timer_entity, title_entity]
    };

    let current_children: &[Entity] = header_children;
    let needs_reorder = if current_children.len() != desired_children.len() {
        true
    } else {
        current_children
            .iter()
            .zip(desired_children.iter())
            .any(|(current, desired)| current != desired)
    };

    if needs_reorder {
        commands
            .entity(header_entity)
            .replace_children(&desired_children);
    }
}

fn game_timer_system(
    time: Res<Time>,
    mut game_timer: ResMut<GameTimer>,
    mut timer_text_query: Query<&mut Text, With<GameTimerText>>,
) {
    let mut timer_text = timer_text_query
        .single_mut()
        .expect("exactly one GameTimerText should exist");

    game_timer.elapsed += time.delta();
    let total_seconds = game_timer.elapsed.as_secs();
    if total_seconds == game_timer.last_displayed_seconds {
        return;
    }

    game_timer.last_displayed_seconds = total_seconds;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    timer_text.0 = format!("{minutes:01}:{seconds:02}");
}

fn seed_button_interaction_system(
    mut interaction_query: SeedButtonInteractionQuery,
    mut icon_query: Query<(Entity, &mut Text), With<SeedCopyIcon>>,
    mut clipboard: ResMut<ClipboardResource>,
    puzzle_settings: Res<PuzzleSettings>,
    mut commands: Commands,
) {
    for (&interaction, children) in &mut interaction_query {
        if interaction != Interaction::Pressed {
            continue;
        }

        clipboard.copy(puzzle_settings.seed.clone());

        for child in children.iter() {
            if let Ok((icon_entity, mut icon_text)) = icon_query.get_mut(child) {
                icon_text.0 = CHECK_ICON.to_string();
                commands
                    .entity(icon_entity)
                    .insert(SeedCopyFeedbackTimer(Timer::from_seconds(
                        1.5,
                        TimerMode::Once,
                    )));
            }
        }
    }
}

fn seed_copy_feedback_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut SeedCopyFeedbackTimer, &mut Text), With<SeedCopyIcon>>,
) {
    for (entity, mut feedback_timer, mut text) in &mut query {
        if feedback_timer.0.tick(time.delta()).finished() {
            text.0 = COPY_ICON.to_string();
            commands.entity(entity).remove::<SeedCopyFeedbackTimer>();
        }
    }
}

fn classic_puzzle_cell_apply_edit_system(
    mut events: EventReader<PuzzleCellEditEvent>,
    mut grid_state: ResMut<ClassicGridState>,
) {
    for event in events.read() {
        grid_state.set(event.position.row, event.position.col, event.value);
    }
}

fn classic_puzzle_focus_clear_system(
    mut events: EventReader<PuzzleCellFocusCleared>,
    mut escape_nav_state: ResMut<EscapeNavState>,
) {
    if events.is_empty() {
        return;
    }
    escape_nav_state.focus_cleared_this_frame = true;
    events.clear();
}

fn classic_puzzle_neighbor_highlight_system(
    theme: Res<Theme>,
    focused_entity: Res<FocusedEntity>,
    mut commands: Commands,
    position_query: Query<&PuzzleCellPosition, With<PuzzleCell>>,
    mut cell_query: PuzzleCellHighlightQuery,
) {
    let Some(current) = focused_entity.current else {
        for (entity, _, mut background, highlight, kind, _) in &mut cell_query {
            if highlight.is_some() {
                *background = BackgroundColor(base_color_for_kind(&theme, kind));
                commands
                    .entity(entity)
                    .remove::<PuzzleCellNeighborHighlight>();
            }
        }
        return;
    };

    let focus_position = match position_query.get(current) {
        Ok(position) => position,
        Err(_) => {
            for (entity, _, mut background, highlight, kind, _) in &mut cell_query {
                if highlight.is_some() {
                    *background = BackgroundColor(base_color_for_kind(&theme, kind));
                    commands
                        .entity(entity)
                        .remove::<PuzzleCellNeighborHighlight>();
                }
            }
            return;
        }
    };

    let mut neighbor_mask = [[false; NUM_COLS]; NUM_ROWS];
    for (row, col) in ClassicGrid::neighbor_positions(focus_position.row, focus_position.col) {
        neighbor_mask[row][col] = true;
    }

    for (entity, position, mut background, highlight, kind, interaction) in &mut cell_query {
        let should_highlight = neighbor_mask[position.row][position.col];
        let base_color = base_color_for_kind(&theme, kind);
        if should_highlight {
            if let Some(mut highlight) = highlight {
                highlight.previous = base_color;
                if *interaction != Interaction::Hovered {
                    *background = BackgroundColor(PUZZLE_CELL_NEIGHBOR_HIGHLIGHT_COLOR);
                }
            } else {
                if *interaction != Interaction::Hovered {
                    *background = BackgroundColor(PUZZLE_CELL_NEIGHBOR_HIGHLIGHT_COLOR);
                }
                commands.entity(entity).insert(PuzzleCellNeighborHighlight {
                    previous: base_color,
                });
            }
        } else if highlight.is_some() {
            *background = BackgroundColor(base_color);
            commands
                .entity(entity)
                .remove::<PuzzleCellNeighborHighlight>();
        }
    }
}

fn base_color_for_kind(theme: &Theme, kind: Option<&PuzzleCellKind>) -> Color {
    if kind.is_some_and(|k| matches!(k, PuzzleCellKind::Given)) {
        theme.puzzle_given_background_color()
    } else {
        theme.button_normal_background_color()
    }
}
