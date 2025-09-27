use indoc::indoc;

use super::PuzzleMeta;

#[derive(Default)]
pub struct FullKropkiPuzzle {}

impl PuzzleMeta for FullKropkiPuzzle {
    fn title() -> &'static str {
        "Full Kropki Sudoku"
    }

    fn description() -> &'static str {
        indoc! {"
            Classic rules plus Kropki dot rules for adjacent cells:
             • a black dot means one of the numbers is twice the value of the other
           	 • a white dot means the numbers are consecutive
        "}
    }
}
