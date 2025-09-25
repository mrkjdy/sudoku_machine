use arrayvec::ArrayVec;
use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};
use rand_seeder::{SipHasher, SipRng};
use std::fmt::Display;

use crate::{
    grids::classic::ClassicGrid,
    utility::{element_set::ElementSet, priority_queue::ArrayPriorityQueue},
};

/// The total number of cells in a classic 9x9 Sudoku board.
const BOARD_SIZE: usize = 9 * 9;
/// The number of cells in a "group" (row, column, and box) without repeats.
const GROUP_SIZE: usize = 9 + 8 + 4;

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
    empty_cell_queue: ArrayPriorityQueue<ElementSet, BOARD_SIZE>,
}

pub type CellCoords = (u8, u8, u8);
pub type CellIndex = u8;
pub type CellValue = Option<u8>;

impl Default for ClassicPuzzle {
    fn default() -> Self {
        Self::new()
    }
}

impl ClassicPuzzle {
    /// Creates a new, blank, classic 9x9 Sudoku board
    #[must_use]
    pub fn new() -> Self {
        Self {
            grid: ClassicGrid::default(),
            row_sets: std::array::from_fn(|_| ElementSet::CLASSIC),
            col_sets: std::array::from_fn(|_| ElementSet::CLASSIC),
            box_sets: std::array::from_fn(|_| ElementSet::CLASSIC),
            empty_cell_queue: ArrayPriorityQueue::from_iter_unsafe(
                (0..BOARD_SIZE).map(|k| (k, ElementSet::CLASSIC)),
            ),
        }
    }

    /// Calculates and returns the "cell index" for some row and column indexes (0 to 8)
    fn get_cell_index((row, col): (u8, u8)) -> CellIndex {
        row * 9 + col
    }

    /// Calculates and returns the row and column indexes for some "cell index" (0 to 80)
    fn get_row_col(cell_index: CellIndex) -> (u8, u8) {
        (cell_index / 9, cell_index % 9)
    }

    /// Calculates and returns the box index for some "cell index" (0 to 80)
    fn get_box_index((row, col): (u8, u8)) -> u8 {
        (row / 3) * 3 + (col / 3)
    }

    /// Calculates and returns the row, column, and box indexes for some "cell index" (0 to 80)
    #[must_use]
    pub fn get_cell_coords(cell_index: CellIndex) -> CellCoords {
        let (row, col) = Self::get_row_col(cell_index);
        let box_index = Self::get_box_index((row, col));
        (row, col, box_index)
    }

    /// Sets a cell in the grid and removes the value from the corresponding sets
    pub fn set(&mut self, (row, col, box_index): CellCoords, val: u8) {
        debug_assert_eq!(box_index, Self::get_box_index((row, col)));
        // Update the sets
        self.row_sets[row as usize].remove(val);
        self.col_sets[col as usize].remove(val);
        self.box_sets[box_index as usize].remove(val);
        // Set the value in the grid
        self.grid.set((row, col), Some(val));
    }

    /// Clears a cell in the grid and adds the value to the corresponding sets
    pub fn delete(&mut self, (row, col, box_index): CellCoords) {
        debug_assert_eq!(box_index, Self::get_box_index((row, col)));
        // Get the current value
        if let Some(value) = self.grid.get_by_row_col((row, col)) {
            // Update the sets
            self.row_sets[row as usize].insert(value);
            self.col_sets[col as usize].insert(value);
            self.box_sets[box_index as usize].insert(value);
            // Clear the value in the grid
            self.grid.set((row, col), None);
        }
    }

    /// Gets the "element set" for a given cell. An "element set" is a set of all possible values
    /// that can be placed in a cell, based on the empty cells in the "group" (row, column, or box).
    fn get_element_set(&self, (row, col, box_index): CellCoords) -> ElementSet {
        self.row_sets[row as usize]
            .intersection(&self.col_sets[col as usize])
            .intersection(&self.box_sets[box_index as usize])
    }

    /// Returns a vector of pairs (cell index, value) for all filled cells in the grid.
    fn get_all_filled_cell_pairs(&self) -> Vec<(CellIndex, u8)> {
        self.grid
            .iter_all()
            .enumerate()
            .filter_map(|(i, &val)| val.map(|v| (i as u8, v)))
            .collect()
    }

    // Collect empty neighbors in the same row, column, and box as the given coordinates
    fn collect_empty_neighbors_for(&self, coords: CellCoords) -> ArrayVec<CellIndex, GROUP_SIZE> {
        let (cell_row, cell_col, cell_box) = coords;
        let mut out: ArrayVec<CellIndex, GROUP_SIZE> = ArrayVec::new();

        // Collect empty neighbors in the same row
        for col in 0..9 {
            if self.grid.get_by_row_col((cell_row, col)).is_none() {
                out.push(Self::get_cell_index((cell_row, col)));
            }
        }

        // Collect empty neighbors in the same column
        for row in 0..9 {
            if row != cell_row && self.grid.get_by_row_col((row, cell_col)).is_none() {
                out.push(Self::get_cell_index((row, cell_col)));
            }
        }

        // Collect empty neighbors in the same box
        let tl_box_row = (cell_box / 3) * 3;
        let tl_box_col = (cell_box % 3) * 3;
        for off in 0..9u8 {
            let row = tl_box_row + (off / 3);
            let col = tl_box_col + (off % 3);
            if row != cell_row && col != cell_col && self.grid.get_by_row_col((row, col)).is_none()
            {
                out.push(Self::get_cell_index((row, col)));
            }
        }
        out
    }

