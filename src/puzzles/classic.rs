use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};
use rand_seeder::{SipHasher, SipRng};
use std::fmt::Display;

use crate::{
    grids::classic::ClassicGrid,
    utility::{element_set::ElementSet, priority_queue::PriorityQueue},
};

#[derive(Clone)]
pub struct ClassicPuzzle {
    /// The actual Sudoku grid
    grid: ClassicGrid,
    /// The remaining numbers that need to be placed for each row
    row_sets: [ElementSet; 9],
    /// The remaining numbers that need to be placed for each column
    col_sets: [ElementSet; 9],
    /// The remaining numbers that need to be placed for each 3x3 box
    box_sets: [ElementSet; 9],
    /// A priority queue for getting the next cell with the fewest possibilities
    empty_cell_queue: PriorityQueue<ElementSet>,
}

type CellCoords = (u8, u8, u8);
type CellIndex = u8;
type CellValue = Option<u8>;

impl ClassicPuzzle {
    /// Creates a new, blank, classic 9x9 Sudoku board
    pub fn new() -> Self {
        Self {
            grid: ClassicGrid::default(),
            row_sets: std::array::from_fn(|_| ElementSet::CLASSIC),
            col_sets: std::array::from_fn(|_| ElementSet::CLASSIC),
            box_sets: std::array::from_fn(|_| ElementSet::CLASSIC),
            empty_cell_queue: PriorityQueue::from_iter_unsafe(
                (0..81).map(|k| (k, ElementSet::CLASSIC)),
            ),
        }
    }

    fn get_cell_index((row, col): (u8, u8)) -> CellIndex {
        row * 9 + col
    }

    /// Calculates and returns the row and column indexes for some "cell index" (one from 0 to 80)
    fn get_row_col(cell_index: CellIndex) -> (u8, u8) {
        (cell_index / 9, cell_index % 9)
    }

    /// Calculates and returns the box index for some "cell index" (one from 0 to 80)
    fn get_box_index((row, col): (u8, u8)) -> u8 {
        (row / 3) * 3 + (col / 3)
    }

    /// Calculates and returns the row, column, and box indexes for some "cell index" (one from 0
    /// to 80)
    fn get_cell_coords(cell_index: u8) -> CellCoords {
        let (row, col) = Self::get_row_col(cell_index);
        let box_index = Self::get_box_index((row, col));
        (row, col, box_index)
    }

    /// Sets a cell in the grid and removes the value from the corresponding sets
    pub fn set(&mut self, (row, col, box_index): CellCoords, val: u8) {
        // Update the sets
        self.row_sets[row as usize].remove(val);
        self.col_sets[col as usize].remove(val);
        self.box_sets[box_index as usize].remove(val);
        // Set the value in the grid
        self.grid.set((row, col), Some(val));
    }

    /// Clears a cell in the grid and adds the value to the corresponding sets
    pub fn delete(&mut self, (row, col, box_index): CellCoords) {
        // Get the current value
        if let Some(value) = self.grid.get((row, col)) {
            // Update the sets
            self.row_sets[row as usize].insert(value);
            self.col_sets[col as usize].insert(value);
            self.box_sets[box_index as usize].insert(value);
            // Clear the value in the grid
            self.grid.set((row, col), None);
        }
    }

    fn get_element_set(&self, (row, col, box_index): CellCoords) -> ElementSet {
        self.row_sets[row as usize]
            .intersection(&self.col_sets[col as usize])
            .intersection(&self.box_sets[box_index as usize])
    }

    /// Gets all of the cell indexes and values in a row, column, and box.
    fn get_group_cells(
        &self,
        (cell_row, cell_col, cell_box): CellCoords,
    ) -> Vec<(CellIndex, CellValue)> {
        let mut cells = Vec::<(CellIndex, CellValue)>::with_capacity(21);

        // Add cell indexes from the row
        for (col, &val) in self.grid.iter_row(cell_row).enumerate() {
            let cell_index = cell_row * 9 + col as u8;
            cells.push((cell_index, val));
        }

        // Add cells indexes from the col
        for (row, &val) in self.grid.iter_col(cell_col).enumerate() {
            let cell_index = (row * 9) as u8 + cell_col;
            if row != cell_row as usize {
                cells.push((cell_index, val));
            }
        }

        // Add cell indexes from the 3x3 box
        let tl_box_row = (cell_box / 3) * 3;
        let tl_box_col = (cell_box % 3) * 3;
        for (box_offset, &val) in self.grid.iter_box(cell_box).enumerate() {
            let row = tl_box_row + (box_offset / 3) as u8;
            let col = tl_box_col + (box_offset % 3) as u8;
            let cell_index = Self::get_cell_index((row, col));
            if (row != cell_row) && (col != cell_col) {
                cells.push((cell_index, val));
            }
        }

        cells
    }

