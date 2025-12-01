//! Registry of all implemented days
//!
//! Each day's solution is registered here. To add a new day:
//! 1. Create the day module (e.g., `day01`)
//! 2. Add it to this file with `pub mod dayNN;`
//! 3. Register it in the `get_days()` function

use crate::runner::DayInfo;

// Example module declaration (uncomment when implementing):
// pub mod day01;

/// Get all registered days
///
/// Returns a vector of all implemented day solutions.
/// Days are automatically discovered and registered here.
pub fn get_days() -> Vec<DayInfo> {
    vec![
        // Register days here as they're implemented:
        // DayInfo {
        //     number: 1,
        //     solver: Box::new(day01::Day01),
        //     input: include_str!("day01/input/input.txt"),
        // },
    ]
}

/// Get a specific day by number
pub fn get_day(number: usize) -> Option<DayInfo> {
    get_days().into_iter().find(|d| d.number == number)
}
