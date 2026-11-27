use std::fmt::{Display, Write};

use arrayvec::ArrayVec;

use crate::puzzles::Grid;

pub const NUM_COLS: usize = 9;
pub const NUM_ROWS: usize = 9;
pub const BOX_SIZE: usize = 3;

#[derive(Clone, Copy, Debug, Default)]
pub struct ClassicGrid(pub Grid<NUM_COLS, NUM_ROWS>);

pub struct ColIter<'a> {
    grid: &'a ClassicGrid,
    row: u8,
    col: u8,
}

impl<'a> ColIter<'a> {
    /// Create a `ColIter` for iterating over a column of cells in the grid.
    #[must_use]
    pub fn new(grid: &'a ClassicGrid, col: u8) -> Self {
        Self { grid, row: 0, col }
    }
}

impl<'a> Iterator for ColIter<'a> {
    type Item = &'a Option<u8>;

    /// Iterate over the cells in a column of the grid.
    fn next(&mut self) -> Option<Self::Item> {
        if self.row as usize >= NUM_ROWS {
            return None;
        }
        let val = &self.grid.0[self.row as usize][self.col as usize];
        self.row += 1;
        Some(val)
    }
}

pub struct BoxIter<'a> {
    grid: &'a ClassicGrid,
    row_start: u8,
    col_start: u8,
    index: u8,
}

impl<'a> BoxIter<'a> {
    /// Create a `BoxIter` for iterating over a box of cells in the grid.
    #[must_use]
    pub fn new(grid: &'a ClassicGrid, box_index: u8) -> Self {
        Self {
            grid,
            row_start: (box_index / 3) * 3,
            col_start: (box_index % 3) * 3,
            index: 0,
        }
    }
}

impl<'a> Iterator for BoxIter<'a> {
    type Item = &'a Option<u8>;

    /// Iterate over the cells in a box of the grid.
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 9 {
            return None;
        }
        let row = self.row_start + (self.index / 3);
        let col = self.col_start + (self.index % 3);
        let val = &self.grid.0[row as usize][col as usize];
        self.index += 1;
        Some(val)
    }
}

pub const CLASSIC_NEIGHBOR_CAPACITY: usize = 20;

impl ClassicGrid {
    /// Iterate over all cells in the grid.
    pub fn iter_all(&self) -> impl Iterator<Item = &Option<u8>> {
        self.0.iter().flatten()
    }

    /// Iterate over a row of cells in the grid.
    pub fn iter_row(&self, row: u8) -> impl Iterator<Item = &Option<u8>> {
        self.0[row as usize].iter()
    }

    /// Iterate over a column of cells in the grid.
    #[must_use]
    pub fn iter_col(&self, col: u8) -> ColIter<'_> {
        ColIter::new(self, col)
    }

    /// Iterate over a box of cells in the grid.
    #[must_use]
    pub fn iter_box(&self, box_index: u8) -> BoxIter<'_> {
        BoxIter::new(self, box_index)
    }

    /// Iterate over all neighboring positions (row, column, and box) for a cell.
    pub fn neighbor_positions(row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
        debug_assert!(row < NUM_ROWS && col < NUM_COLS);

        let mut neighbors: ArrayVec<(usize, usize), CLASSIC_NEIGHBOR_CAPACITY> = ArrayVec::new();

        for c in 0..NUM_COLS {
            if c != col {
                neighbors.push((row, c));
            }
        }

        for r in 0..NUM_ROWS {
            if r != row {
                neighbors.push((r, col));
            }
        }

        let box_row_start = (row / BOX_SIZE) * BOX_SIZE;
        let box_col_start = (col / BOX_SIZE) * BOX_SIZE;
        for local_row in 0..BOX_SIZE {
            for local_col in 0..BOX_SIZE {
                let r = box_row_start + local_row;
                let c = box_col_start + local_col;
                if r == row || c == col {
                    continue;
                }
                neighbors.push((r, c));
            }
        }

        neighbors.into_iter()
    }

    /// Iterate over the neighboring cells for a cell, yielding their positions and values.
    pub fn iter_neighbors(
        &self,
        row: usize,
        col: usize,
    ) -> impl Iterator<Item = ((usize, usize), &Option<u8>)> + '_ {
        let grid = &self.0;
        Self::neighbor_positions(row, col).map(move |(r, c)| ((r, c), &grid[r][c]))
    }

    /// Get the value of a cell in the grid by its row and column indices.
    #[must_use]
    pub fn get_by_row_col(&self, (row, col): (u8, u8)) -> Option<u8> {
        self.0[row as usize][col as usize]
    }

    /// Get the value of a cell in the grid by its global index.
    #[must_use]
    pub fn get_by_cell_index(&self, index: u8) -> Option<u8> {
        let row = index / 9;
        let col = index % 9;
        self.get_by_row_col((row, col))
    }

    /// Set the value of a cell in the grid.
    pub fn set(&mut self, (row, col): (u8, u8), val: Option<u8>) {
        self.0[row as usize][col as usize] = val;
    }
}

