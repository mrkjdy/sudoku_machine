use bevy::{
    ecs::spawn::SpawnIter,
    input::{keyboard::Key, keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use crate::{
    plugins::{
        common::theme::{
            focus::{FocusOutline, FocusedEntity},
            node::{
                ThemedBackgroundColor, ThemedBorderColor, ThemedBorderRadius, ThemedBorderRect,
            },
            text::{ThemedFontWeight, ThemedTextColor},
        },
        nav::EscapeNavState,
        screens::PIXELS_PER_CH,
    },
    puzzles::{
        classic::grid::{ClassicGrid, NUM_COLS, NUM_ROWS},
        CellValue, Row,
    },
};

const BLOCK_SIZE: usize = 3;
const THIN_LINE: f32 = 1.0;
const THICK_LINE: f32 = 3.0;

fn given_focus_border() -> Color {
    Color::srgb(0.6, 0.6, 0.6)
}

#[derive(Component, Clone, Copy)]
pub(crate) struct ClassicCell {
    row: usize,
    col: usize,
    given: bool,
}

#[derive(Component, Default)]
pub(crate) struct ClassicCellText;

#[derive(Resource, Clone, Copy)]
pub struct ClassicGridState {
    grid: ClassicGrid,
}

impl ClassicGridState {
    #[must_use]
    pub const fn new(grid: ClassicGrid) -> Self {
        Self { grid }
    }

    fn set(&mut self, row: usize, col: usize, value: CellValue) {
        self.grid.set((row as u8, col as u8), value);
    }

    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> CellValue {
        self.grid.get_by_row_col((row as u8, col as u8))
    }
}

struct CellBundleOptions {
    cell_value: CellValue,
    font_size: f32,
    row: usize,
    col: usize,
}

fn block_separator_width(index: usize, max_index: usize) -> f32 {
    if index >= max_index {
        0.0
    } else if (index + 1) % BLOCK_SIZE == 0 {
        THICK_LINE
    } else {
        THIN_LINE
    }
}

fn cell_border(row: usize, col: usize) -> UiRect {
    let right = block_separator_width(col, NUM_COLS - 1);
    let bottom = block_separator_width(row, NUM_ROWS - 1);

    UiRect {
        left: Val::Px(0.0),
        right: Val::Px(right),
        top: Val::Px(0.0),
        bottom: Val::Px(bottom),
    }
}

fn cell_bundle(options: CellBundleOptions) -> impl Bundle {
    let CellBundleOptions {
        cell_value,
        font_size,
        row,
        col,
    } = options;
    let given = cell_value.is_some();
    let cell_button_bundle = cell_button_bundle(cell_value, font_size, row, col, given);

    let cell_width = 100.0 / NUM_COLS as f32;

    (
        Node {
            width: Val::Percent(cell_width),
            height: Val::Percent(100.0),
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::Stretch,
            border: cell_border(row, col),
            ..default()
        },
        ThemedBorderColor,
        children![cell_button_bundle],
    )
}

fn cell_button_bundle(
    cell_value: CellValue,
    font_size: f32,
    row: usize,
    col: usize,
    given: bool,
) -> impl Bundle {
    let value_string = match cell_value {
        Some(value) => value.to_string(),
        None => String::new(),
    };

    let text_bundle = (
        ClassicCellText,
        if given {
            ThemedFontWeight::Bold
        } else {
            ThemedFontWeight::Regular
        },
        ThemedTextColor,
        Text::from(value_string),
        TextFont::from_font_size(font_size),
        TextLayout::new_with_justify(JustifyText::Center),
    );

    (
        ClassicCell { row, col, given },
        Button,
        ThemedBackgroundColor,
        ThemedBorderColor,
        if given {
            FocusOutline::new(Color::NONE, Some(given_focus_border()))
        } else {
            FocusOutline::transparent()
        },
        BorderColor(Color::NONE),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        children![text_bundle],
    )
}

struct RowBundleOptions {
    node: Node,
    cells: Row<NUM_COLS>,
    font_size: f32,
    row: usize,
}

fn row_bundle(options: RowBundleOptions) -> impl Bundle {
    let RowBundleOptions {
        node,
        cells,
        font_size,
        row,
    } = options;
    let cell_bundles = cells.into_iter().enumerate().map(move |(col, cell)| {
        cell_bundle(CellBundleOptions {
            cell_value: cell,
            font_size,
            row,
            col,
        })
    });

    let row_height = 100.0 / NUM_ROWS as f32;

    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(row_height),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Stretch,
            align_items: AlignItems::Stretch,
            ..node
        },
        Children::spawn(SpawnIter(cell_bundles)),
    )
}