    /// Applies `num` to `coords`' neighbors:
    /// - Returns true if an immediate dead-end is detected (some neighbor becomes empty).
    /// - Otherwise, applies updates to neighbors that actually lose `num` and records undo entries.
    fn propagate_choice(
        &mut self,
        coords: CellCoords,
        val: u8,
        undo: &mut ArrayVec<(CellIndex, ElementSet), GROUP_SIZE>,
    ) -> bool {
        // Collect neighbors once
        let neighbors = self.collect_empty_neighbors_for(coords);
        let current_index = Self::get_cell_index((coords.0, coords.1));

        // First pass (single pass actually): detect immediate contradiction and gather updates
        let mut to_update: ArrayVec<(CellIndex, ElementSet), GROUP_SIZE> = ArrayVec::new();

        // for &ci in neighbors.iter() {
        //     if ci != current_index && self.grid.get_by_row_col(Self::get_row_col(ci)).is_none() {
        //         debug_assert!(
        //             self.empty_cell_queue
        //                 .get_priority_unsafe(ci as usize)
        //                 .is_some(),
        //             "Queue missing empty neighbor {}",
        //             ci
        //         );
        //     }
        // }

        // Attempt the value from the neighbor's possibilities
        for &ci in &neighbors {
            // Skip the current cell
            if ci == current_index {
                continue;
            }

            // Get the previous set of possible values for the empty neighbor
            let &old_set = self
                .empty_cell_queue
                .get_priority_unsafe(ci as usize)
                .unwrap();

            // Skip if the value is already excluded from the neighbor's possibilities
            if !old_set.has(val) {
                continue;
            }

            // If this neighbor only had `num`, removing it would make it empty => dead end
            if old_set.len() == 1 {
                return true;
            }

            // Otherwise, we plan to remove `num` and record the value for undo
            to_update.push((ci, old_set));
        }

        // Safe to apply updates now. Record undo entries and update the queue.
        undo.clear();
        for &(ci, old_set) in &to_update {
            let mut new_set = old_set;
            new_set.remove(val);
            undo.push((ci, old_set));
            self.empty_cell_queue.insert_unsafe((ci as usize, new_set));
        }

        // Return false to indicate that the a dead end was not found
        false
    }

