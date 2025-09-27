use bevy::{ecs::spawn::SpawnIter, prelude::*};

use crate::{
    plugins::{
        common::theme::{
            node::{
                ThemedBackgroundColor, ThemedBorderColor, ThemedBorderRadius, ThemedBorderRect,
            },
            text::{ThemedFontWeight, ThemedTextColor},
        },
        screens::PIXELS_PER_CH,
    },
    puzzles::{
        classic::grid::{ClassicGrid, NUM_COLS},
        CellValue, Row,
    },
};

struct CellBundleOptions {
    cell_value: CellValue,
    font_size: f32,
    row: usize,
    col: usize,
}

fn cell_border(row: usize, col: usize) -> UiRect {
    let left = match col {
        3 | 6 => 2.0,
        _ => 1.0,
    };
    let right = match col {
        2 | 5 => 2.0,
        _ => 1.0,
    };
    let top = match row {
        3 | 6 => 2.0,
        _ => 1.0,
    };
    let bottom = match row {
        2 | 5 => 2.0,
        _ => 1.0,
    };

    UiRect {
        left: Val::Px(left),
        right: Val::Px(right),
        top: Val::Px(top),
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
    let value_string = match cell_value {
        Some(value) => value.to_string(),
        None => String::new(),
    };

    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border: cell_border(row, col),
            ..default()
        },
        ThemedBorderColor,
        children![(
            ThemedFontWeight::Regular,
            ThemedTextColor,
            Text::from(value_string),
            TextFont::from_font_size(font_size),
            TextLayout::new_with_justify(JustifyText::Center),
        )],
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

    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(-1.0),
            ..node
        },
        Children::spawn(SpawnIter(cell_bundles)),
    )
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
            row_gap: Val::Px(-1.0),
            width: Val::Percent(96.0),
            max_width: Val::Px(65.0 * PIXELS_PER_CH),
            height: Val::Percent(96.0),
            max_height: Val::Px(65.0 * PIXELS_PER_CH),
            ..default()
        },
        ThemedBackgroundColor,
        ThemedBorderColor,
        ThemedBorderRadius,
        ThemedBorderRect,
        Children::spawn(SpawnIter(row_bundles)),
    )
}
