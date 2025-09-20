use std::time::Duration;

use divan::{bench, Bencher};
use rand::RngCore;
use rand::{seq::SliceRandom, Rng};
use rand_seeder::{SipHasher, SipRng};
use sudoku_machine::{
    puzzles::classic::{CellIndex, ClassicPuzzle},
    utility::seed::SeedRng,
};

fn main() {
    divan::main();
}

struct MyRng(fastrand::Rng);

impl RngCore for MyRng {
    fn next_u32(&mut self) -> u32 {
        self.0.u32(..)
    }

    fn next_u64(&mut self) -> u64 {
        self.0.u64(..)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill(dest)
    }
}

impl MyRng {
    fn with_seed(seed: u64) -> Self {
        MyRng(fastrand::Rng::with_seed(seed))
    }
}

fn create_random_sip_rng() -> SipRng {
    let seed = rand::rng().gen_seed();
    SipHasher::from(seed).into_rng()
}

#[bench(min_time=Duration::from_secs(10))]
fn fill_from_siprng(bencher: Bencher) {
    bencher
        .with_inputs(|| (ClassicPuzzle::new(), create_random_sip_rng()))
        .bench_values(|(mut puzzle, mut rng)| {
            puzzle.fill_from_rng(&mut rng);
        });
}

fn create_random_my_rng() -> MyRng {
    let seed = rand::rng().random();
    MyRng::with_seed(seed)
}

#[bench(min_time=Duration::from_secs(10))]
fn fill_from_myrng(bencher: Bencher) {
    bencher
        .with_inputs(|| (ClassicPuzzle::new(), create_random_my_rng()))
        .bench_values(|(mut puzzle, mut rng)| {
            puzzle.fill_from_rng(&mut rng);
        });
}

fn create_random_puzzle() -> ClassicPuzzle {
    let seed = rand::rng().gen_seed();
    ClassicPuzzle::from_seed(seed)
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_0_removed_iterative(bencher: Bencher) {
    bencher
        .with_inputs(|| create_random_puzzle())
        .bench_values(|puzzle| {
            ClassicPuzzle::find_solutions_iterative(puzzle);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_0_removed_recursive(bencher: Bencher) {
    bencher
        .with_inputs(|| create_random_puzzle())
        .bench_values(|puzzle| {
            ClassicPuzzle::find_solutions_recursive(puzzle);
        });
}

fn create_random_vec_of_cell_indexes() -> Vec<CellIndex> {
    let mut rng = create_random_my_rng();
    let mut all_cell_indexes: Vec<CellIndex> = (0..81).collect();
    all_cell_indexes.shuffle(&mut rng);
    all_cell_indexes
}

fn find_n_filled_cells(puzzle: &ClassicPuzzle, n: usize) -> Vec<CellIndex> {
    let cell_indexes = create_random_vec_of_cell_indexes();
    cell_indexes
        .into_iter()
        .filter(move |cell_index| {
            let cell_coords = ClassicPuzzle::get_cell_coords(*cell_index);
            puzzle.grid.get((cell_coords.0, cell_coords.1)).is_some()
        })
        .take(n)
        .collect()
}

fn remove_n_filled_cells(mut puzzle: ClassicPuzzle, n: usize) -> ClassicPuzzle {
    let cells_to_remove = find_n_filled_cells(&puzzle, n)
        .into_iter()
        .take(n)
        .collect::<Vec<_>>();
    for cell_index in cells_to_remove {
        let cell_coords = ClassicPuzzle::get_cell_coords(cell_index);
        puzzle.delete(cell_coords);
    }
    puzzle
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_1_removed_iterative(bencher: Bencher) {
    bencher
        .with_inputs(|| remove_n_filled_cells(create_random_puzzle(), 1))
        .bench_values(|puzzle| {
            ClassicPuzzle::find_solutions_iterative(puzzle);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_1_removed_recursive(bencher: Bencher) {
    bencher
        .with_inputs(|| remove_n_filled_cells(create_random_puzzle(), 1))
        .bench_values(|puzzle| {
            ClassicPuzzle::find_solutions_recursive(puzzle);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_2_removed_iterative(bencher: Bencher) {
    bencher
        .with_inputs(|| remove_n_filled_cells(create_random_puzzle(), 2))
        .bench_values(|puzzle| {
            ClassicPuzzle::find_solutions_iterative(puzzle);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_2_removed_recursive(bencher: Bencher) {
    bencher
        .with_inputs(|| remove_n_filled_cells(create_random_puzzle(), 2))
        .bench_values(|puzzle| {
            ClassicPuzzle::find_solutions_recursive(puzzle);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_4_removed_iterative(bencher: Bencher) {
    bencher
        .with_inputs(|| remove_n_filled_cells(create_random_puzzle(), 4))
        .bench_values(|puzzle| {
            ClassicPuzzle::find_solutions_iterative(puzzle);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_4_removed_recursive(bencher: Bencher) {
    bencher
        .with_inputs(|| remove_n_filled_cells(create_random_puzzle(), 4))
        .bench_values(|puzzle| {
            ClassicPuzzle::find_solutions_recursive(puzzle);
        });
}