    fn get_empty_group_cell_indexes(&self, cell_coords: CellCoords) -> Vec<CellIndex> {
        self.get_group_cells(cell_coords)
            .iter()
            .copied()
            .filter_map(|(ci, cv)| if cv.is_none() { Some(ci) } else { None })
            .collect()
    }

    pub fn fill_from_rng<T: Rng>(&mut self, mut rng: &mut T) {
        // List of cells used to initialize unfilled cell heap
        let mut all_cell_indexes: Vec<CellIndex> = (0..81).collect();

        // Shuffle the cells so that
        all_cell_indexes.shuffle(&mut rng);

        // Initialize a priority queue to get the next cell with the fewest possibilities
        self.empty_cell_queue.fill_from_iter_unsafe(
            all_cell_indexes
                .iter()
                .map(|&cell_index| (cell_index as usize, ElementSet::from(1..9))),
        );

        // Stack of cells that have already been filled and their unattempted possibilities
        let mut filled_cell_pairs: Vec<(CellIndex, ElementSet)> = Vec::new();

        // Fill the board
        while let Some(cell) = self.empty_cell_queue.pop() {
            let (current_cell_index, mut current_possibilities) = (cell.0 as u8, cell.1);
            let current_cell_coords = Self::get_cell_coords(current_cell_index);

            // Try choosing a random possibility
            if let Some(num) = current_possibilities.iter().choose(&mut rng) {
                // Remove this number from the set of unattempted possibilities for this index
                current_possibilities.remove(num);

                // Place the number in the cell
                self.set(current_cell_coords, num);

                // Update the possibilities left in the heap for each of the empty cells
                // related to the current cell.
                for cell_index in self.get_empty_group_cell_indexes(current_cell_coords) {
                    let mut element_set = *self
                        .empty_cell_queue
                        .get_priority_unsafe(cell_index as usize)
                        .unwrap();
                    element_set.remove(num);
                    self.empty_cell_queue
                        .insert_unsafe((cell_index as usize, element_set));
                }

                // Add this cell to the stack of filled cells
                filled_cell_pairs.push((current_cell_index, current_possibilities));
            } else {
                // Get the last filled cell
                let (previous_cell, previous_cell_possibilities) = filled_cell_pairs.pop().unwrap();
                let previous_cell_coords = Self::get_cell_coords(previous_cell);

                // Find the cells which may have had their possibilities changed by the last filled
                // cell.
                let dirty_cell_indexes = self.get_empty_group_cell_indexes(previous_cell_coords);

                // Remove the filled number from the board. Need to do this now so that the
                // possibilities for cells being reset are accurate.
                self.delete(previous_cell_coords);

                // Reset the possibilities for the current cell.
                let current_cell_possibilities = self.get_element_set(current_cell_coords);
                self.empty_cell_queue
                    .insert_unsafe((current_cell_index as usize, current_cell_possibilities));

                // Reset the possibilities for the dirty cells related to the last filled cell.
                for cell_index in dirty_cell_indexes {
                    let cell_coords = Self::get_cell_coords(cell_index);
                    let possibilities = self.get_element_set(cell_coords);
                    self.empty_cell_queue
                        .insert_unsafe((cell_index as usize, possibilities));
                }

                // Add the last filled cell back to the queue so that a different possibility can
                // be tried.
                self.empty_cell_queue
                    .insert_unsafe((previous_cell as usize, previous_cell_possibilities));
            }
        }
    }

