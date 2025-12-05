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
//! ### Default Implementation: Day04 (Hybrid)
//!
//! Uses the best algorithm for each part:
//! - **Part 1:** Simple neighbor counting without pre-computation overhead
//! - **Part 2:** Pre-computed neighbor counts with incremental updates via VecDeque
//!
//! Algorithm for Part 2:
//! 1. Parse grid and compute initial neighbor counts for all rolls
//! 2. Initialize queue with all accessible rolls (< 4 neighbors)
//! 3. Process queue: remove roll, decrement neighbor counts, add newly accessible rolls
//! 4. Each roll is processed exactly once
//!
//! **Performance:** ~1.22ms total (213µs part1, 1.01ms part2)
//!
//! ### Alternative Implementations
//!
//! - `Day04DirtyTracking`: Uses HashSet dirty tracking (~1.5ms total)
//! - `Day04Naive`: Naive grid scanning (~5.6ms total)

mod solution;

pub use solution::{Day04, Day04DirtyTracking, Day04Naive};
