# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


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
