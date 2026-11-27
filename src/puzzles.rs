use classic::puzzle::ClassicPuzzle;
#[cfg(debug_assertions)]
use full_kropki::FullKropkiPuzzle;
#[cfg(debug_assertions)]
use knight::KnightPuzzle;
use num_enum::TryFromPrimitive;
use strum_macros::EnumIter;

pub mod classic;
pub mod full_kropki;
pub mod knight;

pub type CellCoords = (u8, u8, u8);
pub type CellIndex = u8;
pub type CellValue = Option<u8>;

pub type Row<const NUM_COLS: usize> = [CellValue; NUM_COLS];

pub type Grid<const NUM_COLS: usize, const NUM_ROWS: usize> = [Row<NUM_COLS>; NUM_ROWS];

pub trait PuzzleMeta {
    fn title() -> &'static str;
    fn description() -> &'static str;
}

#[derive(Default, EnumIter, TryFromPrimitive, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum PuzzleType {
    #[default]
    Classic,
    #[cfg(debug_assertions)]
    FullKropki,
    #[cfg(debug_assertions)]
    Knight,
}

impl PuzzleType {
    #[must_use]
    pub fn title(&self) -> &'static str {
        match self {
            PuzzleType::Classic => ClassicPuzzle::title(),
            #[cfg(debug_assertions)]
            PuzzleType::FullKropki => FullKropkiPuzzle::title(),
            #[cfg(debug_assertions)]
            PuzzleType::Knight => KnightPuzzle::title(),
        }
    }

    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            PuzzleType::Classic => ClassicPuzzle::description(),
            #[cfg(debug_assertions)]
            PuzzleType::Knight => KnightPuzzle::description(),
            #[cfg(debug_assertions)]
            PuzzleType::FullKropki => FullKropkiPuzzle::description(),
        }
    }

    // pub fn bundle_from_seed(&self, seed: &str) -> Box<dyn Bundle> {
    //     match self {
    //         PuzzleType::Classic => Box::new(ClassicPuzzle::bundle_from_seed(seed)),
    //         PuzzleType::Knight => Box::new(KnightPuzzle::bundle_from_seed(seed)),
    //         PuzzleType::FullKropki => Box::new(FullKropkiPuzzle::bundle_from_seed(seed)),
    //     }
    // }
}
