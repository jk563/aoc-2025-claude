//! Solution implementation for Day 2

use crate::runner::Day;

/// Solver for Day 2
pub struct Day02;

impl Day for Day02 {
    fn part1(&self, input: &str) -> String {
        let ranges = parse_ranges(input);
        let sum: u64 = ranges
            .iter()
            .flat_map(|(start, end)| (*start..=*end).filter(|&id| is_invalid_id(id)))
            .sum();
        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let ranges = parse_ranges(input);
        let sum: u64 = ranges
            .iter()
            .flat_map(|(start, end)| (*start..=*end).filter(|&id| is_invalid_id_v2(id)))
            .sum();
        sum.to_string()
    }
}

// Helper functions

/// Parse comma-separated ranges into (start, end) tuples
fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .filter_map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse::<u64>().ok()?;
                let end = parts[1].parse::<u64>().ok()?;
                Some((start, end))
            } else {
                None
            }
        })
        .collect()
}

/// Check if an ID is invalid (made of a pattern repeated twice) - String-based implementation
fn is_invalid_id_string(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    // Must have even number of digits to be splittable
    if !len.is_multiple_of(2) {
        return false;
    }

    // Split in half and check if both halves are equal
    let mid = len / 2;
    let first_half = &s[..mid];
    let second_half = &s[mid..];

    first_half == second_half
}

/// Check if an ID is invalid (made of a pattern repeated at least twice) - String-based implementation
fn is_invalid_id_v2_string(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=(len / 2) {
        // Check if the string length is divisible by pattern length
        if len.is_multiple_of(pattern_len) {
            let pattern = &s[..pattern_len];
            let repetitions = len / pattern_len;

            // Check if repeating the pattern gives us the original string
            if pattern.repeat(repetitions) == s {
                return true;
            }
        }
    }

    false
}

// Math-based helper functions for optimized validation

/// Count the number of digits in a number
#[inline]
fn count_digits(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

/// Get the first k digits of a number
#[inline]
fn get_first_k_digits(n: u64, total_digits: u32, k: u32) -> u64 {
    n / 10u64.pow(total_digits - k)
}

/// Get the last k digits of a number
#[inline]
fn get_last_k_digits(n: u64, k: u32) -> u64 {
    n % 10u64.pow(k)
}

/// Extract pattern of length k from the start of a number
#[inline]
fn extract_pattern(n: u64, total_digits: u32, pattern_len: u32) -> u64 {
    get_first_k_digits(n, total_digits, pattern_len)
}

/// Check if a number consists of a pattern repeated exactly
fn matches_repeated_pattern(n: u64, total_digits: u32, pattern_len: u32) -> bool {
    if total_digits % pattern_len != 0 {
        return false;
    }

    let pattern = extract_pattern(n, total_digits, pattern_len);
    let repetitions = total_digits / pattern_len;

    // Build expected value by repeating pattern
    let mut expected = 0u64;
    let multiplier = 10u64.pow(pattern_len);

    for _ in 0..repetitions {
        expected = expected * multiplier + pattern;
    }

    n == expected
}

/// Check if an ID is invalid (made of a pattern repeated twice) - Math-based implementation
fn is_invalid_id(id: u64) -> bool {
    let digits = count_digits(id);

    // Must have even number of digits to be splittable
    if digits % 2 != 0 {
        return false;
    }

    let half = digits / 2;
    let first_half = get_first_k_digits(id, digits, half);
    let second_half = get_last_k_digits(id, half);

    first_half == second_half
}

/// Check if an ID is invalid (made of a pattern repeated at least twice) - Math-based implementation
fn is_invalid_id_v2(id: u64) -> bool {
    let digits = count_digits(id);

    // Try all possible pattern lengths from 1 to digits/2
    for pattern_len in 1..=(digits / 2) {
        if matches_repeated_pattern(id, digits, pattern_len) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    /// Helper to run part1 with a specific validation function
    fn part1_with_validator<F>(input: &str, validator: F) -> String
    where
        F: Fn(u64) -> bool,
    {
        let ranges = parse_ranges(input);
        let sum: u64 = ranges
            .iter()
            .flat_map(|(start, end)| (*start..=*end).filter(|&id| validator(id)))
            .sum();
        sum.to_string()
    }

    /// Helper to run part2 with a specific validation function
    fn part2_with_validator<F>(input: &str, validator: F) -> String
    where
        F: Fn(u64) -> bool,
    {
        let ranges = parse_ranges(input);
        let sum: u64 = ranges
            .iter()
            .flat_map(|(start, end)| (*start..=*end).filter(|&id| validator(id)))
            .sum();
        sum.to_string()
    }

    #[test]
    fn test_part1_example_math() {
        assert_eq!(
            part1_with_validator(EXAMPLE, is_invalid_id),
            "1227775554",
            "Math-based implementation failed"
        );
    }

    #[test]
    fn test_part1_example_string() {
        assert_eq!(
            part1_with_validator(EXAMPLE, is_invalid_id_string),
            "1227775554",
            "String-based implementation failed"
        );
    }

    #[test]
    fn test_part2_example_math() {
        assert_eq!(
            part2_with_validator(EXAMPLE, is_invalid_id_v2),
            "4174379265",
            "Math-based implementation failed"
        );
    }

    #[test]
    fn test_part2_example_string() {
        assert_eq!(
            part2_with_validator(EXAMPLE, is_invalid_id_v2_string),
            "4174379265",
            "String-based implementation failed"
        );
    }

    #[test]
    fn test_string_vs_math_equivalence_part1() {
        // Test that both implementations agree on a range of values
        for id in [1212, 123456, 9999, 100100, 10, 1234321, 111111] {
            assert_eq!(
                is_invalid_id_string(id),
                is_invalid_id(id),
                "Mismatch for ID: {}",
                id
            );
        }
    }

    #[test]
    fn test_string_vs_math_equivalence_part2() {
        for id in [121212, 111111, 123123, 9999, 1234, 100100100, 12121212] {
            assert_eq!(
                is_invalid_id_v2_string(id),
                is_invalid_id_v2(id),
                "Mismatch for ID: {}",
                id
            );
        }
    }
}
