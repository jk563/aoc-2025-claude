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

/// Check if an ID is invalid (made of a pattern repeated twice)
fn is_invalid_id(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    // Must have even number of digits to be splittable
    if len % 2 != 0 {
        return false;
    }

    // Split in half and check if both halves are equal
    let mid = len / 2;
    let first_half = &s[..mid];
    let second_half = &s[mid..];

    first_half == second_half
}

/// Check if an ID is invalid (made of a pattern repeated at least twice)
fn is_invalid_id_v2(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=(len / 2) {
        // Check if the string length is divisible by pattern length
        if len % pattern_len == 0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1_example() {
        let day = Day02;
        assert_eq!(day.part1(EXAMPLE), "1227775554");
    }

    #[test]
    fn test_part2_example() {
        let day = Day02;
        assert_eq!(day.part2(EXAMPLE), "4174379265");
    }
}

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = include_str!("input/input.txt");
        let day = Day02;
        b.iter(|| day.part1(input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = include_str!("input/input.txt");
        let day = Day02;
        b.iter(|| day.part2(input));
    }
}
