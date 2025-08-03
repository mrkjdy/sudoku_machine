# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.2.0](https://github.com/mrkjdy/sudoku_machine/compare/v0.1.8...v0.2.0) - 2025-08-03

### Added

- use symbols and font for nav button ([#67](https://github.com/mrkjdy/sudoku_machine/pull/67))

### Fixed

- make fps counter actually readable ([#83](https://github.com/mrkjdy/sudoku_machine/pull/83))
- couple issues after 0.16 upgrade ([#65](https://github.com/mrkjdy/sudoku_machine/pull/65))
- disable BorderlessFullscreen on wasm ([#49](https://github.com/mrkjdy/sudoku_machine/pull/49))

### Other

- *(deps)* bump rand from 0.9.1 to 0.9.2 ([#82](https://github.com/mrkjdy/sudoku_machine/pull/82))
- *(deps)* bump strum_macros from 0.27.1 to 0.27.2 ([#81](https://github.com/mrkjdy/sudoku_machine/pull/81))
- *(deps)* bump strum from 0.27.1 to 0.27.2 ([#80](https://github.com/mrkjdy/sudoku_machine/pull/80))
- bump arboard to 3.6.0 ([#84](https://github.com/mrkjdy/sudoku_machine/pull/84))
- *(deps)* bump num_enum from 0.7.3 to 0.7.4 ([#78](https://github.com/mrkjdy/sudoku_machine/pull/78))
- create issues for TODOs ([#68](https://github.com/mrkjdy/sudoku_machine/pull/68))
- *(deps)* bump bevy from 0.16.0 to 0.16.1 ([#77](https://github.com/mrkjdy/sudoku_machine/pull/77))
- split up themed ([#66](https://github.com/mrkjdy/sudoku_machine/pull/66))
- *(deps)* bump getrandom from 0.3.2 to 0.3.3 ([#64](https://github.com/mrkjdy/sudoku_machine/pull/64))
- upgrade to bevy 0.16 ([#63](https://github.com/mrkjdy/sudoku_machine/pull/63))
- *(deps)* bump rand from 0.9.0 to 0.9.1 ([#60](https://github.com/mrkjdy/sudoku_machine/pull/60))
- add CODEOWNERS ([#59](https://github.com/mrkjdy/sudoku_machine/pull/59))
- *(deps)* bump divan from 0.1.17 to 0.1.18 ([#54](https://github.com/mrkjdy/sudoku_machine/pull/54))
- *(deps)* bump arboard from 3.4.1 to 3.5.0 ([#53](https://github.com/mrkjdy/sudoku_machine/pull/53))
- *(deps)* bump getrandom from 0.3.1 to 0.3.2 ([#52](https://github.com/mrkjdy/sudoku_machine/pull/52))
- *(deps)* bump crossbeam-channel from 0.5.14 to 0.5.15 ([#55](https://github.com/mrkjdy/sudoku_machine/pull/55))
- only cargo build on non-release events ([#58](https://github.com/mrkjdy/sudoku_machine/pull/58))
- don't install code signing cert on dependabot ([#56](https://github.com/mrkjdy/sudoku_machine/pull/56))
- cargo update ([#50](https://github.com/mrkjdy/sudoku_machine/pull/50))

## [0.1.8](https://github.com/mrkjdy/sudoku_machine/compare/v0.1.7...v0.1.8) - 2025-02-28

### Fixed

- macOS code signing (#45)
- use correct path for wasm import in index.html (#43)

### Other

- add job to deploy to GitHub pages (#47)
- setup fps counter (#46)

## [0.1.7](https://github.com/mrkjdy/sudoku_machine/compare/v0.1.6...v0.1.7) - 2025-02-21

### Fixed

- wasm building and clipboard (#40)

### Other

- build binaries on all PRs and remove macOS env vars (#42)
- build binaries on release PRs (#41)
- *(deps)* bump strum_macros from 0.27.0 to 0.27.1 (#38)
- *(deps)* bump strum from 0.27.0 to 0.27.1 (#37)

## [0.1.6](https://github.com/mrkjdy/sudoku_machine/compare/v0.1.5...v0.1.6) - 2025-02-17

### Other

- upgrade actions/upload-artifact (#35)

## [0.1.5](https://github.com/mrkjdy/sudoku_machine/compare/v0.1.4...v0.1.5) - 2025-02-17

### Other

- explicitly set release-plz release outputs so that dependent jobs can use them (#33)
- only enable bevy dynamic_linking with d aliases (#32)

## [0.1.4](https://github.com/mrkjdy/sudoku_machine/compare/v0.1.3...v0.1.4) - 2025-02-17

### Other

- use toJSON to stringify release plz release output (#30)

## [0.1.3](https://github.com/mrkjdy/sudoku_machine/compare/v0.1.2...v0.1.3) - 2025-02-17

### Other

- use PAT instead of default GITHUB_TOKEN to trigger other workflows (#29)
- print release-plz release output (#27)

## [0.1.2](https://github.com/mrkjdy/sudoku_machine/compare/v0.1.1...v0.1.2) - 2025-02-16

### Other

- use releases_created as boolean rather than string (#25)

## [0.1.1](https://github.com/mrkjdy/sudoku_machine/compare/v0.1.0...v0.1.1) - 2025-02-16

### Other

- build binaries in workflow after successful release-plz release and fix changelog header (#23)

## [0.1.0](https://github.com/mrkjdy/sudoku_machine/releases/tag/v0.1.0) - 2025-02-16

### Added

- create a new puzzle screen and code to generate classic puzzles

### Fixed

- use correct path for LICENSE.txt in README.md
- use correct path for Heroicons license
- correct PartialOrd for ElementSet

### Other

- add system dependencies to release-plz workflow (#22)
- release v0.1.0 (#19)
- release v0.1.0 (#16)
- add repository and author to Cargo.toml (#17)
- release v0.1.0 (#13)
- add bench to ci (#14)
- release v0.1.0 (#10)
- fix CI / CD (#11)
- Revert "chore: release v0.1.1"
- fix errors in benches/
- update test data after upgrading rand crates
- update dependencies
- Merge branch 'main' into dependabot/cargo/strum-0.27.0
- configure description and license in Cargo.toml
- configure publishing to crates.io
- use correct trigger for binary release and name for release-plz workflows
- release v0.1.0
- configure ci and release workflows
- create dependabot.yml
- use indoc for puzzle descriptions
- create README.md and licenses
- address clippy issues
- remove commented code
- use structs with default instead of builder pattern
- rename package to sudoku_machine
- upgrade to bevy 0.15
- setup bevy
- create new project with cargo
