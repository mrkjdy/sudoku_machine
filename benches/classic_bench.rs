use std::time::Duration;

use divan::{bench, Bencher};
use rand::Rng;
use rand::RngCore;
use rand_seeder::{SipHasher, SipRng};
use sudoku_machine::{puzzles::classic::ClassicPuzzle, utility::seed::SeedRng};

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

fn create_random_my_rng() -> MyRng {
    let seed = rand::rng().random();
    MyRng::with_seed(seed)
}

fn create_random_puzzle(rng: &mut MyRng) -> ClassicPuzzle {
    let seed = rng.gen_seed();
    ClassicPuzzle::from_seed(seed)
}

#[bench(min_time = Duration::from_secs(10))]
fn count_solutions_4_removed_recursive(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = create_random_my_rng();
            let mut puzzle = create_random_puzzle(&mut rng);
            puzzle.remove_n_random_filled_cells(&mut rng, 4);
            puzzle
        })
        .bench_values(|puzzle| {
            let _ = ClassicPuzzle::count_solutions_recursive(puzzle);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn count_solutions_4_removed_iterative(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = create_random_my_rng();
            let mut puzzle = create_random_puzzle(&mut rng);
            puzzle.remove_n_random_filled_cells(&mut rng, 4);
            puzzle
        })
        .bench_values(|puzzle| {
            let _ = ClassicPuzzle::count_solutions_iterative(puzzle);
        });
}

#[bench(min_time=Duration::from_secs(10))]
fn fill_from_siprng(bencher: Bencher) {
    bencher
        .with_inputs(|| (ClassicPuzzle::new(), create_random_sip_rng()))
        .bench_values(|(mut puzzle, mut rng)| {
            puzzle.fill_from_rng(&mut rng);
        });
}

#[bench(min_time=Duration::from_secs(10))]
fn fill_from_myrng(bencher: Bencher) {
    bencher
        .with_inputs(|| (ClassicPuzzle::new(), create_random_my_rng()))
        .bench_values(|(mut puzzle, mut rng)| {
            puzzle.fill_from_rng(&mut rng);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_0_removed_iterative(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = create_random_my_rng();
            create_random_puzzle(&mut rng)
        })
        .bench_values(|puzzle| {
            let _ = ClassicPuzzle::find_solutions_iterative(puzzle);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_0_removed_recursive(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = create_random_my_rng();
            create_random_puzzle(&mut rng)
        })
        .bench_values(|puzzle| {
            let _ = ClassicPuzzle::find_solutions_recursive(puzzle);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_1_removed_iterative(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = create_random_my_rng();
            let mut puzzle = create_random_puzzle(&mut rng);
            puzzle.remove_n_random_filled_cells(&mut rng, 1);
            puzzle
        })
        .bench_values(|puzzle| {
            let _ = ClassicPuzzle::find_solutions_iterative(puzzle);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_1_removed_recursive(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = create_random_my_rng();
            let mut puzzle = create_random_puzzle(&mut rng);
            puzzle.remove_n_random_filled_cells(&mut rng, 1);
            puzzle
        })
        .bench_values(|puzzle| {
            let _ = ClassicPuzzle::find_solutions_recursive(puzzle);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_2_removed_iterative(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = create_random_my_rng();
            let mut puzzle = create_random_puzzle(&mut rng);
            puzzle.remove_n_random_filled_cells(&mut rng, 2);
            puzzle
        })
        .bench_values(|puzzle| {
            let _ = ClassicPuzzle::find_solutions_iterative(puzzle);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_2_removed_recursive(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = create_random_my_rng();
            let mut puzzle = create_random_puzzle(&mut rng);
            puzzle.remove_n_random_filled_cells(&mut rng, 2);
            puzzle
        })
        .bench_values(|puzzle| {
            let _ = ClassicPuzzle::find_solutions_recursive(puzzle);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_4_removed_iterative(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = create_random_my_rng();
            let mut puzzle = create_random_puzzle(&mut rng);
            puzzle.remove_n_random_filled_cells(&mut rng, 4);
            puzzle
        })
        .bench_values(|puzzle| {
            let _ = ClassicPuzzle::find_solutions_iterative(puzzle);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn find_solutions_4_removed_recursive(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = create_random_my_rng();
            let mut puzzle = create_random_puzzle(&mut rng);
            puzzle.remove_n_random_filled_cells(&mut rng, 4);
            puzzle
        })
        .bench_values(|puzzle| {
            let _ = ClassicPuzzle::find_solutions_recursive(puzzle);
        });
}

#[bench(min_time = Duration::from_secs(10))]
fn from_seed_recursive(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = create_random_my_rng();
            let seed = rng.gen_seed();
            seed
        })
        .bench_values(|seed| {
            let _ = ClassicPuzzle::from_seed(seed);
        });
}
