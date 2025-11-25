use bevy::{
    input::{keyboard::Key, keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use crate::plugins::common::theme::{
    focus::{FocusOutline, FocusedEntity},
    node::{ThemedBackgroundColor, ThemedBorderColor, ThemedBorderRadius, ThemedBorderRect},
    text::{ThemedFontWeight, ThemedTextColor},
};

#[derive(Default, Clone)]
pub struct PuzzleCellBundleOptions {
    pub label: String,
    pub font_size: f32,
    pub given: bool,
    pub cell_node: Node,
    pub text_node: Node,
}

#[derive(Component)]
#[require(
    Button,
    Node,
    Interaction,
    BorderColor,
    FocusOutline,
    ThemedBackgroundColor,
    ThemedBorderColor,
    ThemedBorderRadius,
    ThemedBorderRect
)]
pub struct PuzzleCell;

#[derive(Component)]
pub struct PuzzleCellNeighborHighlight {
    pub previous: Color,
}

pub const PUZZLE_CELL_NEIGHBOR_HIGHLIGHT_COLOR: Color = Color::srgba(0.45, 0.55, 0.95, 0.35);

#[derive(Component, Clone, Copy, PartialEq, Eq, Default)]
pub enum PuzzleCellKind {
    #[default]
    Editable,
    Given,
}

#[derive(Component)]
#[require(Text, ThemedTextColor)]
pub struct PuzzleCellValue;

#[derive(Component, Clone, Copy, Default)]
pub struct PuzzleCellPosition {
    pub row: usize,
    pub col: usize,
}

#[derive(Resource, Clone, Copy)]
pub struct PuzzleCellBoardSize {
    pub rows: usize,
    pub cols: usize,
}

#[derive(Event)]
pub struct PuzzleCellEditEvent {
    pub entity: Entity,
    pub position: PuzzleCellPosition,
    pub value: Option<u8>,
}

#[derive(Event, Default)]
pub struct PuzzleCellFocusCleared;

pub fn puzzle_cell_bundle(options: PuzzleCellBundleOptions) -> impl Bundle {
    let PuzzleCellBundleOptions {
        label,
        font_size,
        given,
        mut cell_node,
        mut text_node,
    } = options;

    if matches!(cell_node.width, Val::Auto) {
        cell_node.width = Val::Percent(100.0);
    }
    if matches!(cell_node.height, Val::Auto) {
        cell_node.height = Val::Percent(100.0);
    }
    cell_node.align_items = AlignItems::Center;
    cell_node.justify_content = JustifyContent::Center;

    text_node.justify_content = JustifyContent::Center;

    let weight = if given {
        ThemedFontWeight::Bold
    } else {
        ThemedFontWeight::Regular
    };

    let focus_outline = if given {
        FocusOutline::new(Color::NONE, Some(Color::srgb(0.6, 0.6, 0.6)))
    } else {
        FocusOutline::transparent()
    };

    (
        PuzzleCell,
        PuzzleCellKind::from(given),
        focus_outline,
        BorderColor(Color::NONE),
        cell_node,
        children![(
            PuzzleCellValue,
            Text::from(label),
            TextFont::from_font_size(font_size),
            weight,
            ThemedTextColor,
            text_node,
        )],
    )
}

impl From<bool> for PuzzleCellKind {
    fn from(value: bool) -> Self {
        if value {
            Self::Given
        } else {
            Self::Editable
        }
    }
}

enum PuzzleCellEdit {
    Set(u8),
    Clear,
}

fn key_to_cell_edit(key: &Key) -> Option<PuzzleCellEdit> {
    match key {
        Key::Character(value) => {
            let mut chars = value.chars();
            let ch = chars.next()?;
            if chars.next().is_some() {
                return None;
            }
            let digit = ch.to_digit(10)? as u8;
            if (1..=9).contains(&digit) {
                Some(PuzzleCellEdit::Set(digit))
            } else {
                None
            }
        }
        Key::Backspace | Key::Delete => Some(PuzzleCellEdit::Clear),
        _ => None,
    }
}

