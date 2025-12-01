#![feature(test)]

//! Benchmark harness for all daily solutions
//!
//! This file will automatically include benchmarks from each day's solution module.
//! Individual days include their own benchmark code using the #[bench] attribute.

extern crate test;

// Benchmarks are defined in each day's solution.rs file
// They're automatically discovered by cargo bench when using:
// #[cfg(all(feature = "bench", test))]
// mod benches { ... }
