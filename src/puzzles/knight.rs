use indoc::indoc;

use super::PuzzleMeta;

#[derive(Default)]
pub struct KnightPuzzle {}

impl PuzzleMeta for KnightPuzzle {
    fn title() -> &'static str {
        "Knight"
    }

    fn description() -> &'static str {
        indoc! {"
            Classic rules plus no identical numbers can be a knight’s move apart, like in chess.
        "}
    }
}