    /// Fills the board with random values, ensuring that each row, column, and box contains all
    /// numbers from 1 to 9.
    pub fn fill_from_rng<T: Rng>(&mut self, mut rng: &mut T) {
        #[derive(Clone)]
        struct GenFrame {
            cell_index: CellIndex,
            possibilities: ElementSet, // remaining values for this cell (untried)
            undo: ArrayVec<(CellIndex, ElementSet), GROUP_SIZE>, // (neighbor_index, old_set) for changed neighbors
        }

        // List of cells used to initialize unfilled cell heap
        let mut all_cell_indexes: ArrayVec<CellIndex, BOARD_SIZE> = ArrayVec::new();
        for i in 0..BOARD_SIZE {
            all_cell_indexes.push(i as u8);
        }

        // Shuffle the cells so that they are randomly ordered
        all_cell_indexes.shuffle(&mut rng);

        // Initialize a priority queue to get the next cell with the fewest possibilities
        self.empty_cell_queue
            .fill_from_iter_unsafe(all_cell_indexes.iter().map(|&cell_index| {
                (
                    cell_index as usize,
                    ElementSet::from(1..=9), // Represents all possible values for the cell
                )
            }));

        // Stack of cells that have already been filled (with their remaining possibilities and undo)
        let mut stack: ArrayVec<GenFrame, BOARD_SIZE> = ArrayVec::new();

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

                // Update the possibilities left in the heap for each of the empty cells neighboring
                // the current cell, recording undo info only for neighbors that change.
                let mut undo: ArrayVec<(CellIndex, ElementSet), GROUP_SIZE> = ArrayVec::new();
                let dead_end = self.propagate_choice(current_cell_coords, num, &mut undo);

                if dead_end {
                    // No neighbor updates were applied; just revert the cell and try next number
                    self.delete(current_cell_coords);
                    // IMPORTANT: reinsert the current (now-empty) cell with its remaining possibilities
                    self.empty_cell_queue
                        .insert_unsafe((current_cell_index as usize, current_possibilities));
                    continue;
                }

                // Push this decision frame onto the stack
                stack.push(GenFrame {
                    cell_index: current_cell_index,
                    possibilities: current_possibilities,
                    undo,
                });
            } else {
                // Dead-end for the current cell; backtrack to the last filled cell (if any)
                let GenFrame {
                    cell_index: previous_cell,
                    possibilities: previous_cell_possibilities,
                    undo,
                } = stack.pop().unwrap();

                let previous_cell_coords = Self::get_cell_coords(previous_cell);

                // Remove the filled number from the board first so restored sets are valid
                self.delete(previous_cell_coords);

                // Restore the possibilities for the neighbors we changed when we set the previous cell
                for &(ci, old_set) in undo.iter().rev() {
                    self.empty_cell_queue.insert_unsafe((ci as usize, old_set));
                }

                // Reset the possibilities for the current cell (it stays empty)
                let current_cell_possibilities = self.get_element_set(current_cell_coords);
                self.empty_cell_queue
                    .insert_unsafe((current_cell_index as usize, current_cell_possibilities));

                // Add the last filled cell back to the queue so that a different possibility can be tried
                self.empty_cell_queue
                    .insert_unsafe((previous_cell as usize, previous_cell_possibilities));
            }
        }
    }

    /// Returns the candidate values for `coords` ordered by LCV (least-constraining first).
    /// The current cell is skipped. Neighbor sets are prefetched once. Short circuits when there
    /// are 2 or fewer candidates.
    fn order_values_lcv(&self, coords: CellCoords, candidates: ElementSet) -> ArrayVec<u8, 9> {
        // Fast path: tiny domains don't benefit from LCV sorting
        if candidates.len() <= 2 {
            let mut out: ArrayVec<u8, 9> = ArrayVec::new();
            for v in &candidates {
                out.push(v);
            }
            // Deterministic order
            out.sort_unstable();
            return out;
        }

        // Collect the empty neighbors once and prefetch their ElementSets
        let neighbors = self.collect_empty_neighbors_for(coords);
        let current_index = Self::get_cell_index((coords.0, coords.1));

        let mut neigh_sets: ArrayVec<ElementSet, GROUP_SIZE> = ArrayVec::new();
        let mut neigh_idx: ArrayVec<CellIndex, GROUP_SIZE> = ArrayVec::new();

        for &ci in &neighbors {
            if ci == current_index {
                continue; // popped MRV cell isn't in the queue
            }
            if let Some(es) = self.empty_cell_queue.get_priority_unsafe(ci as usize) {
                neigh_idx.push(ci);
                neigh_sets.push(*es);
            }
        }

        // Score each candidate by how many neighbors would lose this value
        let mut scored: ArrayVec<(u8, u8), 9> = ArrayVec::new();
        for val in &candidates {
            let mut score: u8 = 0;
            for es in &neigh_sets {
                if es.has(val) {
                    score += 1;
                }
            }
            scored.push((val, score));
        }

        // Least-constraining first; tie-breaker by value for determinism
        scored.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

        let mut out: ArrayVec<u8, 9> = ArrayVec::new();
        for (v, _) in scored {
            out.push(v);
        }
        out
    }

    /// Visit all solutions recursively. Stops when the passed function returns false or all
    /// solutions have been visited.
    pub fn visit_solutions_recursive<F>(mut puzzle: ClassicPuzzle, mut visit: F)
    where
        F: FnMut(&ClassicGrid) -> bool,
    {
        fn dfs<F>(puzzle: &mut ClassicPuzzle, visit: &mut F) -> bool
        where
            F: FnMut(&ClassicGrid) -> bool,
        {
            // If solved, yield the solution and decide whether to continue
            if puzzle.empty_cell_queue.is_empty() {
                return visit(&puzzle.grid);
            }

            // Choose MRV cell
            let (cell_index, cell_possibilities) = puzzle.empty_cell_queue.pop().unwrap();
            let cell_coords = ClassicPuzzle::get_cell_coords(cell_index as u8);

            // Undo log for changed neighbors
            let mut undo: ArrayVec<(CellIndex, ElementSet), GROUP_SIZE> = ArrayVec::new();

            // LCV ordering for this MRV cell
            let ordered_vals = puzzle.order_values_lcv(cell_coords, cell_possibilities);

            // Try each value in LCV order
            for &num in &ordered_vals {
                // Set current cell
                puzzle.set(cell_coords, num);

                // Apply choice to neighbors (or detect contradiction early)
                let dead_end = puzzle.propagate_choice(cell_coords, num, &mut undo);

                let mut keep_going = true;
                if !dead_end {
                    // Recurse or yield
                    if puzzle.empty_cell_queue.is_empty() {
                        keep_going = visit(&puzzle.grid);
                    } else {
                        keep_going = dfs(puzzle, visit);
                    }

                    // Backtrack if necessary
                    for &(ci, old_set) in undo.iter().rev() {
                        puzzle
                            .empty_cell_queue
                            .insert_unsafe((ci as usize, old_set));
                    }
                }

                // Restore state and continue/stop
                puzzle.delete(cell_coords);

                if !keep_going {
                    // Reinset current cell before unwinding
                    let es = puzzle.get_element_set(cell_coords);
                    puzzle.empty_cell_queue.insert_unsafe((cell_index, es));
                    return false;
                }
            }

            // Reinsert the MRV cell (recomputed) on the way back up
            let es = puzzle.get_element_set(cell_coords);
            puzzle.empty_cell_queue.insert_unsafe((cell_index, es));
            true
        }

        dfs(&mut puzzle, &mut visit);
    }

    /// Find all solutions recursively.
    #[must_use]
    pub fn find_solutions_recursive(puzzle: ClassicPuzzle) -> Vec<ClassicGrid> {
        let mut sols = Vec::new();
        Self::visit_solutions_recursive(puzzle, |grid| {
            sols.push(*grid);
            true
        });
        sols
    }

    /// Count all solutions recursively.
    #[must_use]
    pub fn count_solutions_recursive(puzzle: ClassicPuzzle) -> usize {
        let mut count = 0;
        Self::visit_solutions_recursive(puzzle, |_| {
            count += 1;
            true
        });
        count
    }

    /// Find solutions up to a maximum count recursively.
    #[must_use]
    pub fn find_solutions_bounded_recursive(
        puzzle: ClassicPuzzle,
        max_count: usize,
    ) -> Vec<ClassicGrid> {
        let mut sols = Vec::new();
        if max_count == 0 {
            return sols;
        }
        let mut count = 0;
        Self::visit_solutions_recursive(puzzle, |grid| {
            sols.push(*grid);
            count += 1;
            count < max_count
        });
        sols
    }

    /// Count solutions up to a maximum count recursively.
    #[must_use]
    pub fn count_solutions_bounded_recursive(puzzle: ClassicPuzzle, max_count: usize) -> usize {
        let mut count = 0;
        if max_count == 0 {
            return count;
        }
        Self::visit_solutions_recursive(puzzle, |_| {
            count += 1;
            count < max_count
        });
        count
    }

    /// Visit solutions iteratively. Stops when the passed function returns false or when all
    /// solutions have been visited.
    pub fn visit_solutions_iterative<F>(mut puzzle: ClassicPuzzle, mut visit: F)
    where
        F: FnMut(&ClassicGrid) -> bool,
    {
        #[derive(Clone)]
        struct Frame {
            cell_index: CellIndex,
            order: ArrayVec<u8, 9>, // LCV-order values to try
            next_ix: u8,            // next index into `order` to try
            chosen: CellValue,      // currently chosen value (if any)
            undo: ArrayVec<(CellIndex, ElementSet), GROUP_SIZE>,
        }

        // Already solved
        if puzzle.empty_cell_queue.is_empty() {
            let _ = visit(&puzzle.grid);
            return;
        }

        // Initialize the stack with MRV cell
        let (first_index, first_poss) = puzzle.empty_cell_queue.pop().unwrap();
        let first_cell_coords = Self::get_cell_coords(first_index as u8);
        let mut stack: ArrayVec<Frame, BOARD_SIZE> = ArrayVec::new();
        stack.push(Frame {
            cell_index: first_index as u8,
            order: puzzle.order_values_lcv(first_cell_coords, first_poss),
            next_ix: 0,
            chosen: None,
            undo: ArrayVec::new(),
        });

        // Main loop
        'outer: while let Some(frame) = stack.last_mut() {
            let coords = Self::get_cell_coords(frame.cell_index);

            // If we had chosen a value previously at this depth, revert now
            if frame.chosen.is_some() {
                for &(ci, old_set) in frame.undo.iter().rev() {
                    puzzle
                        .empty_cell_queue
                        .insert_unsafe((ci as usize, old_set));
                }
                frame.undo.clear();
                puzzle.delete(coords);
                frame.chosen = None;
            }

            // Try next possibility at this depth
            if (frame.next_ix as usize) < frame.order.len() {
                let num = frame.order[frame.next_ix as usize];
                frame.next_ix += 1;

                puzzle.set(coords, num);
                frame.undo.clear();

                // Apply choice to neighbors (or detect contradiction early)
                let dead_end = puzzle.propagate_choice(coords, num, &mut frame.undo);

                // Propagate choice and check for dead end, undoing if necessary
                if dead_end {
                    // No updates were applied, so nothing to restore
                    puzzle.delete(coords);
                    continue;
                }

                // Record chosen value
                frame.chosen = Some(num);

                // Found a solution: yield and optionally stop
                if puzzle.empty_cell_queue.is_empty() {
                    if !visit(&puzzle.grid) {
                        break 'outer;
                    }
                    // leave state; loop will revert on next iteration
                    continue;
                }

                // Go deeper with next MRV
                let (next_index, next_poss) = puzzle.empty_cell_queue.pop().unwrap();
                let next_coords = Self::get_cell_coords(next_index as u8);
                stack.push(Frame {
                    cell_index: next_index as u8,
                    order: puzzle.order_values_lcv(next_coords, next_poss),
                    next_ix: 0,
                    chosen: None,
                    undo: ArrayVec::new(),
                });
            } else {
                // Exhausted this cell: reinsert it and backtrack
                let es = puzzle.get_element_set(coords);
                puzzle
                    .empty_cell_queue
                    .insert_unsafe((frame.cell_index as usize, es));
                stack.pop();
            }
        }
    }

    /// Find all solutions iteratively.
    #[must_use]
    pub fn find_solutions_iterative(puzzle: ClassicPuzzle) -> Vec<ClassicGrid> {
        let mut sols = Vec::new();
        Self::visit_solutions_iterative(puzzle, |grid| {
            sols.push(*grid);
            true
        });
        sols
    }

    /// Count all solutions iteratively.
    #[must_use]
    pub fn count_solutions_iterative(puzzle: ClassicPuzzle) -> usize {
        let mut count = 0;
        Self::visit_solutions_iterative(puzzle, |_| {
            count += 1;
            true
        });
        count
    }

    /// Find solutions up to a maximum count iteratively.
    #[must_use]
    pub fn find_solutions_bounded_iterative(
        puzzle: ClassicPuzzle,
        max_count: usize,
    ) -> Vec<ClassicGrid> {
        let mut sols = Vec::new();
        if max_count == 0 {
            return sols;
        }
        let mut count = 0;
        Self::visit_solutions_iterative(puzzle, |grid| {
            sols.push(*grid);
            count += 1;
            count < max_count
        });
        sols
    }

    /// Count solutions up to a maximum count iteratively.
    #[must_use]
    pub fn count_solutions_bounded_iterative(puzzle: ClassicPuzzle, max_count: usize) -> usize {
        let mut count = 0;
        if max_count == 0 {
            return count;
        }
        Self::visit_solutions_iterative(puzzle, |_| {
            count += 1;
            count < max_count
        });
        count
    }

    /// Checks if the puzzle has exactly one solution.
    fn is_well_posed(&self) -> bool {
        Self::count_solutions_bounded_recursive(self.clone(), 2) == 1
    }

    /// Clears cells from the puzzle until it has exactly one solution.
    pub fn minimize_from_rng<T: Rng>(&mut self, mut rng: &mut T) {
        // Create a list of pairs (cell index, value) for all filled cells in the grid
        let mut unattempted_filled_cell_pairs = self.get_all_filled_cell_pairs();

        // Shuffle the cells
        unattempted_filled_cell_pairs.shuffle(&mut rng);

        // Loop until there are no cells left to attempt
        while let Some((current_cell_index, cell_value)) = unattempted_filled_cell_pairs.pop() {
            let cell_coords = Self::get_cell_coords(current_cell_index);

            // Try to remove the value from this cell
            self.delete(cell_coords);

            // Make a clone of the cell queue to reset it later. It's efficient to just clone the
            // queue if it needs to be reset because it also keeps track of the neighbors in the
            // same group.
            let original_empty_cell_queue = self.empty_cell_queue.clone();

            // Add this cell to the empty cell queue and update the possibilities for all of the
            // empty cells in its group.
            let buf = self.collect_empty_neighbors_for(cell_coords);
            for ci in &buf {
                let coords = Self::get_cell_coords(*ci);
                let es = self.get_element_set(coords);
                self.empty_cell_queue.insert_unsafe((*ci as usize, es));
            }

            // If the board is not well-posed, then put the value back and reset the queue.
            if !self.is_well_posed() {
                // Put the value back if the puzzle is no longer well-posed
                self.set(cell_coords, cell_value);

                // Need to remove this cell from the queue and reset the possibilities for cells in
                // its group.
                self.empty_cell_queue = original_empty_cell_queue;
            }
        }
    }

    pub fn remove_n_random_filled_cells<T: Rng>(&mut self, rng: &mut T, n: usize) {
        let filled_cell_pairs = self.get_all_filled_cell_pairs();
        for _ in 0..n {
            let pair_index = rng.random_range(0..filled_cell_pairs.len());
            let (cell_index, _) = filled_cell_pairs[pair_index];
            let cell_coords = Self::get_cell_coords(cell_index);
            self.delete(cell_coords);
            let possibilities = self.get_element_set(cell_coords);
            self.empty_cell_queue
                .insert_unsafe((cell_index as usize, possibilities));
        }
    }

    /// Creates and sets up a puzzle given some string seed
    #[must_use]
    pub fn from_seed(seed: String) -> Self {
        let mut puzzle = ClassicPuzzle::new();

        let mut rng: SipRng = SipHasher::from(seed).into_rng();

        // Fill the board
        puzzle.fill_from_rng(&mut rng);

        // Remove numbers
        puzzle.minimize_from_rng(&mut rng);

        puzzle
    }

    #[must_use]
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
    /// Creates a new `ClassicPuzzle` from a `ClassicGrid`
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

        let mut empty_cell_queue = ArrayPriorityQueue::new();
        empty_cell_queue.init_map_none(BOARD_SIZE);
        empty_cell_queue.fill_from_iter_unsafe((0..BOARD_SIZE).filter_map(|cell_index| {
            let (row_index, col_index, box_index) = Self::get_cell_coords(cell_index as u8);
            if grid.get_by_row_col((row_index, col_index)).is_none() {
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
    /// Creates a new `ClassicPuzzle` from a grid string
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

    const SEED_PUZZLE_SOLUTION_STR: &str = indoc! {"
        5 6 2 | 1 4 3 | 9 8 7
        3 7 8 | 2 5 9 | 6 1 4
        9 4 1 | 7 8 6 | 3 2 5
        ------|-------|------
        4 8 6 | 5 1 2 | 7 3 9
        7 2 3 | 9 6 8 | 5 4 1
        1 5 9 | 4 3 7 | 8 6 2
        ------|-------|------
        2 3 4 | 8 7 5 | 1 9 6
        8 9 7 | 6 2 1 | 4 5 3
        6 1 5 | 3 9 4 | 2 7 8
    "};

    const SEED_PUZZLE_MINIMUM_STR: &str = indoc! {"
        . 6 . | 1 . . | . 8 7
        . . . | . . 9 | . . 4
        9 . . | 7 8 . | 3 . .
        ------|-------|------
        . . 6 | 5 . 2 | . . .
        . . . | 9 . 8 | . . .
        . . . | . 3 . | . . .
        ------|-------|------
        2 . . | . 7 . | . . 6
        8 9 7 | . 2 . | 4 . .
        . . 5 | . . . | . . 8
    "};

    /// A test puzzle with the absolute minimum number of clues (17). Taken from wikipedia.
    const HARD_PUZZLE_MINIMUM_STR: &str = indoc! {"
        . . . | . . . | . 1 .
        . . . | . . 2 | . . 3
        . . . | 4 . . | . . .
        ------|-------|------
        . . . | . . . | 5 . .
        4 . 1 | 6 . . | . . .
        . . 7 | 1 . . | . . .
        ------|-------|------
        . 5 . | . . . | 2 . .
        . . . | . 8 . | . 4 .
        . 3 . | 9 1 . | . . .
    "};

    /// The solution to the minimum puzzle above.
    const HARD_PUZZLE_SOLUTION_STR: &str = indoc! {"
        7 4 5 | 3 6 8 | 9 1 2
        8 1 9 | 5 7 2 | 4 6 3
        3 6 2 | 4 9 1 | 8 5 7
        ------|-------|------
        6 9 3 | 8 2 4 | 5 7 1
        4 2 1 | 6 5 7 | 3 9 8
        5 8 7 | 1 3 9 | 6 2 4
        ------|-------|------
        1 5 8 | 7 4 6 | 2 3 9
        9 7 6 | 2 8 3 | 1 4 5
        2 3 4 | 9 1 5 | 7 8 6
    "};

    /// Test that setting and deleting a value works correctly.
    #[test]
    fn set_and_delete() {
        let mut puzzle = ClassicPuzzle::new();
        puzzle.set((0, 0, 0), 1);
        assert_eq!(puzzle.grid.get_by_row_col((0, 0)), Some(1));
        puzzle.delete((0, 0, 0));
        assert_eq!(puzzle.grid.get_by_row_col((0, 0)), None);
    }

    /// Test that getting the cell index given a row and column works correctly.
    #[test]
    fn get_cell_index() {
        assert_eq!(ClassicPuzzle::get_cell_index((0, 0)), 0);
        assert_eq!(ClassicPuzzle::get_cell_index((8, 8)), 80);
    }

    /// Test that getting the row and column given a cell index works correctly.
    #[test]
    fn get_row_col() {
        assert_eq!(ClassicPuzzle::get_row_col(0), (0, 0));
        assert_eq!(ClassicPuzzle::get_row_col(80), (8, 8));
    }

    /// Test that getting the box index given a row and column works correctly.
    #[test]
    fn get_box_index() {
        assert_eq!(ClassicPuzzle::get_box_index((0, 0)), 0);
        assert_eq!(ClassicPuzzle::get_box_index((8, 8)), 8);
    }

    /// Test that getting the coordinate tuple of a cell index works correctly.
    #[test]
    fn get_cell_coords() {
        assert_eq!(ClassicPuzzle::get_cell_coords(0), (0, 0, 0));
        assert_eq!(ClassicPuzzle::get_cell_coords(80), (8, 8, 8));
    }

    /// Test that loading an empty puzzle from a grid sets the fields in the puzzle correctly.
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
        assert_eq!(puzzle.empty_cell_queue.len(), BOARD_SIZE);
    }

    /// Test that loading a filled puzzle from a string sets the fields in the puzzle correctly.
    #[test]
    fn from_grid_filled() {
        let grid = ClassicGrid::from(SEED_PUZZLE_SOLUTION_STR);
        let puzzle = ClassicPuzzle::from(grid);
        assert_eq!(puzzle.grid, grid);
        assert!(puzzle.row_sets.iter().all(|row_set| row_set.is_empty()));
        assert!(puzzle.col_sets.iter().all(|col_set| col_set.is_empty()));
        assert!(puzzle.box_sets.iter().all(|box_set| box_set.is_empty()));
        assert!(puzzle.empty_cell_queue.is_empty());
    }

    /// Test that loading a puzzle from a string sets the fields in the puzzle correctly.
    #[test]
    fn from_grid_minimum() {
        let grid = ClassicGrid::from(SEED_PUZZLE_MINIMUM_STR);
        let puzzle = ClassicPuzzle::from(grid);
        let expected_row_set_lens = [5, 7, 5, 6, 7, 8, 6, 4, 7];
        let expected_col_set_lens = [6, 7, 6, 5, 5, 6, 7, 8, 5];
        let expected_box_set_lens = [7, 5, 5, 8, 4, 9, 4, 7, 6];
        let check_sizes = |sets: &[ElementSet], expected_lens: &[u8], set_type: &str| {
            let set_lens: Vec<u8> = sets.iter().map(|set| set.len()).collect();
            assert_eq!(
                set_lens, expected_lens,
                "expected {} set lengths to be {:?} but they were {:?}",
                set_type, expected_lens, set_lens
            );
        };
        assert_eq!(puzzle.grid, grid);
        check_sizes(&puzzle.row_sets, &expected_row_set_lens, "row");
        check_sizes(&puzzle.col_sets, &expected_col_set_lens, "col");
        check_sizes(&puzzle.box_sets, &expected_box_set_lens, "box");
        assert_eq!(puzzle.empty_cell_queue.len(), 55);
    }

    /// Test that loading a puzzle from a string and back produces the original string.
    #[test]
    fn display() {
        let puzzle = ClassicPuzzle::from(SEED_PUZZLE_SOLUTION_STR);
        assert_eq!(puzzle.to_string(), SEED_PUZZLE_SOLUTION_STR);
    }

    /// Test that filling from RNG produces the expected puzzle for a given seed.
    #[test]
    fn fill_from_rng_determinism() {
        let mut rng: SipRng = SipHasher::from(SEED).into_rng();
        let mut puzzle = ClassicPuzzle::new();
        puzzle.fill_from_rng(&mut rng);
        let puzzle_str = puzzle.to_string();
        assert_eq!(
            puzzle_str,
            SEED_PUZZLE_SOLUTION_STR,
            "Generated puzzle\n{}should equal\n{}",
            puzzle_str.replace("\n", "    \n"),
            SEED_PUZZLE_SOLUTION_STR.replace("\n", "    \n")
        );
    }

    /// Test that filling from RNG produces a valid puzzle for many different seeds.
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

    /// Test that filling from RNG produces exactly one solution even when the puzzle is filled.
    #[test]
    fn find_solutions_filled_recursive() {
        let puzzle = ClassicPuzzle::from(HARD_PUZZLE_SOLUTION_STR);
        let solutions = ClassicPuzzle::find_solutions_recursive(puzzle.clone());
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0].to_string(), HARD_PUZZLE_SOLUTION_STR);
    }

    /// Test that filling from RNG produces exactly one solution when one cell is empty.
    #[test]
    fn find_solutions_one_missing_recursive() {
        let mut puzzle = ClassicPuzzle::from(HARD_PUZZLE_SOLUTION_STR);
        let cell_index = 7;
        let cell_coords = ClassicPuzzle::get_cell_coords(cell_index);
        puzzle.delete(cell_coords);
        let possibilities = puzzle.get_element_set(cell_coords);
        puzzle
            .empty_cell_queue
            .insert_unsafe((cell_index as usize, possibilities));
        let solutions = ClassicPuzzle::find_solutions_recursive(puzzle.clone());
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0].to_string(), HARD_PUZZLE_SOLUTION_STR);
    }

    /// Test that filling from RNG produces exactly one solution when one row is empty.
    #[test]
    fn find_solutions_row_missing_recursive() {
        let mut puzzle = ClassicPuzzle::from(HARD_PUZZLE_SOLUTION_STR);

        // Delete the entire first row (row = 0) with correct coords
        let mut row_cell_indexes = Vec::with_capacity(9);
        for col in 0..9 {
            let cell_index = ClassicPuzzle::get_cell_index((0, col));
            let cell_coords = ClassicPuzzle::get_cell_coords(cell_index);
            puzzle.delete(cell_coords);
            row_cell_indexes.push(cell_index);
        }

        // Insert the empty cells into the queue with up-to-date possibilities
        for cell_index in row_cell_indexes {
            let cell_coords = ClassicPuzzle::get_cell_coords(cell_index);
            let possibilities = puzzle.get_element_set(cell_coords);
            puzzle
                .empty_cell_queue
                .insert_unsafe((cell_index as usize, possibilities));
        }

        let solutions = ClassicPuzzle::find_solutions_recursive(puzzle.clone());
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0].to_string(), HARD_PUZZLE_SOLUTION_STR);
    }

    /// Test that filling from RNG produces exactly one solution when initialized with a minimum
    /// puzzle.
    #[test]
    fn find_solutions_minimum_recursive() {
        let puzzle = ClassicPuzzle::from(HARD_PUZZLE_MINIMUM_STR);
        let solutions = ClassicPuzzle::find_solutions_recursive(puzzle.clone());
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0].to_string(), HARD_PUZZLE_SOLUTION_STR);
    }

    /// Test that filling from RNG produces more than one solution when initialized with a minimum
    /// puzzle that has had one cell cleared.
    #[test]
    fn find_solutions_multiple_recursive() {
        let mut puzzle = ClassicPuzzle::from(HARD_PUZZLE_MINIMUM_STR);
        let cell_index = 7;
        let cell_coords = ClassicPuzzle::get_cell_coords(cell_index);
        puzzle.delete(cell_coords);
        let possibilities = puzzle.get_element_set(cell_coords);
        puzzle
            .empty_cell_queue
            .insert_unsafe((cell_index as usize, possibilities));
        let solutions = ClassicPuzzle::find_solutions_recursive(puzzle.clone());
        assert!(solutions.len() > 1);
    }

    /// Test that filling from RNG produces exactly one solution even when the puzzle is filled.
    #[test]
    fn find_solutions_filled_iterative() {
        let puzzle = ClassicPuzzle::from(HARD_PUZZLE_SOLUTION_STR);
        let solutions = ClassicPuzzle::find_solutions_iterative(puzzle.clone());
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0].to_string(), HARD_PUZZLE_SOLUTION_STR);
    }

    /// Test that filling from RNG produces exactly one solution when one cell is empty.
    #[test]
    fn find_solutions_one_missing_iterative() {
        let mut puzzle = ClassicPuzzle::from(HARD_PUZZLE_SOLUTION_STR);
        let cell_index = 7;
        let cell_coords = ClassicPuzzle::get_cell_coords(cell_index);
        puzzle.delete(cell_coords);
        let possibilities = puzzle.get_element_set(cell_coords);
        puzzle
            .empty_cell_queue
            .insert_unsafe((cell_index as usize, possibilities));
        let solutions = ClassicPuzzle::find_solutions_iterative(puzzle.clone());
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0].to_string(), HARD_PUZZLE_SOLUTION_STR);
    }

    /// Test that filling from RNG produces exactly one solution when one row is empty.
    #[test]
    fn find_solutions_row_missing_iterative() {
        let mut puzzle = ClassicPuzzle::from(HARD_PUZZLE_SOLUTION_STR);

        // Delete the entire first row (row = 0) with correct coords
        let mut row_cell_indexes = Vec::with_capacity(9);
        for col in 0..9 {
            let cell_index = ClassicPuzzle::get_cell_index((0, col));
            let cell_coords = ClassicPuzzle::get_cell_coords(cell_index);
            puzzle.delete(cell_coords);
            row_cell_indexes.push(cell_index);
        }

        // Insert the empty cells into the queue with up-to-date possibilities
        for cell_index in row_cell_indexes {
            let cell_coords = ClassicPuzzle::get_cell_coords(cell_index);
            let possibilities = puzzle.get_element_set(cell_coords);
            puzzle
                .empty_cell_queue
                .insert_unsafe((cell_index as usize, possibilities));
        }

        let solutions = ClassicPuzzle::find_solutions_iterative(puzzle.clone());
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0].to_string(), HARD_PUZZLE_SOLUTION_STR);
    }

    // FIXME - The iterative solution finding algorithm is not working correctly. After it's fixed,
    // benchmark it against the recursive solution finding algorithm.
    /// Test that filling from RNG produces exactly one solution when initialized with a minimum
    /// puzzle.
    #[test]
    fn find_solutions_minimum_iterative() {
        let puzzle = ClassicPuzzle::from(HARD_PUZZLE_MINIMUM_STR);
        let solutions = ClassicPuzzle::find_solutions_iterative(puzzle.clone());
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0].to_string(), HARD_PUZZLE_SOLUTION_STR);
    }

    /// Test that filling from RNG produces more than one solution when initialized with a minimum
    /// puzzle that has had one cell cleared.
    #[test]
    fn find_solutions_multiple_iterative() {
        let mut puzzle = ClassicPuzzle::from(HARD_PUZZLE_MINIMUM_STR);
        let cell_index = 7;
        let cell_coords = ClassicPuzzle::get_cell_coords(cell_index);
        puzzle.delete(cell_coords);
        let possibilities = puzzle.get_element_set(cell_coords);
        puzzle
            .empty_cell_queue
            .insert_unsafe((cell_index as usize, possibilities));
        let solutions = ClassicPuzzle::find_solutions_iterative(puzzle.clone());
        assert!(solutions.len() > 1);
    }

    /// Test that a puzzle with the minimum number of clues is well-posed.
    #[test]
    fn is_well_posed() {
        let puzzle = ClassicPuzzle::from(HARD_PUZZLE_MINIMUM_STR);
        assert!(puzzle.is_well_posed());
    }

    /// Test that a puzzle that has too few clues is not well-posed.
    #[test]
    fn is_not_well_posed() {
        let mut puzzle = ClassicPuzzle::from(HARD_PUZZLE_MINIMUM_STR);
        let cell_index = 7;
        let cell_coords = ClassicPuzzle::get_cell_coords(cell_index);
        puzzle.delete(cell_coords);
        let possibilities = puzzle.get_element_set(cell_coords);
        puzzle
            .empty_cell_queue
            .insert_unsafe((cell_index as usize, possibilities));
        assert!(!puzzle.is_well_posed());
    }

    /// Test that minimizing a puzzle from a random number generator produces the expected minimum
    /// puzzle.
    #[test]
    fn minimize_from_rng_determinism() {
        let mut puzzle = ClassicPuzzle::from(SEED_PUZZLE_SOLUTION_STR);
        let mut rng: SipRng = SipHasher::from(SEED).into_rng();
        puzzle.minimize_from_rng(&mut rng);
        let puzzle_str = puzzle.to_string();
        assert_eq!(
            puzzle_str,
            SEED_PUZZLE_MINIMUM_STR,
            "Generated puzzle\n{}should equal\n{}",
            puzzle_str.replace("\n", "    \n"),
            SEED_PUZZLE_MINIMUM_STR.replace("\n", "    \n")
        );
    }

    #[test]
    fn ill_posed_puzzle_has_more_than_one_solution_recursive() {
        let mut puzzle = ClassicPuzzle::from(HARD_PUZZLE_MINIMUM_STR);
        let mut rng: SipRng = SipHasher::from(SEED).into_rng();
        puzzle.remove_n_random_filled_cells(&mut rng, 1);
        let num_solutions = ClassicPuzzle::count_solutions_recursive(puzzle);
        assert!(num_solutions > 1);
    }

    #[test]
    fn ill_posed_puzzle_has_more_than_one_solution_iterative() {
        let mut puzzle = ClassicPuzzle::from(HARD_PUZZLE_MINIMUM_STR);
        let mut rng: SipRng = SipHasher::from(SEED).into_rng();
        puzzle.remove_n_random_filled_cells(&mut rng, 1);
        let num_solutions = ClassicPuzzle::count_solutions_iterative(puzzle);
        assert!(num_solutions > 1);
    }
}
