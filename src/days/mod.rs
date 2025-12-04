//! Registry of all implemented days
//!
//! Each day's solution is registered here. To add a new day:
//! 1. Create the day module (e.g., `day01`)
//! 2. Add it to this file with `pub mod dayNN;`
//! 3. Register it in the `get_days()` function

use crate::runner::DayInfo;

// Day modules
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

/// Get all registered days
///
/// Returns a vector of all implemented day solutions.
/// Days are automatically discovered and registered here.
pub fn get_days() -> Vec<DayInfo> {
    vec![
        DayInfo {
            number: 1,
            impl_name: None,
            solver: Box::new(day01::Day01),
            input: include_str!("day01/input/input.txt"),
        },
        DayInfo {
            number: 2,
            impl_name: None,
            solver: Box::new(day02::Day02Math),
            input: include_str!("day02/input/input.txt"),
        },
        DayInfo {
            number: 3,
            impl_name: None,
            solver: Box::new(day03::Day03),
            input: include_str!("day03/input/input.txt"),
        },
        DayInfo {
            number: 4,
            impl_name: None,
            solver: Box::new(day04::Day04Optimized),
            input: include_str!("day04/input/input.txt"),
        },
    ]
}

/// Get all implementations for all days
///
/// Returns a vector including all available implementations for each day.
/// This is used when the --all-impls flag is provided.
pub fn get_all_implementations() -> Vec<DayInfo> {
    vec![
        // Day 1 - single implementation
        DayInfo {
            number: 1,
            impl_name: None,
            solver: Box::new(day01::Day01),
            input: include_str!("day01/input/input.txt"),
        },
        // Day 2 - math-based implementation
        DayInfo {
            number: 2,
            impl_name: Some("math".to_string()),
            solver: Box::new(day02::Day02Math),
            input: include_str!("day02/input/input.txt"),
        },
        // Day 2 - string-based implementation
        DayInfo {
            number: 2,
            impl_name: Some("string".to_string()),
            solver: Box::new(day02::Day02String),
            input: include_str!("day02/input/input.txt"),
        },
        // Day 2 - modulo-based implementation (fastest)
        DayInfo {
            number: 2,
            impl_name: Some("modulo".to_string()),
            solver: Box::new(day02::Day02Modulo),
            input: include_str!("day02/input/input.txt"),
        },
        // Day 3 - single implementation
        DayInfo {
            number: 3,
            impl_name: None,
            solver: Box::new(day03::Day03),
            input: include_str!("day03/input/input.txt"),
        },
        // Day 4 - optimized implementation (default)
        DayInfo {
            number: 4,
            impl_name: Some("optimized".to_string()),
            solver: Box::new(day04::Day04Optimized),
            input: include_str!("day04/input/input.txt"),
        },
        // Day 4 - original implementation (for benchmarking)
        DayInfo {
            number: 4,
            impl_name: Some("original".to_string()),
            solver: Box::new(day04::Day04),
            input: include_str!("day04/input/input.txt"),
        },
    ]
}

/// Get a specific day by number
pub fn get_day(number: usize) -> Option<DayInfo> {
    get_days().into_iter().find(|d| d.number == number)
}
