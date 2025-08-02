use std::fmt::{Display, Write};

use crate::puzzles::Grid;

pub const NUM_COLS: usize = 9;
pub const NUM_ROWS: usize = 9;

#[derive(Clone, Copy, Debug, Default)]
pub struct ClassicGrid(pub Grid<NUM_COLS, NUM_ROWS>);

pub struct ColIter<'a> {
    grid: &'a ClassicGrid,
    row: u8,
    col: u8,
}

impl<'a> ColIter<'a> {
    pub fn new(grid: &'a ClassicGrid, col: u8) -> Self {
        Self { grid, row: 0, col }
    }
}

impl<'a> Iterator for ColIter<'a> {
    type Item = &'a Option<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= NUM_ROWS as u8 {
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

impl ClassicGrid {
    pub fn iter_row(&self, row: u8) -> impl Iterator<Item = &Option<u8>> {
        self.0[row as usize].iter()
    }

    pub fn iter_col(&self, col: u8) -> ColIter {
        ColIter::new(self, col)
    }

    pub fn iter_box(&self, box_index: u8) -> BoxIter {
        BoxIter::new(self, box_index)
    }

    pub fn get(&self, (row, col): (u8, u8)) -> Option<u8> {
        self.0[row as usize][col as usize]
    }

    pub fn set(&mut self, (row, col): (u8, u8), val: Option<u8>) {
        self.0[row as usize][col as usize] = val;
    }
}

impl Display for ClassicGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (row_index, row_cells) in self.0.iter().enumerate() {
            for (col_index, cell) in row_cells.iter().enumerate() {
                // Push the a character representing the value of the cell (or . if empty)
                match cell {
                    Some(n) => f.write_char(std::char::from_digit((*n) as u32, 10).unwrap())?,
                    None => f.write_char('.')?,
                };
                // Push the character(s) that follow each number
                match col_index {
                    8 => f.write_char('\n')?,
                    2 | 5 => f.write_str(" | ")?,
                    _ => f.write_char(' ')?,
                };
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
    fn from(s: &str) -> Self {
        let mut grid = ClassicGrid::default();

        // Set the values in the grid
        for (row, line) in s.lines().filter(|l| !l.starts_with("-")).enumerate() {
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
    fn from(nums: [[u8; 9]; 9]) -> Self {
        ClassicGrid(nums.map(|row| row.map(|num| if num == 0 { None } else { Some(num) })))
    }
}

impl PartialEq for ClassicGrid {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

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
}
