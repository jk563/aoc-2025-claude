//! Day 4: Printing Department
//!
//! Solve the forklift warehouse puzzle where rolls can be removed if they have
//! fewer than 4 adjacent neighbors (Moore neighborhood).
//!
//! ## Problem Summary
//!
//! Given a grid of toilet paper rolls (`@`) and empty spaces (`.`), determine:
//! - Part 1: How many rolls can be removed initially (< 4 neighbors)
//! - Part 2: Total rolls that can be removed through successive iterations
//!
//! ## Algorithm
//!
//! ### Default Implementation: Day04NeighborCount
//!
//! Uses pre-computed neighbor counts with incremental updates:
//! 1. Parse grid and compute initial neighbor counts for all rolls
//! 2. Initialize queue with all accessible rolls (< 4 neighbors)
//! 3. Process queue: remove roll, decrement neighbor counts, add newly accessible rolls
//! 4. Each roll is processed exactly once
//!
//! **Performance:** ~0.96ms total (220µs part1, 519µs part2)
//!
//! ### Alternative Implementations
//!
//! - `Day04Optimized`: Uses HashSet dirty tracking (~1.5ms total)
//! - `Day04`: Original naive grid scanning (~9.4ms total)

mod solution;

pub use solution::{Day04, Day04NeighborCount, Day04Optimized};