enum CellEdit {
    Set(u8),
    Clear,
}

fn key_to_cell_edit(key: &Key) -> Option<CellEdit> {
    match key {
        Key::Character(value) => {
            let mut chars = value.chars();
            let ch = chars.next()?;
            if chars.next().is_some() {
                return None;
            }
            let digit = ch.to_digit(10)? as u8;
            if (1..=9).contains(&digit) {
                Some(CellEdit::Set(digit))
            } else {
                None
            }
        }
        Key::Backspace => Some(CellEdit::Clear),
        Key::Delete => Some(CellEdit::Clear),
        _ => None,
    }
}

pub(crate) fn classic_cell_input_system(
    focused_entity: Res<FocusedEntity>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut grid_state: ResMut<ClassicGridState>,
    mut cell_query: Query<(Entity, &ClassicCell, &Children)>,
    mut text_query: Query<&mut Text, With<ClassicCellText>>,
    mut commands: Commands,
    mut escape_nav_state: ResMut<EscapeNavState>,
) {
    let Some(target_entity) = focused_entity.current.or(focused_entity.last) else {
        return;
    };

    let Ok((focused_entity_id, cell, children)) = cell_query.get_mut(target_entity) else {
        return;
    };

    let mut pending_edit: Option<CellEdit> = None;
    let mut pending_move: Option<(isize, isize)> = None;
    let mut clear_focus = false;

    let text_entity = children.first().copied();
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
                continue;
            }
            Key::ArrowUp => pending_move = Some((-1, 0)),
            Key::ArrowDown => pending_move = Some((1, 0)),
            Key::ArrowLeft => pending_move = Some((0, -1)),
            Key::ArrowRight => pending_move = Some((0, 1)),
            _ => {
                if !cell.given {
                    if let Some(edit) = key_to_cell_edit(&keyboard_input_event.logical_key) {
                        pending_edit = Some(edit);
                    }
                }
            }
        }
    }

    if clear_focus {
        escape_nav_state.focus_cleared_this_frame = true;
        commands.insert_resource(FocusedEntity {
            last: focused_entity.current,
            current: None,
        });
        return;
    }

    if let Some((row_delta, col_delta)) = pending_move {
        let new_row = cell.row as isize + row_delta;
        let new_col = cell.col as isize + col_delta;
        if (0..NUM_ROWS as isize).contains(&new_row) && (0..NUM_COLS as isize).contains(&new_col) {
            if let Some(new_focus) =
                find_cell_entity(&cell_query, new_row as usize, new_col as usize)
            {
                let previous = focused_entity.current.unwrap_or(focused_entity_id);
                commands.insert_resource(FocusedEntity {
                    last: Some(previous),
                    current: Some(new_focus),
                });
            }
        }
        return;
    }

    if let Some(edit) = pending_edit {
        let Some(text_entity) = text_entity else {
            return;
        };
        if focused_entity.current.is_none() {
            commands.insert_resource(FocusedEntity {
                last: focused_entity.last,
                current: Some(focused_entity_id),
            });
        }
        match edit {
            CellEdit::Set(value) => {
                grid_state.set(cell.row, cell.col, Some(value));
                if let Ok(mut text) = text_query.get_mut(text_entity) {
                    text.0 = value.to_string();
                }
            }
            CellEdit::Clear => {
                grid_state.set(cell.row, cell.col, None);
                if let Ok(mut text) = text_query.get_mut(text_entity) {
                    text.0.clear();
                }
            }
        }
    }
}

fn find_cell_entity(
    cell_query: &Query<(Entity, &ClassicCell, &Children)>,
    target_row: usize,
    target_col: usize,
) -> Option<Entity> {
    cell_query.iter().find_map(|(entity, cell, _)| {
        (cell.row == target_row && cell.col == target_col).then_some(entity)
    })
}

#[must_use]
pub fn classic_puzzle_bundle(grid: ClassicGrid) -> impl Bundle {
    let font_size = 40.0;

    let row_bundles = grid.0.into_iter().enumerate().map(move |(row, row_cells)| {
        row_bundle(RowBundleOptions {
            node: default(),
            cells: row_cells,
            font_size,
            row,
        })
    });

    (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::Stretch,
            width: Val::Percent(96.0),
            max_width: Val::Px(65.0 * PIXELS_PER_CH),
            aspect_ratio: Some(1.0),
            ..default()
        },
        ThemedBackgroundColor,
        ThemedBorderColor,
        ThemedBorderRadius,
        ThemedBorderRect,
        Children::spawn(SpawnIter(row_bundles)),
    )
}
