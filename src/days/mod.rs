//! Registry of all implemented days
//!
//! Each day's solution is registered here. To add a new day:
//! 1. Create the day module (e.g., `day01`)
//! 2. Add it to this file with `pub mod dayNN;`
//! 3. Register it in the `get_days()` function

use crate::runner::DayInfo;

/// Macro to register a day solution with optional implementation name.
///
/// # Examples
///
/// ```ignore
/// // Register a single implementation (default)
/// register_day!(1, day01::Day01, "day01/input/input.txt")
///
/// // Register a named implementation variant
/// register_day!(2, "math", day02::Day02Math, "day02/input/input.txt")
/// ```
#[macro_export]
macro_rules! register_day {
    // Default implementation (no name)
    ($number:expr, $solver:expr, $input_file:expr) => {
        DayInfo {
            number: $number,
            impl_name: None,
            solver: Box::new($solver),
            input: include_str!($input_file),
        }
    };
    // Named implementation variant
    ($number:expr, $impl_name:expr, $solver:expr, $input_file:expr) => {
        DayInfo {
            number: $number,
            impl_name: Some($impl_name.to_string()),
            solver: Box::new($solver),
            input: include_str!($input_file),
        }
    };
}

// Day modules
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

/// Get all registered days
///
/// Returns a vector of all implemented day solutions.
/// Days are automatically discovered and registered here.
pub fn get_days() -> Vec<DayInfo> {
    vec![
        register_day!(1, day01::Day01, "day01/input/input.txt"),
        register_day!(2, day02::Day02, "day02/input/input.txt"),
        register_day!(3, day03::Day03, "day03/input/input.txt"),
        register_day!(4, day04::Day04, "day04/input/input.txt"),
        register_day!(5, day05::Day05, "day05/input/input.txt"),
        register_day!(6, day06::Day06, "day06/input/input.txt"),
        register_day!(7, day07::Day07, "day07/input/input.txt"),
    ]
}

/// Get all implementations for all days
///
/// Returns a vector including all available implementations for each day.
/// This is used when the --all-impls flag is provided.
pub fn get_all_implementations() -> Vec<DayInfo> {
    vec![
        // Day 1 - single implementation
        register_day!(1, day01::Day01, "day01/input/input.txt"),
        // Day 2 - mathematical generation (fastest, default)
        register_day!(2, "generate", day02::Day02Generate, "day02/input/input.txt"),
        // Day 2 - modulo-based implementation
        register_day!(2, "modulo", day02::Day02Modulo, "day02/input/input.txt"),
        // Day 2 - math-based implementation
        register_day!(2, "math", day02::Day02Math, "day02/input/input.txt"),
        // Day 2 - string-based implementation
        register_day!(2, "string", day02::Day02String, "day02/input/input.txt"),
        // Day 3 - single implementation
        register_day!(3, day03::Day03, "day03/input/input.txt"),
        // Day 4 - default (fastest)
        register_day!(4, day04::Day04, "day04/input/input.txt"),
        // Day 4 - dirty tracking
        register_day!(
            4,
            "dirty-tracking",
            day04::Day04DirtyTracking,
            "day04/input/input.txt"
        ),
        // Day 4 - naive baseline
        register_day!(4, "naive", day04::Day04Naive, "day04/input/input.txt"),
        // Day 5 - single implementation
        register_day!(5, day05::Day05, "day05/input/input.txt"),
        // Day 6 - single implementation
        register_day!(6, day06::Day06, "day06/input/input.txt"),
        // Day 7 - single implementation
        register_day!(7, day07::Day07, "day07/input/input.txt"),
    ]
}

/// Get a specific day by number
pub fn get_day(number: usize) -> Option<DayInfo> {
    get_days().into_iter().find(|d| d.number == number)
}
