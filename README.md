# Advent of Code 2025

High-performance Rust solutions for [Advent of Code 2025](https://adventofcode.com/2025) with comprehensive documentation and benchmarking.

## Features

- **Fast**: Optimized solutions with performance benchmarking
- **Educational**: Detailed documentation explaining algorithms and trade-offs
- **Well-tested**: Comprehensive test coverage with example inputs
- **Clean code**: Readable, well-commented implementations following Rust best practices
- **Minimal dependencies**: Lean dependency tree, custom implementations where practical

## Quick Start

### Prerequisites

- Rust nightly (automatically configured via `rust-toolchain.toml`)
- [just](https://github.com/casey/just) command runner (optional but recommended)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd aoc-2025/claude/rust

# Initialize environment (installs nightly Rust if needed)
just init

# Run all implemented days
just run

# Run a specific day
just run-day 1
```

### Without `just`

```bash
# Run all days
cargo run --release

# Run a specific day
cargo run --release -- --day 1

# Run tests
cargo test

# Run benchmarks
cargo +nightly bench
```

## Project Structure

```
rust/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library root
│   ├── runner.rs            # Day execution and timing logic
│   ├── table.rs             # ASCII table formatting
│   ├── common/              # Shared utilities (grids, parsing, algorithms)
│   │   └── mod.rs
│   └── days/                # Daily solutions
│       ├── mod.rs           # Day registry
│       └── dayNN/           # Each day's solution
│           ├── mod.rs       # Module root with documentation
│           ├── solution.rs  # Part 1 & 2 implementations
│           ├── README.md    # Educational documentation
│           └── input/
│               ├── puzzle.txt   # Problem statement
│               └── input.txt    # Puzzle input
├── benches/                 # Benchmark tests
├── tests/                   # Integration tests
├── scripts/                 # Helper scripts
│   └── new_day.sh          # Scaffold a new day
├── .claude/                 # Claude Code context
│   ├── context.md          # Development guidelines
│   └── templates/          # File templates
└── Justfile                # Development task recipes
```

## Usage

### Running Solutions

```bash
# Run all implemented days
just run

# Run a specific day
just run-day 5

# Show help
cargo run -- --help
```

Output is displayed in an ASCII table with timing information:

```
┌───────┬────────┬────────┬──────────────┬──────────────┬────────────┐
│  Day  │ Part 1 │ Part 2 │ Part 1 Time  │ Part 2 Time  │   Total    │
├───────┼────────┼────────┼──────────────┼──────────────┼────────────┤
│ Day 01│  12345 │  67890 │    1.23 ms   │    2.45 ms   │   3.68 ms  │
└───────┴────────┴────────┴──────────────┴──────────────┴────────────┘
```

### Testing

```bash
# Run all tests
just test

# Run tests for a specific day
just test-day 1

# Run with output
cargo test -- --nocapture
```

### Benchmarking

```bash
# Run all benchmarks
just bench

# Run benchmarks for a specific day
just bench-day 1
```

### Code Quality

```bash
# Format code
just fmt

# Run linter
just lint

# Run all checks (format, lint, test, build)
just ci
```

### Documentation

```bash
# Generate and open documentation
just doc

# Or manually
cargo doc --open --no-deps
```

Each day includes:
- Rustdoc comments explaining the implementation
- A standalone README.md with algorithm analysis and educational content
- See `.claude/templates/day_readme.md` for the documentation template

## Adding a New Day

### Automated (Recommended)

```bash
just new-day 1
```

This creates the directory structure and template files for the specified day.

### Manual Steps

1. Create directory structure:
   ```bash
   mkdir -p src/days/day01/input
   ```

2. Add puzzle input files:
   - `src/days/day01/input/puzzle.txt` - Problem statement from AoC
   - `src/days/day01/input/input.txt` - Your puzzle input

3. Create solution files using templates from `.claude/templates/`

4. Register the day in `src/days/mod.rs`:
   ```rust
   pub mod day01;

   // In get_days():
   DayInfo {
       number: 1,
       solver: Box::new(day01::Day01),
       input: include_str!("day01/input/input.txt"),
   }
   ```

5. Implement the solution following the template in `.claude/templates/`

6. Run tests: `just test-day 1`

7. Document your solution in `src/days/day01/README.md`

## Development Workflow

This project follows a structured workflow for implementing solutions:

1. **Setup**: Verify input files exist, commit them first
2. **Part 1**: Implement with tests, verify with examples, commit
3. **Part 2**: Implement with tests, verify, commit
4. **Optimize**: Profile and benchmark if needed, commit
5. **Refactor**: Move common code to `src/common/`, commit
6. **Document**: Write educational README for the day, commit

See `.claude/context.md` for detailed guidelines and best practices.

## Design Philosophy

### Code Quality

- **Readable over clever**: Clear code is better than obscure optimizations
- **Commented where needed**: Complex logic gets explanatory comments
- **Well-tested**: Example inputs validate correctness before running on real data
- **Documented**: Both rustdoc and standalone docs explain the approach

### Performance

- **Optimal algorithms**: Choose the right algorithm for the input size
- **Benchmarked**: When multiple approaches exist, benchmark to decide
- **Target <1s per day**: Both parts should complete quickly
- **Keep alternatives**: Losing benchmark implementations stay in code for reference

### Dependencies

- **Minimal by default**: Evaluate each dependency's utility vs size
- **Custom when simple**: Implement ourselves if it's <50 lines
- **Standard library first**: Use std when possible

### Documentation

Each day's README includes:
- Problem statement summary
- Algorithm explanation with complexity analysis
- Implementation notes (interesting Rust patterns, challenges)
- Benchmark results (if multiple approaches were tested)
- Alternative approaches considered

## Common Utilities

The `src/common/` module contains shared code used across multiple days:

- **Grid utilities**: 2D array operations, neighbor iteration
- **Parsing helpers**: Common input parsing patterns
- **Graph algorithms**: BFS, DFS, Dijkstra, etc.
- **Math utilities**: GCD, LCM, prime factorization, etc.

These are added incrementally as patterns emerge across different days.

## Git Workflow

Commits follow a structured pattern:

- `dayNN: add puzzle input` - Input files committed first
- `dayNN: implement part1` - Working Part 1 solution
- `dayNN: implement part2` - Working Part 2 solution
- `dayNN: optimize and benchmark` - Performance improvements
- `dayNN: refactor to common` - Moving shared code
- `dayNN: add documentation` - Educational README

This creates a clear history showing the evolution of each solution.

## Performance Targets

- ✅ Both parts <1s combined (ideal)
- ⚠️ Both parts <5s (acceptable)
- ❌ Either part >5s (needs optimization)

## Contributing

This is a personal learning project, but suggestions for optimizations or better algorithms are welcome via issues or pull requests.

## License

MIT License - See LICENSE file for details

## Acknowledgments

- [Advent of Code](https://adventofcode.com/) by Eric Wastl
- The Rust community for excellent tooling and libraries

---

**For Claude Code sessions**: See `.claude/context.md` for comprehensive development guidelines.
