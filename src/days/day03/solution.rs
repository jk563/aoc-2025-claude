//! Solution implementation for Day 3

use crate::runner::Day;

/// Solver for Day 3
pub struct Day03;

impl Day for Day03 {
    fn part1(&self, input: &str) -> String {
        input.lines().map(max_joltage).sum::<u32>().to_string()
    }

    fn part2(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| max_joltage_k(line, 12))
            .sum::<u64>()
            .to_string()
    }
}

// Helper functions

/// Find the maximum joltage for a single battery bank
/// by trying all pairs of batteries in order
/// Optimized using suffix maximum preprocessing: O(n) instead of O(n²)
fn max_joltage(bank: &str) -> u32 {
    let digits = bank.as_bytes();
    let n = digits.len();

    // Precompute suffix maximum for O(n) complexity
    // suffix_max[i] = maximum digit value from position i to end
    let mut suffix_max = vec![0u8; n];
    suffix_max[n - 1] = digits[n - 1];
    for i in (0..n - 1).rev() {
        suffix_max[i] = suffix_max[i + 1].max(digits[i]);
    }

    // Find maximum by pairing each digit with its suffix maximum
    let mut max = 0;
    for i in 0..n - 1 {
        let first = (digits[i] - b'0') as u32;
        let second = (suffix_max[i + 1] - b'0') as u32;
        let joltage = first * 10 + second;
        max = max.max(joltage);
    }

    max
}

/// Find the maximum joltage by selecting exactly k batteries from a bank
/// Uses a greedy algorithm: for each position, select the largest digit
/// that still leaves enough digits for the remaining positions
/// Optimized to build u64 directly without String intermediary
fn max_joltage_k(bank: &str, k: usize) -> u64 {
    let digits = bank.as_bytes();
    let n = digits.len();

    let mut result: u64 = 0;
    let mut start = 0;

    for position in 0..k {
        // We can search up to this index while leaving enough digits for remaining positions
        let search_end = n - k + position + 1;

        // Find the maximum digit in range [start, search_end)
        let mut max_idx = start;
        for i in start + 1..search_end {
            if digits[i] > digits[max_idx] {
                max_idx = i;
            }
        }

        // Build u64 directly
        result = result * 10 + (digits[max_idx] - b'0') as u64;
        start = max_idx + 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1_example() {
        let day = Day03;
        assert_eq!(day.part1(EXAMPLE), "357");
    }

    #[test]
    fn test_part2_example() {
        let day = Day03;
        assert_eq!(day.part2(EXAMPLE), "3121910778619");
    }

    #[test]
    fn test_part1_edge_cases() {
        // Ascending sequence - max should be last two digits
        assert_eq!(max_joltage("123456789"), 89);
        // Descending sequence - max should be first two digits
        assert_eq!(max_joltage("987654321"), 98);
        // All same digits
        assert_eq!(max_joltage("5555"), 55);
        // Two digits only
        assert_eq!(max_joltage("42"), 42);
        // All nines
        assert_eq!(max_joltage("999"), 99);
        // Contains 99 as a pair
        assert_eq!(max_joltage("1928394959"), 99);
    }

    #[test]
    fn test_part2_edge_cases() {
        // Verify greedy correctness matches expectation
        assert_eq!(max_joltage_k("987654321111111", 12), 987654321111);
        // All same digits
        assert_eq!(max_joltage_k("5555555555", 5), 55555);
        // Ascending
        assert_eq!(max_joltage_k("123456789", 5), 56789);
    }
}

// Define benchmarks using the common macro
crate::define_day_benches!(Day03);
