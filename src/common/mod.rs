//! Common utilities shared across multiple days
//!
//! This module contains helper functions, data structures, and algorithms
//! that are used by multiple day solutions. Common patterns include:
//!
//! - Grid/2D array utilities
//! - Parsing helpers
//! - Graph algorithms (BFS, DFS, Dijkstra, etc.)
//! - Number theory utilities (GCD, LCM, primes, etc.)
//! - String manipulation helpers
//!
//! ## Guidelines for Adding Common Code
//!
//! - Add code here when it's used by 2+ days
//! - Keep implementations generic and well-documented
//! - Prefer small, focused utilities over large frameworks
//! - Add examples in doc comments
//! - Consider performance implications
//!
//! ## Module Organization
//!
//! As common code grows, organize into submodules:
//! ```ignore
//! pub mod grid;    // 2D grid utilities
//! pub mod parse;   // Parsing helpers
//! pub mod graph;   // Graph algorithms
//! pub mod math;    // Number theory
//! ```

// Modules will be added here as common patterns emerge during daily implementations
// Example:
// pub mod grid;
// pub mod parse;

/// Parse non-empty, trimmed lines from input and apply a transformation function.
///
/// This utility handles the common pattern of:
/// 1. Iterating over lines
/// 2. Trimming whitespace
/// 3. Skipping empty lines
/// 4. Parsing/transforming each line
///
/// # Examples
///
/// ```
/// use aoc2025::common::parse_lines;
///
/// let input = "L10\nR20\n\nL30\n";
/// let lines = parse_lines(input, |line: &str| {
///     let direction = line.chars().next()?;
///     let distance: i32 = line[1..].parse().ok()?;
///     Some((direction, distance))
/// });
/// assert_eq!(lines, vec![('L', 10), ('R', 20), ('L', 30)]);
/// ```
pub fn parse_lines<T, F>(input: &str, mut parser: F) -> Vec<T>
where
    F: FnMut(&str) -> Option<T>,
{
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                parser(line)
            }
        })
        .collect()
}

/// Macro to define standard benchmarks for a day's solution.
///
/// This macro creates two benchmark functions:
/// - `bench_part1`: Benchmarks part1 of the solution
/// - `bench_part2`: Benchmarks part2 of the solution
///
/// # Examples
///
/// ```ignore
/// use crate::define_day_benches;
///
/// // For unit structs or types with simple construction
/// define_day_benches!(Day01);
///
/// // For types that need a specific instance
/// define_day_benches!(Day02, Day02Math);
/// ```
#[macro_export]
macro_rules! define_day_benches {
    // For types where we have a specific instance (const)
    ($solver_type:ty, $solver_instance:expr) => {
        #[cfg(all(test, not(debug_assertions)))]
        mod benches {
            extern crate test;
            use super::*;
            use test::Bencher;

            #[bench]
            fn bench_part1(b: &mut Bencher) {
                let input = include_str!("input/input.txt");
                b.iter(|| $solver_instance.part1(input));
            }

            #[bench]
            fn bench_part2(b: &mut Bencher) {
                let input = include_str!("input/input.txt");
                b.iter(|| $solver_instance.part2(input));
            }
        }
    };
    // For unit structs or simple types
    ($solver:ident) => {
        #[cfg(all(test, not(debug_assertions)))]
        mod benches {
            extern crate test;
            use super::*;
            use test::Bencher;

            #[bench]
            fn bench_part1(b: &mut Bencher) {
                let input = include_str!("input/input.txt");
                let day = $solver;
                b.iter(|| day.part1(input));
            }

            #[bench]
            fn bench_part2(b: &mut Bencher) {
                let input = include_str!("input/input.txt");
                let day = $solver;
                b.iter(|| day.part2(input));
            }
        }
    };
}
