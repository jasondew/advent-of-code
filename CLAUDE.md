# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is an Advent of Code solutions repository containing solutions from 2015-2024 in multiple programming languages. Each year has its own directory with a consistent structure.

### Language by Year
- **2015-2024 (primary)**: Rust solutions in `YEAR/rust/` subdirectory
- **2016**: Elm solutions (numbered directories 01-16)
- **2017**: Elm, Elixir, and Ruby solutions in separate subdirectories
- **2019-2020**: Elixir and Rust solutions
- **2024**: Most recent, actively developed in Rust

### Input Files
Puzzle inputs are stored in `YEAR/inputs/` with files named `01`, `02`, etc. (no extension).

## Rust Project Structure (2015-2024)

The Rust implementations follow a consistent architecture across years:

```
YEAR/rust/
├── src/
│   ├── lib.rs           # Module exports for day01-day25
│   ├── dayXX.rs         # Solution logic with part1() and part2() functions
│   └── bin/
│       └── dayXX.rs     # Executable that reads input and calls part1/part2
├── benches/
│   └── days.rs          # Criterion benchmarks for all days
├── Cargo.toml           # Dependencies: regex, priority-queue, itertools
├── Makefile             # init/DAY target for new day setup
└── template.rs          # Template for new day solutions
```

### Module Pattern
Each day follows this structure:
- `src/dayXX.rs`: Contains `part1(input: &str)` and `part2(input: &str)` functions marked with `#[must_use]`, plus unit tests with example inputs
- `src/bin/dayXX.rs`: Reads `../inputs/XX` and prints results for both parts
- Tests use embedded example inputs from problem descriptions

### Code Quality
The codebase uses `#![warn(clippy::pedantic)]` in lib.rs. All functions returning values should be marked with `#[must_use]`.

## Common Development Commands

### Rust (from YEAR/rust/ directory)

**Build and run a specific day:**
```bash
cargo run --bin dayXX
```

**Run all tests:**
```bash
cargo test
```

**Run tests for a specific day:**
```bash
cargo test dayXX
```

**Lint with clippy:**
```bash
cargo clippy
```

**Run benchmarks:**
```bash
cargo bench
```

**Set up a new day (from Makefile):**
```bash
make init/XX  # Creates dayXX.rs from templates and updates lib.rs
```

The `make init/XX` command:
1. Copies `template.rs` to `src/dayXX.rs`
2. Copies `bin_template.rs` to `src/bin/dayXX.rs`
3. Adds `pub mod dayXX;` to `src/lib.rs`
4. Replaces `XX` placeholders with the day number

### Benchmarking
The repository uses Criterion for benchmarking with custom configuration:
- Significance level: 0.1
- Sample size: 25

When adding a new day's benchmark, update `benches/days.rs` to include the new day macro invocation and add it to the criterion_group targets list.

## Working with Solutions

When implementing a new day:
1. Use the Makefile to scaffold: `make init/XX`
2. Replace the template logic in `src/dayXX.rs` with actual solution
3. Update the test input and expected values in the tests module
4. Run with `cargo run --bin dayXX` after adding input to `../inputs/XX`
5. Validate with `cargo test dayXX`
6. Add to benchmarks in `benches/days.rs` if desired

Input files are read relative to the binary location (`../inputs/XX` from the rust/ directory).
