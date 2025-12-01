//! ASCII table formatting for displaying results
//!
//! Simple, custom table implementation to avoid external dependencies.
//! Formats day results into a readable ASCII table with proper alignment.

use crate::runner::{format_duration, DayResult};
use std::time::Duration;

/// Format results as an ASCII table
///
/// Creates a table with columns:
/// - Day
/// - Part 1 (result)
/// - Part 2 (result)
/// - Part 1 Time
/// - Part 2 Time
/// - Total Time
pub fn format_results(results: &[DayResult]) -> String {
    if results.is_empty() {
        return String::from("No days implemented yet.\n\nTo get started:\n  1. Run `just new-day 01` to create a new day\n  2. Add puzzle input to src/days/day01/input/\n  3. Implement the solution in src/days/day01/solution.rs\n  4. Register the day in src/days/mod.rs");
    }

    let mut output = String::new();

    // Calculate column widths
    let day_width = 5; // "Day 01" fits comfortably
    let part1_width = results
        .iter()
        .map(|r| r.part1_result.len())
        .max()
        .unwrap_or(10)
        .max(6); // "Part 1" header
    let part2_width = results
        .iter()
        .map(|r| r.part2_result.len())
        .max()
        .unwrap_or(10)
        .max(6); // "Part 2" header
    let time_width = 12; // Wide enough for "1234.56 ms"

    // Top border
    output.push_str(&format!(
        "┌─{}─┬─{}─┬─{}─┬─{}─┬─{}─┬─{}─┐\n",
        "─".repeat(day_width),
        "─".repeat(part1_width),
        "─".repeat(part2_width),
        "─".repeat(time_width),
        "─".repeat(time_width),
        "─".repeat(time_width)
    ));

    // Header
    output.push_str(&format!(
        "│ {:^width$} │ {:^part1$} │ {:^part2$} │ {:^time$} │ {:^time$} │ {:^time$} │\n",
        "Day",
        "Part 1",
        "Part 2",
        "Part 1 Time",
        "Part 2 Time",
        "Total",
        width = day_width,
        part1 = part1_width,
        part2 = part2_width,
        time = time_width
    ));

    // Header separator
    output.push_str(&format!(
        "├─{}─┼─{}─┼─{}─┼─{}─┼─{}─┼─{}─┤\n",
        "─".repeat(day_width),
        "─".repeat(part1_width),
        "─".repeat(part2_width),
        "─".repeat(time_width),
        "─".repeat(time_width),
        "─".repeat(time_width)
    ));

    // Data rows
    for result in results {
        output.push_str(&format!(
            "│ {:>width$} │ {:>part1$} │ {:>part2$} │ {:>time$} │ {:>time$} │ {:>time$} │\n",
            format!("Day {:02}", result.day_number),
            result.part1_result,
            result.part2_result,
            format_duration(result.part1_time),
            format_duration(result.part2_time),
            format_duration(result.total_time()),
            width = day_width,
            part1 = part1_width,
            part2 = part2_width,
            time = time_width
        ));
    }

    // Calculate totals
    let total_time: Duration = results.iter().map(|r| r.total_time()).sum();

    // Bottom separator
    output.push_str(&format!(
        "├─{}─┴─{}─┴─{}─┴─{}─┴─{}─┼─{}─┤\n",
        "─".repeat(day_width),
        "─".repeat(part1_width),
        "─".repeat(part2_width),
        "─".repeat(time_width),
        "─".repeat(time_width),
        "─".repeat(time_width)
    ));

    // Total row
    output.push_str(&format!(
        "│ {:width$} │ {:>time$} │\n",
        "Total",
        format_duration(total_time),
        width = day_width + part1_width + part2_width + time_width + time_width + 10, // +10 for separators
        time = time_width
    ));

    // Bottom border
    output.push_str(&format!(
        "└─{}─┴─{}─┘\n",
        "─".repeat(day_width + part1_width + part2_width + time_width + time_width + 10),
        "─".repeat(time_width)
    ));

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_empty_results() {
        let output = format_results(&[]);
        assert!(output.contains("No days implemented yet"));
    }

    #[test]
    fn test_format_single_result() {
        let results = vec![DayResult {
            day_number: 1,
            part1_result: "42".to_string(),
            part2_result: "100".to_string(),
            part1_time: Duration::from_micros(500),
            part2_time: Duration::from_millis(2),
        }];

        let output = format_results(&results);
        assert!(output.contains("Day 01"));
        assert!(output.contains("42"));
        assert!(output.contains("100"));
        assert!(output.contains("500 µs"));
        assert!(output.contains("2.00 ms"));
    }

    #[test]
    fn test_format_multiple_results() {
        let results = vec![
            DayResult {
                day_number: 1,
                part1_result: "42".to_string(),
                part2_result: "100".to_string(),
                part1_time: Duration::from_micros(500),
                part2_time: Duration::from_millis(2),
            },
            DayResult {
                day_number: 2,
                part1_result: "1234".to_string(),
                part2_result: "5678".to_string(),
                part1_time: Duration::from_millis(5),
                part2_time: Duration::from_millis(10),
            },
        ];

        let output = format_results(&results);
        assert!(output.contains("Day 01"));
        assert!(output.contains("Day 02"));
        assert!(output.contains("Total"));
    }
}