    fn find_solutions_recursive(mut puzzle: ClassicPuzzle) -> Vec<ClassicGrid> {
        let mut solutions: Vec<ClassicGrid> = Vec::new();

        // If the queue is empty, the puzzle is solved
        if puzzle.empty_cell_queue.is_empty() {
            solutions.push(puzzle.grid);
            return solutions;
        }

        // Get the next cell to fill
        let (cell_index, cell_possibilities) = puzzle.empty_cell_queue.pop().unwrap();
        let cell_coords = Self::get_cell_coords(cell_index as u8);

        // Get the cell indexes for the empty cells related to the current cell
        let empty_cell_indexes = puzzle.get_empty_group_cell_indexes(cell_coords);

        // Try each possibility
        for num in cell_possibilities.iter() {
            puzzle.set(cell_coords, num);

            // Update the possibilities left in the queue for each of these related cells.
            for ci in empty_cell_indexes
                .iter()
                .cloned()
                .filter(|&ci| ci as usize != cell_index)
            {
                let element_set = puzzle.get_element_set(Self::get_cell_coords(ci));
                puzzle
                    .empty_cell_queue
                    .insert_unsafe((ci as usize, element_set));
            }

            // Recurse
            solutions.append(&mut Self::find_solutions_recursive(puzzle.clone()));

            // Need to clear the cell to update the sets
            puzzle.delete(cell_coords);
        }

        solutions
    }

    // FIXME
    fn _find_solutions_iterative(mut puzzle: ClassicPuzzle) -> Vec<ClassicGrid> {
        let mut solutions = Vec::new();

        // If the queue is empty, the puzzle is solved
        if puzzle.empty_cell_queue.is_empty() {
            println!("Puzzle is empty! Returning the current grid!");
            solutions.push(puzzle.grid);
            return solutions;
        }

        let mut stack = Vec::new();

        // Initialize the stack with the first cell
        stack.push(puzzle.empty_cell_queue.pop().unwrap());

        while let Some((current_cell_index, mut current_cell_possibilities)) = stack.pop() {
            if current_cell_possibilities.is_empty() {
                continue;
            }

            // Try the next possibility
            let num = current_cell_possibilities.pop().unwrap();
            let current_cell_coords = Self::get_cell_coords(current_cell_index as u8);

            // Set the cell
            puzzle.set(current_cell_coords, num);

            // If there are no more empty cells, then the puzzle is solved
            if puzzle.empty_cell_queue.is_empty() {
                solutions.push(puzzle.grid);
                // Clear the cell and add it back to the queue
                puzzle.delete(current_cell_coords);
                // Ok to reuse current_cell_possibilities, because it should be empty
                current_cell_possibilities.insert(num);
                puzzle
                    .empty_cell_queue
                    .insert_unsafe((current_cell_index, current_cell_possibilities));
                continue;
            }

            let empty_cell_indexes = puzzle.get_empty_group_cell_indexes(current_cell_coords);

            // If any of the related empty cells have only one possibility and it would be
            // eliminated by filling the current cell, then clear the current cell, update
            // the possibilities for the related empty cells, and continue to the next cell.
            if empty_cell_indexes.iter().any(|eci| {
                let ecp = puzzle
                    .empty_cell_queue
                    .get_priority_unsafe(*eci as usize)
                    .unwrap();
                ecp.len() == 1 && ecp.has(num)
            }) {
                puzzle.delete(current_cell_coords);
                for ci in empty_cell_indexes {
                    let element_set = puzzle.get_element_set(Self::get_cell_coords(ci));
                    puzzle
                        .empty_cell_queue
                        .insert_unsafe((ci as usize, element_set));
                }
                stack.push((current_cell_index, current_cell_possibilities));
                continue;
            }

            // Remove the number from the possibilities for the related empty cells
            for ci in puzzle.get_empty_group_cell_indexes(current_cell_coords) {
                let mut element_set = *puzzle
                    .empty_cell_queue
                    .get_priority_unsafe(ci as usize)
                    .unwrap();
                element_set.remove(num);
                puzzle
                    .empty_cell_queue
                    .insert_unsafe((ci as usize, element_set));
            }

            // Add the next cell to the stack
            stack.push(puzzle.empty_cell_queue.pop().unwrap());
        }

        solutions
    }

    /// Checks if the puzzle has exactly one solution
    fn is_well_posed(&self) -> bool {
        Self::find_solutions_recursive(self.clone()).len() == 1
    }

