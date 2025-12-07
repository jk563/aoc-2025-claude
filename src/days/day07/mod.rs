//! Day 7: Tachyon Manifold Splitters
//!
//! Track tachyon beams through a manifold with splitters. Part 1 counts splits (beams merge).
//! Part 2 counts distinct timelines (particles don't merge).
//!
//! ## Problem Summary
//!
//! A tachyon beam enters at position S and travels downward. When it hits a splitter (^),
//! it creates two beams going left and right. Part 1 counts total splits. Part 2 counts
//! the number of distinct timelines when particles branch at each splitter.
//!
//! ## Algorithm
//!
//! **Part 1:** Use HashSet to track active beam columns. Find intersection with splitters,
//! count hits, and update beam positions. Beams merge automatically via set deduplication.
//!
//! **Part 2:** Use HashMap to track particle counts at each column. When hitting a splitter,
//! add count to both left and right neighbors. Sum final counts for total timelines.

mod solution;

pub use solution::Day07;
