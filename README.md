# Sudoku Machine

Sudoku Machine is built with [Rust](https://www.rust-lang.org/) and
[Bevy](https://bevyengine.org/). It will be a game that allows you to generate
and solve different Sudoku variants including the classic 9x9 puzzle, Knight,
and Full Kropki. With more to come!

I'm building this game as a way to learn Rust and Bevy, and to have a fun
project to work on in my free time. I also plan to build a web version of the
game using WebAssembly/WebGPU and make it available at https://mrkjdy.dev.

## TODO

- CI/CD
- Implement a SpawnWidget trait for spawning widgets instead of the Spawn trait
- UI for the classic puzzle
- Puzzle generators for Knight and Full Kropki
- UIs for the Knight and Full Kropki puzzles
- The Continue button
- The History menu
- SVG rendering for buttons
- Make available at https://mrkjdy.dev
- Additional puzzle types - Hexadoku, Diagonal, etc.

### License Information

All source code in this repository is licensed under the **MIT License**. See
the [LICENSE.txt](LICENSE.txt) file for details.

#### Fonts

The fonts located in `assets/fonts/` are licensed under the **SIL Open Font
License (OFL)**. For more information, see
[`assets/fonts/LICENSE-OpenFont.txt`](assets/fonts/LICENSE-OpenFont.txt)

#### Icons

The icons located in `assets/icons/` are part of **Heroicons**, which are
licensed under the **MIT License**. For more information, see
[`assets/icons/LICENSE-Heroicons.txt`](assets/icons/LICENSE-Heroicons.txt).