impl Display for ClassicGrid {
    /// Display the grid in a human-readable format.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (row_index, row_cells) in self.0.iter().enumerate() {
            for (col_index, cell) in row_cells.iter().enumerate() {
                // Push the a character representing the value of the cell (or . if empty)
                match cell {
                    Some(n) => f.write_char(std::char::from_digit(u32::from(*n), 10).unwrap())?,
                    None => f.write_char('.')?,
                }
                // Push the character(s) that follow each number
                match col_index {
                    8 => f.write_char('\n')?,
                    2 | 5 => f.write_str(" | ")?,
                    _ => f.write_char(' ')?,
                }
            }
            // Push a row separator if needed
            if row_index == 2 || row_index == 5 {
                f.write_str("------|-------|------\n")?;
            }
        }
        Ok(())
    }
}

impl From<&str> for ClassicGrid {
    /// Create a `ClassicGrid` from a string representation.
    fn from(s: &str) -> Self {
        let mut grid = ClassicGrid::default();

        // Set the values in the grid
        for (row, line) in s.lines().filter(|l| !l.starts_with('-')).enumerate() {
            for (col, c) in line
                .chars()
                .filter(|&c| c.is_ascii_digit() || c == '.')
                .enumerate()
            {
                if let Some(num) = c.to_digit(10) {
                    grid.0[row][col] = Some(num as u8);
                }
            }
        }

        grid
    }
}

impl From<[[u8; 9]; 9]> for ClassicGrid {
    /// Create a `ClassicGrid` from a 2D array of u8 values.
    fn from(nums: [[u8; 9]; 9]) -> Self {
        ClassicGrid(nums.map(|row| row.map(|num| if num == 0 { None } else { Some(num) })))
    }
}

impl PartialEq for ClassicGrid {
    /// Check if two `ClassicGrids` are equal.
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::collections::HashSet;

    const GRID_NUMS: [[u8; 9]; 9] = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    const GRID_STR: &str = indoc! {"
        5 3 . | . 7 . | . . .
        6 . . | 1 9 5 | . . .
        . 9 8 | . . . | . 6 .
        ------|-------|------
        8 . . | . 6 . | . . 3
        4 . . | 8 . 3 | . . 1
        7 . . | . 2 . | . . 6
        ------|-------|------
        . 6 . | . . . | 2 8 .
        . . . | 4 1 9 | . . 5
        . . . | . 8 . | . 7 9
    "};

    #[test]
    fn test_display() {
        let grid = ClassicGrid::from(GRID_NUMS);
        assert_eq!(grid.to_string(), GRID_STR);
    }

    #[test]
    fn test_from_str() {
        let grid_from_str = ClassicGrid::from(GRID_STR);
        let expected_grid = ClassicGrid::from(GRID_NUMS);
        assert_eq!(grid_from_str, expected_grid);
    }

    #[test]
    fn test_iter_all() {
        let grid = ClassicGrid::from(GRID_NUMS);
        for (i, cell) in grid.iter_all().enumerate() {
            assert_eq!(cell, &grid.get_by_cell_index(i as u8));
        }
    }

    #[test]
    fn test_neighbor_positions_center() {
        let mut neighbors: Vec<_> = ClassicGrid::neighbor_positions(4, 4).collect();
        neighbors.sort_unstable();
        neighbors.dedup();
        assert_eq!(neighbors.len(), 20);
        assert!(!neighbors.contains(&(4, 4)));
    }

    #[test]
    fn test_neighbor_positions_corner_unique() {
        let neighbors: Vec<_> = ClassicGrid::neighbor_positions(0, 0).collect();
        let unique: HashSet<_> = neighbors.iter().copied().collect();
        assert_eq!(neighbors.len(), 20);
        assert_eq!(neighbors.len(), unique.len());
        assert!(unique.contains(&(1, 1)));
        assert!(unique.contains(&(2, 2)));
    }

    #[test]
    fn test_iter_neighbors_matches_grid_values() {
        let grid = ClassicGrid::from(GRID_NUMS);
        let mut neighbors: Vec<_> = grid
            .iter_neighbors(4, 4)
            .map(|((row, col), value)| (row, col, *value))
            .collect();
        neighbors.sort_unstable();
        assert_eq!(neighbors.len(), 20);
        for (row, col, value) in neighbors {
            assert_eq!(value, grid.get_by_row_col((row as u8, col as u8)));
        }
    }
}
