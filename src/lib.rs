pub mod plugins;
pub mod puzzles;
pub mod utility;

pub const APP_TITLE: &str = "Sudoku Machine";

#[cfg(all(test, target_family = "wasm"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
