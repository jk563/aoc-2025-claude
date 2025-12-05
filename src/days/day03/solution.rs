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
fn max_joltage(bank: &str) -> u32 {
    let digits: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut max = 0;

    // Try all pairs (i, j) where i < j
    for i in 0..digits.len() - 1 {
        for j in i + 1..digits.len() {
            let joltage = digits[i] * 10 + digits[j];
            max = max.max(joltage);
        }
    }

    max
}

/// Find the maximum joltage by selecting exactly k batteries from a bank
/// Uses a greedy algorithm: for each position, select the largest digit
/// that still leaves enough digits for the remaining positions
fn max_joltage_k(bank: &str, k: usize) -> u64 {
    let digits: Vec<char> = bank.chars().collect();
    let n = digits.len();

    let mut result = String::new();
    let mut start = 0;

    for position in 0..k {
        // We can search up to this index while leaving enough digits for remaining positions
        let search_end = n - k + position + 1;

        // Find the maximum digit in range [start, search_end)
        let mut max_idx = start;
        for i in start..search_end {
            if digits[i] > digits[max_idx] {
                max_idx = i;
            }
        }

        result.push(digits[max_idx]);
        start = max_idx + 1;
    }

    result.parse().unwrap()
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
}

// Define benchmarks using the common macro
crate::define_day_benches!(Day03);