pub fn puzzle_cell_input_system(
    mut focused_entity: ResMut<FocusedEntity>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    cell_query: Query<(Entity, &PuzzleCellPosition, &PuzzleCellKind, &Children)>,
    mut value_query: Query<&mut Text, With<PuzzleCellValue>>,
    board_size: Res<PuzzleCellBoardSize>,
    mut edit_events: EventWriter<PuzzleCellEditEvent>,
    mut focus_clear_events: EventWriter<PuzzleCellFocusCleared>,
) {
    let Some(target_entity) = focused_entity.current.or(focused_entity.last) else {
        return;
    };

    let Ok((_, position_ref, kind_ref, children_ref)) = cell_query.get(target_entity) else {
        return;
    };
    let position = *position_ref;
    let kind = *kind_ref;
    let children: Vec<Entity> = children_ref.iter().collect::<Vec<Entity>>();

    let mut pending_edit: Option<PuzzleCellEdit> = None;
    let mut pending_move: Option<(isize, isize)> = None;
    let mut clear_focus = false;
    let has_current_focus = focused_entity.current.is_some();

    for keyboard_input_event in keyboard_input_events.read() {
        if keyboard_input_event.state != ButtonState::Pressed {
            continue;
        }
        match &keyboard_input_event.logical_key {
            Key::Escape => {
                if has_current_focus {
                    clear_focus = true;
                }
            }
            Key::ArrowUp => pending_move = Some((-1, 0)),
            Key::ArrowDown => pending_move = Some((1, 0)),
            Key::ArrowLeft => pending_move = Some((0, -1)),
            Key::ArrowRight => pending_move = Some((0, 1)),
            _ => {
                if kind == PuzzleCellKind::Editable {
                    if let Some(edit) = key_to_cell_edit(&keyboard_input_event.logical_key) {
                        pending_edit = Some(edit);
                    }
                }
            }
        }
    }

    if clear_focus {
        focus_clear_events.write_default();
        focused_entity.last = focused_entity.current;
        focused_entity.current = None;
        return;
    }

    if let Some((row_delta, col_delta)) = pending_move {
        let new_row = position.row as isize + row_delta;
        let new_col = position.col as isize + col_delta;
        if (0..board_size.rows as isize).contains(&new_row)
            && (0..board_size.cols as isize).contains(&new_col)
        {
            if let Some(new_focus) =
                find_cell_entity(&cell_query, new_row as usize, new_col as usize)
            {
                focused_entity.last = focused_entity.current.or(Some(target_entity));
                focused_entity.current = Some(new_focus);
            }
        }
        return;
    }

    let Some(edit) = pending_edit else {
        return;
    };

    let mut value_entity = None;
    for child in &children {
        if value_query.get(*child).is_ok() {
            value_entity = Some(*child);
            break;
        }
    }
    let Some(value_entity) = value_entity else {
        return;
    };

    if let Ok(mut text) = value_query.get_mut(value_entity) {
        match edit {
            PuzzleCellEdit::Set(value) => {
                text.0 = value.to_string();
                edit_events.write(PuzzleCellEditEvent {
                    entity: target_entity,
                    position,
                    value: Some(value),
                });
            }
            PuzzleCellEdit::Clear => {
                text.0.clear();
                edit_events.write(PuzzleCellEditEvent {
                    entity: target_entity,
                    position,
                    value: None,
                });
            }
        }
    }

    if focused_entity.current.is_none() {
        focused_entity.current = Some(target_entity);
    }
}

fn find_cell_entity(
    cell_query: &Query<(Entity, &PuzzleCellPosition, &PuzzleCellKind, &Children)>,
    target_row: usize,
    target_col: usize,
) -> Option<Entity> {
    cell_query.iter().find_map(|(entity, position, _, _)| {
        (position.row == target_row && position.col == target_col).then_some(entity)
    })
}