    pub fn remove_from_rng<T: Rng>(&mut self, mut rng: &mut T) {
        // Create a list of cells left to attempt
        let mut all_cell_indexes: Vec<CellIndex> = (0..81).collect();

        // Shuffle the cells
        all_cell_indexes.shuffle(&mut rng);

        // Loop until there are no cells left to attempt
        while let Some(current_cell_index) = all_cell_indexes.pop() {
            let cell_coords = Self::get_cell_coords(current_cell_index);

            let val = self.grid.get((cell_coords.0, cell_coords.1)).unwrap();

            // Try to remove the value from this cell
            self.delete(cell_coords);

            // Make a clone of the cell queue to reset it later if necessary
            let original_empty_cell_queue = self.empty_cell_queue.clone();

            // Add this cell to the empty cell queue and update the possibilities for all of the
            // empty cells in its group.
            for cell_index in self.get_empty_group_cell_indexes(cell_coords) {
                let cell_coords = Self::get_cell_coords(cell_index);
                let possibilities = self.get_element_set(cell_coords);
                self.empty_cell_queue
                    .insert_unsafe((cell_index as usize, possibilities));
            }

            // If the board is well-posed (there is only one solution), then continue clearing cells
            if self.is_well_posed() {
                continue;
            }

            // Put the value back if the puzzle is no longer well-posed
            self.set(cell_coords, val);

            // Need to remove this cell from the queue and reset the possibilities for cells in its
            // group.
            self.empty_cell_queue = original_empty_cell_queue;
        }
    }

    /// Creates and sets up a puzzle given some string seed
    pub fn from_seed(seed: String) -> Self {
        let mut puzzle = ClassicPuzzle::new();

        let mut rng: SipRng = SipHasher::from(seed).into_rng();

        // Fill the board
        puzzle.fill_from_rng(&mut rng);

        // Remove numbers
        puzzle.remove_from_rng(&mut rng);

        puzzle
    }

    pub fn num_clues(&self) -> u8 {
        (0..9).fold(0, |acc: u8, row| acc + (9 - self.row_sets[row].len()))
    }
}

impl Display for ClassicPuzzle {
    /// Displays the puzzle as a string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.fmt(f)
    }
}

impl From<ClassicGrid> for ClassicPuzzle {
    /// Creates a new ClassicPuzzle from a ClassicGrid
    fn from(grid: ClassicGrid) -> Self {
        fn set_from_iter<'a, I: Iterator<Item = &'a Option<u8>>>(iter: I) -> ElementSet {
            let mut element_set = ElementSet::CLASSIC;
            iter.for_each(|o| {
                if let Some(num) = o {
                    element_set.remove(*num);
                }
            });
            element_set
        }
        let row_sets = std::array::from_fn(|i| set_from_iter(grid.iter_row(i as u8)));
        let col_sets = std::array::from_fn(|i| set_from_iter(grid.iter_col(i as u8)));
        let box_sets = std::array::from_fn(|i| set_from_iter(grid.iter_box(i as u8)));

        let mut empty_cell_queue = PriorityQueue::with_capacity(81);
        empty_cell_queue.init_map_none(81);
        empty_cell_queue.fill_from_iter_unsafe((0..81).filter_map(|cell_index| {
            let (row_index, col_index, box_index) = Self::get_cell_coords(cell_index as u8);
            if grid.get((row_index, col_index)).is_none() {
                let element_set = row_sets[row_index as usize]
                    .intersection(&col_sets[col_index as usize])
                    .intersection(&box_sets[box_index as usize]);
                Some((cell_index, element_set))
            } else {
                None
            }
        }));

        Self {
            grid,
            row_sets,
            col_sets,
            box_sets,
            empty_cell_queue,
        }
    }
}

