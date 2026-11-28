# Sudoku Machine

Sudoku Machine is built with [Rust](https://www.rust-lang.org/) and
[Bevy](https://bevyengine.org/). It will be a game that allows you to generate
and solve different Sudoku variants including the classic 9x9 puzzle, Knight,
and Full Kropki. With more to come!

I'm building this game as a way to learn Rust and Bevy, and to have a fun
project to work on in my free time. I also plan to build a web version of the
game using WebAssembly/WebGPU and make it available at https://mrkjdy.dev.

## TODO

See [Issues](https://github.com/mrkjdy/sudoku_machine/issues)

## Development

See [.cargo/config.toml](.cargo/config.toml) for a list of useful aliases!

### Running Sudoku Machine

On Debian or Ubuntu, install `libasound2-dev`, `libudev-dev`, and
`libwayland-dev` before building the native app.

There are several ways to run the game locally:

| Command                                                | Description                                                                                                                                                 |
| ------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cargo run`                                            | Run the game in debug mode.                                                                                                                                 |
| `cargo run --release`                                  | Run the game in release mode.                                                                                                                               |
| `cargo rund`                                           | Run the game with Bevy's dynamic linking enabled. See the [docs](https://bevy.org/learn/quick-start/getting-started/setup/#dynamic-linking).                |
| `cargo runw`[<sup>\*</sup>](#note-about-running-wasm)  | Run the game for the wasm32-unknown-unknown target. Uses WebGL2.                                                                                            |
| `cargo runww`[<sup>\*</sup>](#note-about-running-wasm) | Run the game for the wasm32-unknown-unknown target with Bevy's WebGPU feature enabled. See the [docs](https://docs.rs/bevy/latest/bevy/#optional-features). |

If `cargo rund` cannot resolve `bevy_dylib` after a Bevy upgrade, run
`cargo info bevy_dylib@<locked Bevy version>` to refresh Cargo's registry
entry, then retry. Use the `bevy` version in [Cargo.lock](Cargo.lock).

#### Note about running Wasm

These commands use [`wasm-bindgen-cli`](https://github.com/wasm-bindgen/wasm-bindgen) and [`wasm-server-runner`](https://github.com/jakobhellermann/wasm-server-runner). You can install them with:

```bash
cargo install -f wasm-bindgen-cli@<wasm-bindgen version> wasm-server-runner
```

Check the version of `wasm-bindgen` installed in [Cargo.lock](Cargo.lock) to determine which version of `wasm-bindgen-cli` to install. It's important to reinstall `wasm-server-runner` when updating `wasm-bindgen-cli` because `wasm-server-runner` builds using your installed version of `wasm-bindgen`.

### Browser tests for Wasm

Install Chrome and a matching [ChromeDriver](https://developer.chrome.com/docs/chromedriver/downloads).
Put ChromeDriver on your PATH or set `CHROMEDRIVER` to its executable.
Install the `wasm-bindgen-test-runner` from the same version of
`wasm-bindgen-cli` as the `wasm-bindgen` crate in `Cargo.lock`.

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked --bin wasm-bindgen-test-runner --version 0.2.108 wasm-bindgen-cli
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
  WASM_BINDGEN_USE_BROWSER=1 \
  cargo test --locked --lib --profile ci --target wasm32-unknown-unknown --features bevy/webgpu
```

The clipboard test runs in headless Chrome with a simulated clipboard. It checks
that a rejected write stays queued and is retried on the next pointer release.

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
