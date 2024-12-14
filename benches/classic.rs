use std::time::Duration;

use bevy_sudoku::{puzzles::classic::ClassicPuzzle, utility::seed::SeedRng};
use divan::{bench, Bencher};
use rand::{Rng, RngCore};
use rand_seeder::SipHasher;

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

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.0.fill(dest);
        Ok(())
    }
}

impl MyRng {
    fn with_seed(seed: u64) -> Self {
        MyRng(fastrand::Rng::with_seed(seed))
    }
}

#[bench(min_time=Duration::from_secs(10))]
fn fill_from_siprng(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let seed = rand::thread_rng().gen_seed();
            let rng = SipHasher::from(seed).into_rng();
            let puzzle = ClassicPuzzle::new();
            (puzzle, rng)
        })
        .bench_values(|(mut puzzle, mut rng)| {
            puzzle.fill_from_rng(&mut rng);
        });
}

#[bench(min_time=Duration::from_secs(10))]
fn fill_from_myrng(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let seed = rand::thread_rng().gen();
            let rng = MyRng::with_seed(seed);
            let puzzle = ClassicPuzzle::new();
            (puzzle, rng)
        })
        .bench_values(|(mut puzzle, mut rng)| {
            puzzle.fill_from_rng(&mut rng);
        });
}

#[bench(min_time=Duration::from_secs(600))]
fn minimum_clues(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let seed = rand::thread_rng().gen();
            let mut rng = MyRng::with_seed(seed);
            let mut puzzle = ClassicPuzzle::new();
            puzzle.fill_from_rng(&mut rng);
            (puzzle, rng)
        })
        .bench_values(|(mut puzzle, mut rng)| {
            puzzle.remove_from_rng(&mut rng);
            let num_clues = puzzle.num_clues();
            if num_clues < 21 {
                println!("Found puzzle with {} clues", num_clues);
                println!("{}", puzzle.to_string());
            }
        });
}