impl From<&str> for ClassicPuzzle {
    /// Creates a new ClassicPuzzle from a grid string
    fn from(s: &str) -> Self {
        Self::from(ClassicGrid::from(s))
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;
    use crate::utility::seed::SeedRng;

    const SEED: &str = "test";

    const FILLED_PUZZLE_STR: &str = indoc! {"
        6 5 4 | 7 8 1 | 2 3 9
        9 1 8 | 3 4 2 | 5 6 7
        7 3 2 | 6 5 9 | 1 4 8
        ------|-------|------
        8 4 6 | 2 7 3 | 9 1 5
        1 7 3 | 5 9 8 | 6 2 4
        5 2 9 | 1 6 4 | 7 8 3
        ------|-------|------
        4 8 1 | 9 2 7 | 3 5 6
        3 6 7 | 4 1 5 | 8 9 2
        2 9 5 | 8 3 6 | 4 7 1
    "};

    const MINIMUM_PUZZLE_STR: &str = indoc! {"
        . 5 . | . . 1 | . . 9
        . . 8 | . . . | 5 . .
        7 . . | . 5 . | 1 . 8
        ------|-------|------
        . 4 . | 2 . . | . . .
        1 . 3 | . . . | . 2 .
        . . 9 | . . . | 7 . 3
        ------|-------|------
        . . . | . . 7 | . 5 .
        3 . 7 | . 1 . | . . .
        . . . | 8 . . | 4 . .
    "};

    #[test]
    fn set_and_delete() {
        let mut puzzle = ClassicPuzzle::new();
        puzzle.set((0, 0, 0), 1);
        assert_eq!(puzzle.grid.get((0, 0)), Some(1));
        puzzle.delete((0, 0, 0));
        assert_eq!(puzzle.grid.get((0, 0)), None);
    }

    #[test]
    fn get_cell_index() {
        assert_eq!(ClassicPuzzle::get_cell_index((0, 0)), 0);
        assert_eq!(ClassicPuzzle::get_cell_index((8, 8)), 80);
    }

    #[test]
    fn get_row_col() {
        assert_eq!(ClassicPuzzle::get_row_col(0), (0, 0));
        assert_eq!(ClassicPuzzle::get_row_col(80), (8, 8));
    }

    #[test]
    fn get_box_index() {
        assert_eq!(ClassicPuzzle::get_box_index((0, 0)), 0);
        assert_eq!(ClassicPuzzle::get_box_index((8, 8)), 8);
    }

    #[test]
    fn get_cell_coords() {
        assert_eq!(ClassicPuzzle::get_cell_coords(0), (0, 0, 0));
        assert_eq!(ClassicPuzzle::get_cell_coords(80), (8, 8, 8));
    }

    #[test]
    fn from_grid_empty() {
        let grid = ClassicGrid::default();
        let puzzle = ClassicPuzzle::from(grid);
        assert_eq!(puzzle.grid, grid);
        assert!(puzzle
            .row_sets
            .iter()
            .all(|row_set| row_set == &ElementSet::CLASSIC));
        assert!(puzzle
            .col_sets
            .iter()
            .all(|col_set| col_set == &ElementSet::CLASSIC));
        assert!(puzzle
            .box_sets
            .iter()
            .all(|box_set| box_set == &ElementSet::CLASSIC));
        assert_eq!(puzzle.empty_cell_queue.len(), 81);
    }

    #[test]
    fn from_grid_filled() {
        let grid = ClassicGrid::from(FILLED_PUZZLE_STR);
        let puzzle = ClassicPuzzle::from(grid);
        assert_eq!(puzzle.grid, grid);
        assert!(puzzle.row_sets.iter().all(|row_set| row_set.is_empty()));
        assert!(puzzle.col_sets.iter().all(|col_set| col_set.is_empty()));
        assert!(puzzle.box_sets.iter().all(|box_set| box_set.is_empty()));
        assert!(puzzle.empty_cell_queue.is_empty());
    }

    #[test]
    fn from_grid_minimum() {
        let grid = ClassicGrid::from(MINIMUM_PUZZLE_STR);
        let puzzle = ClassicPuzzle::from(grid);
        let expected_row_set_lens = [6, 7, 5, 7, 6, 6, 7, 6, 7];
        let expected_col_set_lens = [6, 7, 5, 7, 7, 7, 5, 7, 6];
        let expected_box_set_lens = [6, 7, 5, 5, 8, 6, 7, 6, 7];
        let check_sizes = |sets: &[ElementSet], expected_lens: &[u8], set_type: &str| {
            sets.iter().enumerate().for_each(|(i, set)| {
                let actual_len = set.len();
                let expected_len = expected_lens[i];
                assert_eq!(
                    actual_len, expected_len,
                    "expected {} set {} to have a length of {} but it was {}",
                    set_type, i, expected_len, actual_len
                );
            })
        };
        assert_eq!(puzzle.grid, grid);
        check_sizes(&puzzle.row_sets, &expected_row_set_lens, "row");
        check_sizes(&puzzle.col_sets, &expected_col_set_lens, "col");
        check_sizes(&puzzle.box_sets, &expected_box_set_lens, "box");
        assert_eq!(puzzle.empty_cell_queue.len(), 57);
    }

    #[test]
    fn display() {
        let puzzle = ClassicPuzzle::from(FILLED_PUZZLE_STR);
        assert_eq!(puzzle.to_string(), FILLED_PUZZLE_STR);
    }

    #[test]
    fn fill_from_rng_determinism() {
        let mut rng: SipRng = SipHasher::from(SEED).into_rng();
        let mut puzzle = ClassicPuzzle::new();
        puzzle.fill_from_rng(&mut rng);
        let puzzle_str = puzzle.to_string();
        assert_eq!(
            puzzle_str,
            FILLED_PUZZLE_STR,
            "Generated puzzle\n{}should equal\n{}",
            puzzle_str.replace("\n", "    \n"),
            FILLED_PUZZLE_STR.replace("\n", "    \n")
        );
    }

    #[test]
    fn fill_from_rng_total() {
        let mut seed_rng: SipRng = SipHasher::from(SEED).into_rng();
        for _ in 0..10_000 {
            let seed = seed_rng.gen_seed();
            let mut rng = SipHasher::from(seed).into_rng();
            let mut puzzle = ClassicPuzzle::new();
            puzzle.fill_from_rng(&mut rng);
        }
    }

    #[test]
    fn find_solutions_filled() {
        let puzzle = ClassicPuzzle::from(FILLED_PUZZLE_STR);
        let solutions = ClassicPuzzle::find_solutions_recursive(puzzle.clone());
        assert_eq!(solutions.len(), 1);
    }

    #[test]
    fn find_solutions_one_missing() {
        let mut puzzle = ClassicPuzzle::from(FILLED_PUZZLE_STR);
        puzzle.delete((0, 1, 0));
        let solutions = ClassicPuzzle::find_solutions_recursive(puzzle.clone());
        assert_eq!(solutions.len(), 1);
    }

    #[test]
    fn find_solutions_row_missing() {
        let mut puzzle = ClassicPuzzle::from(FILLED_PUZZLE_STR);
        for col in 0..9 {
            puzzle.delete((0, col, 0));
        }
        let solutions = ClassicPuzzle::find_solutions_recursive(puzzle.clone());
        assert_eq!(solutions.len(), 1);
    }

    // FIXME - The iterative solution finding algorithm is not working correctly. After it's fixed,
    // benchmark it against the recursive solution finding algorithm.
    #[test]
    fn find_solutions_minimum() {
        let puzzle = ClassicPuzzle::from(MINIMUM_PUZZLE_STR);
        let solutions = ClassicPuzzle::find_solutions_recursive(puzzle.clone());
        assert_eq!(solutions.len(), 1);
    }

    #[test]
    fn find_solutions_multiple() {
        let mut puzzle = ClassicPuzzle::from(MINIMUM_PUZZLE_STR);
        puzzle.delete((0, 1, 0));
        let solutions = ClassicPuzzle::find_solutions_recursive(puzzle.clone());
        assert!(solutions.len() > 1);
    }

    #[test]
    fn is_well_posed() {
        let puzzle = ClassicPuzzle::from(MINIMUM_PUZZLE_STR);
        assert!(puzzle.is_well_posed());
    }

    #[test]
    fn is_not_well_posed() {
        let mut puzzle = ClassicPuzzle::from(MINIMUM_PUZZLE_STR);
        puzzle.delete((0, 1, 0));
        assert!(!puzzle.is_well_posed());
    }

    #[test]
    fn remove_from_rng_determinism() {
        let mut puzzle = ClassicPuzzle::from(FILLED_PUZZLE_STR);
        let mut rng: SipRng = SipHasher::from(SEED).into_rng();
        puzzle.remove_from_rng(&mut rng);
        let puzzle_str = puzzle.to_string();
        assert_eq!(
            puzzle_str,
            MINIMUM_PUZZLE_STR,
            "Generated puzzle\n{}should equal\n{}",
            puzzle_str.replace("\n", "    \n"),
            MINIMUM_PUZZLE_STR.replace("\n", "    \n")
        );
    }
}
