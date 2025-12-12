//! Day 9: Movie Theater
//!
//! Find the largest rectangle in a tile grid based on red tile positions.
//!
//! ## Problem Summary
//!
//! **Part 1**: Find the largest rectangle using any two red tiles as opposite corners.
//! This is a straightforward O(n²) brute force approach checking all pairs.
//!
//! **Part 2**: Find the largest rectangle that only contains red or green tiles.
//! Red tiles form a polygon (connected in order), and green tiles are on the
//! polygon edges and inside it. This requires computational geometry: ray casting
//! for point-in-polygon tests, cross products for point-on-segment checks, and
//! caching to avoid redundant expensive calculations.
//!
//! ## Algorithm
//!
//! Part 2 uses:
//! - **Ray Casting**: Point-in-polygon testing using horizontal ray intersections
//! - **Cross Product**: Elegant collinearity check for point-on-segment validation
//! - **HashMap Caching**: Memoize expensive geometric calculations
//! - **Adaptive Sampling**: Dense sampling for small rectangles, sparse for large
//! - **Bounding Box Pruning**: Early rejection of invalid rectangles

mod solution;

pub use solution::Day09;
