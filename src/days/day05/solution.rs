//! Solution implementation for Day 5: Cafeteria
//!
//! This puzzle involves checking ingredient IDs against fresh ingredient ranges.
//!
//! # Part 1
//! Count how many ingredient IDs from a list fall within any of the fresh ranges.
//! Uses a simple O(n×m) linear search approach which is optimal for the input size.
//!
//! # Part 2
//! Count the total number of unique IDs covered by all ranges.
//! Uses range merging to avoid double-counting overlapping ranges.

use crate::runner::Day;

/// Solver for Day 5
pub struct Day05;

/// Parse the input into fresh ingredient ranges and ingredient IDs to check.
///
/// The input format is two sections separated by a blank line:
/// - First section: ranges in format "start-end" (inclusive)
/// - Second section: individual ingredient IDs, one per line
fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    // Parse ranges in format "start-end"
    let ranges = parts[0]
        .lines()
        .map(|line| {
            let nums: Vec<u64> = line.split('-').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    // Parse ingredient IDs to check
    let ids = parts[1]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ids)
}

/// Check if an ingredient ID is fresh (falls within any range).
///
/// Uses `.any()` for early termination - stops checking as soon as a match is found.
fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    ranges.iter().any(|&(start, end)| id >= start && id <= end)
}

/// Merge overlapping or adjacent ranges into non-overlapping ranges.
///
/// Algorithm:
/// 1. Sort ranges by start position
/// 2. Iterate through sorted ranges, merging when they overlap or are adjacent
/// 3. Adjacent ranges (e.g., 5-10 and 11-20) are merged into continuous ranges
///
/// Time complexity: O(n log n) for sorting, O(n) for merging
fn merge_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return vec![];
    }

    // Sort ranges by start position
    let mut sorted = ranges.to_vec();
    sorted.sort_by_key(|&(start, _)| start);

    let mut merged = vec![sorted[0]];

    for &(start, end) in &sorted[1..] {
        let last_idx = merged.len() - 1;
        let (last_start, last_end) = merged[last_idx];

        // Check if current range overlaps or is adjacent (start <= last_end + 1)
        // This handles both overlapping ranges and adjacent ranges
        if start <= last_end + 1 {
            // Merge by extending the end if the current range extends further
            merged[last_idx] = (last_start, last_end.max(end));
        } else {
            // No overlap, add as new range
            merged.push((start, end));
        }
    }

    merged
}

/// Count the total number of unique ingredient IDs covered by all ranges.
///
/// First merges overlapping ranges to avoid double-counting, then sums the sizes.
fn count_ids_in_ranges(ranges: &[(u64, u64)]) -> u64 {
    let merged = merge_ranges(ranges);
    merged.iter().map(|&(start, end)| end - start + 1).sum()
}

impl Day for Day05 {
    fn part1(&self, input: &str) -> String {
        let (ranges, ids) = parse_input(input);
        let count = ids.iter().filter(|&&id| is_fresh(id, &ranges)).count();
        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (ranges, _) = parse_input(input);
        let count = count_ids_in_ranges(&ranges);
        count.to_string()
    }
}

// Helper functions

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1_example() {
        let day = Day05;
        assert_eq!(day.part1(EXAMPLE), "3");
    }

    #[test]
    fn test_part2_example() {
        let day = Day05;
        assert_eq!(day.part2(EXAMPLE), "14");
    }
}

// Define benchmarks using the common macro
crate::define_day_benches!(Day05);
