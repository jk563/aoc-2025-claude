//! Solution implementation for Day 2

use crate::runner::Day;

/// Generic solver for Day 2 that accepts different validation functions
///
/// This allows us to easily swap out different validation implementations
/// (math-based, string-based, modulo-based) without duplicating the Day trait implementation.
pub struct ValidatingSolver<P1, P2>
where
    P1: Fn(u64) -> bool,
    P2: Fn(u64) -> bool,
{
    part1_validator: P1,
    part2_validator: P2,
}

impl<P1, P2> ValidatingSolver<P1, P2>
where
    P1: Fn(u64) -> bool,
    P2: Fn(u64) -> bool,
{
    pub const fn new(part1_validator: P1, part2_validator: P2) -> Self {
        Self {
            part1_validator,
            part2_validator,
        }
    }
}

impl<P1, P2> Day for ValidatingSolver<P1, P2>
where
    P1: Fn(u64) -> bool + Send + Sync,
    P2: Fn(u64) -> bool + Send + Sync,
{
    fn part1(&self, input: &str) -> String {
        let ranges = parse_ranges(input);
        let sum: u64 = ranges
            .iter()
            .flat_map(|(start, end)| (*start..=*end).filter(|&id| (self.part1_validator)(id)))
            .sum();
        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let ranges = parse_ranges(input);
        let sum: u64 = ranges
            .iter()
            .flat_map(|(start, end)| (*start..=*end).filter(|&id| (self.part2_validator)(id)))
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

// Modulo-based helper functions for divisibility checking

/// Compute the divisor for a pattern repeated a certain number of times
/// For a pattern of length `pattern_len` repeated `repeat_count` times,
/// the divisor is: sum of 10^(i*pattern_len) for i from 0 to repeat_count-1
/// Example: pattern_len=2, repeat_count=2 -> 10^0 + 10^2 = 1 + 100 = 101
#[inline]
fn compute_divisor(pattern_len: u32, repeat_count: u32) -> u64 {
    let mut divisor = 0u64;
    let multiplier = 10u64.pow(pattern_len);
    let mut power = 1u64;

    for _ in 0..repeat_count {
        divisor += power;
        power *= multiplier;
    }

    divisor
}

/// Check if an ID is invalid (made of a pattern repeated twice) - Modulo-based implementation
fn is_invalid_id_modulo(id: u64) -> bool {
    let digits = count_digits(id);

    // Must have even number of digits
    if digits % 2 != 0 {
        return false;
    }

    // For part 1, check if divisible by 10^(digits/2) + 1
    let divisor = compute_divisor(digits / 2, 2);
    id % divisor == 0
}

/// Check if an ID is invalid (made of a pattern repeated at least twice) - Modulo-based implementation
fn is_invalid_id_v2_modulo(id: u64) -> bool {
    let digits = count_digits(id);

    // Try all possible pattern lengths from 1 to digits/2
    for pattern_len in 1..=(digits / 2) {
        // Check if digits is divisible by pattern_len
        if digits % pattern_len == 0 {
            let repeat_count = digits / pattern_len;
            let divisor = compute_divisor(pattern_len, repeat_count);

            if id % divisor == 0 {
                return true;
            }
        }
    }

    false
}

// Solver instances

/// Solver for Day 2 using math-based validation
#[allow(non_upper_case_globals)]
pub const Day02Math: ValidatingSolver<fn(u64) -> bool, fn(u64) -> bool> =
    ValidatingSolver::new(is_invalid_id, is_invalid_id_v2);

/// Solver for Day 2 using string-based validation
#[allow(non_upper_case_globals)]
pub const Day02String: ValidatingSolver<fn(u64) -> bool, fn(u64) -> bool> =
    ValidatingSolver::new(is_invalid_id_string, is_invalid_id_v2_string);

/// Solver for Day 2 using modulo divisor checking (fastest implementation)
#[allow(non_upper_case_globals)]
pub const Day02Modulo: ValidatingSolver<fn(u64) -> bool, fn(u64) -> bool> =
    ValidatingSolver::new(is_invalid_id_modulo, is_invalid_id_v2_modulo);

/// Default solver for Day 2 (uses modulo divisor checking)
#[allow(non_upper_case_globals)]
pub const Day02: ValidatingSolver<fn(u64) -> bool, fn(u64) -> bool> = Day02Modulo;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    /// Helper to validate ranges with a specific validation function
    fn validate_ranges<F>(input: &str, validator: F) -> String
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
            validate_ranges(EXAMPLE, is_invalid_id),
            "1227775554",
            "Math-based implementation failed"
        );
    }

    #[test]
    fn test_part1_example_string() {
        assert_eq!(
            validate_ranges(EXAMPLE, is_invalid_id_string),
            "1227775554",
            "String-based implementation failed"
        );
    }

    #[test]
    fn test_part2_example_math() {
        assert_eq!(
            validate_ranges(EXAMPLE, is_invalid_id_v2),
            "4174379265",
            "Math-based implementation failed"
        );
    }

    #[test]
    fn test_part2_example_string() {
        assert_eq!(
            validate_ranges(EXAMPLE, is_invalid_id_v2_string),
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

    #[test]
    fn test_part1_example_modulo() {
        assert_eq!(
            validate_ranges(EXAMPLE, is_invalid_id_modulo),
            "1227775554",
            "Modulo-based implementation failed"
        );
    }

    #[test]
    fn test_part2_example_modulo() {
        assert_eq!(
            validate_ranges(EXAMPLE, is_invalid_id_v2_modulo),
            "4174379265",
            "Modulo-based implementation failed"
        );
    }

    #[test]
    fn test_modulo_vs_math_equivalence_part1() {
        for id in [1212, 123456, 9999, 100100, 10, 1234321, 111111] {
            assert_eq!(
                is_invalid_id_modulo(id),
                is_invalid_id(id),
                "Mismatch for ID: {}",
                id
            );
        }
    }

    #[test]
    fn test_modulo_vs_math_equivalence_part2() {
        for id in [121212, 111111, 123123, 9999, 1234, 100100100, 12121212] {
            assert_eq!(
                is_invalid_id_v2_modulo(id),
                is_invalid_id_v2(id),
                "Mismatch for ID: {}",
                id
            );
        }
    }
}

#[cfg(all(test, not(debug_assertions)))]
mod benches {
    extern crate test;
    use super::*;
    use test::Bencher;

    const INPUT: &str = include_str!("input/input.txt");

    // Part 1 benchmarks
    #[bench]
    fn bench_part1_math(b: &mut Bencher) {
        let solver = Day02Math;
        b.iter(|| solver.part1(INPUT));
    }

    #[bench]
    fn bench_part1_string(b: &mut Bencher) {
        let solver = Day02String;
        b.iter(|| solver.part1(INPUT));
    }

    #[bench]
    fn bench_part1_modulo(b: &mut Bencher) {
        let solver = Day02Modulo;
        b.iter(|| solver.part1(INPUT));
    }

    // Part 2 benchmarks
    #[bench]
    fn bench_part2_math(b: &mut Bencher) {
        let solver = Day02Math;
        b.iter(|| solver.part2(INPUT));
    }

    #[bench]
    fn bench_part2_string(b: &mut Bencher) {
        let solver = Day02String;
        b.iter(|| solver.part2(INPUT));
    }

    #[bench]
    fn bench_part2_modulo(b: &mut Bencher) {
        let solver = Day02Modulo;
        b.iter(|| solver.part2(INPUT));
    }

    // Individual validation function benchmarks for part 1
    #[bench]
    fn bench_is_invalid_id_math(b: &mut Bencher) {
        b.iter(|| {
            for id in 1000..2000 {
                test::black_box(is_invalid_id(id));
            }
        });
    }

    #[bench]
    fn bench_is_invalid_id_string(b: &mut Bencher) {
        b.iter(|| {
            for id in 1000..2000 {
                test::black_box(is_invalid_id_string(id));
            }
        });
    }

    // Individual validation function benchmarks for part 2
    #[bench]
    fn bench_is_invalid_id_v2_math(b: &mut Bencher) {
        b.iter(|| {
            for id in 1000..2000 {
                test::black_box(is_invalid_id_v2(id));
            }
        });
    }

    #[bench]
    fn bench_is_invalid_id_v2_string(b: &mut Bencher) {
        b.iter(|| {
            for id in 1000..2000 {
                test::black_box(is_invalid_id_v2_string(id));
            }
        });
    }

    #[bench]
    fn bench_is_invalid_id_modulo(b: &mut Bencher) {
        b.iter(|| {
            for id in 1000..2000 {
                test::black_box(is_invalid_id_modulo(id));
            }
        });
    }

    #[bench]
    fn bench_is_invalid_id_v2_modulo(b: &mut Bencher) {
        b.iter(|| {
            for id in 1000..2000 {
                test::black_box(is_invalid_id_v2_modulo(id));
            }
        });
    }
}
