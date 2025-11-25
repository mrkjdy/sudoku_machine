use bevy::{ecs::spawn::SpawnIter, prelude::*};

use crate::{
    plugins::{
        common::{
            bundles::puzzle_cell::{
                puzzle_cell_bundle, PuzzleCellBundleOptions, PuzzleCellPosition,
            },
            theme::node::{
                ThemedBackgroundColor, ThemedBorderColor, ThemedBorderRadius, ThemedBorderRect,
            },
        },
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

#[derive(Resource, Clone, Copy)]
pub struct ClassicGridState {
    grid: ClassicGrid,
}

impl ClassicGridState {
    #[must_use]
    pub const fn new(grid: ClassicGrid) -> Self {
        Self { grid }
    }

    pub fn set(&mut self, row: usize, col: usize, value: CellValue) {
        self.grid.set((row as u8, col as u8), value);
    }

    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> CellValue {
        self.grid.get_by_row_col((row as u8, col as u8))
    }
}

fn block_separator_width(index: usize, max_index: usize) -> f32 {
    if index >= max_index {
        0.0
    } else if (index + 1).is_multiple_of(BLOCK_SIZE) {
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
        let given = cell.is_some();
        let label = cell.map(|value| value.to_string()).unwrap_or_default();
        let cell_options = PuzzleCellBundleOptions {
            label,
            font_size,
            given,
            cell_node: Node {
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            text_node: default(),
        };
        let cell_bundle = (
            PuzzleCellPosition { row, col },
            puzzle_cell_bundle(cell_options),
        );

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
            children![cell_bundle],
        )
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
