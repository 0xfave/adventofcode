# 🎄 Advent of Code 2025 [![License: MIT][license-badge]][license]

[license]: https://opensource.org/licenses/MIT
[license-badge]: https://img.shields.io/badge/License-MIT-blue.svg

My solutions for [Advent of Code 2025](https://adventofcode.com/2025), implemented in Rust.

## Progress

- ⭐⭐ **Day 1**: Secret Entrance - Dial rotation and counting zero crossings
- ⭐⭐ **Day 2**: Gift Shop - Finding invalid product IDs with repeating patterns

## Running Solutions

```bash
# Run all solutions
cargo run --release

# Run tests
cargo test

# Run linting and formatting checks
just full-check
```

## Project Structure

```text
src/
├── lib/
│   ├── day01/
│   │   ├── day01_part1.rs  # Day 1 Part 1 solution
│   │   ├── day01_part2.rs  # Day 1 Part 2 solution
│   │   ├── input.txt       # Day 1 puzzle input
│   │   └── mod.rs
│   ├── day02/
│   │   ├── day02_part1.rs  # Day 2 Part 1 solution
│   │   ├── day02_part2.rs  # Day 2 Part 2 solution
│   │   ├── input.txt       # Day 2 puzzle input
│   │   └── mod.rs
│   └── mod.rs
├── lib.rs
└── main.rs
```

## About Advent of Code

[Advent of Code](https://adventofcode.com/) is an annual event featuring daily programming puzzles throughout December. Each day presents a two-part challenge that tests problem-solving skills and algorithmic thinking.

## License

This project is licensed under MIT.
