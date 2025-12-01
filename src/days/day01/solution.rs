//! Solution implementation for Day 1: Secret Entrance
//!
//! This puzzle involves simulating a safe dial with 100 positions (0-99).
//! We start at position 50 and follow a series of rotations (L for left, R for right).
//! The dial wraps around circularly.
//!
//! Part 1: Count how many times the dial lands on position 0 after any rotation.
//! Part 2: Count how many times the dial passes through 0 during any rotation (including at the end).

use crate::runner::Day;

/// Solver for Day 1
pub struct Day01;

impl Day for Day01 {
    fn part1(&self, input: &str) -> String {
        count_zeros(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        count_zeros_during_rotations(input).to_string()
    }
}

// Helper functions

/// Parse and execute rotations, counting how many times we land on 0
fn count_zeros(input: &str) -> usize {
    const DIAL_SIZE: i32 = 100;
    const START_POS: i32 = 50;

    let mut position = START_POS;
    let mut zero_count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse direction (first char) and distance (rest)
        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().unwrap();

        // Apply rotation
        position = match direction {
            'L' => (position - distance).rem_euclid(DIAL_SIZE),
            'R' => (position + distance).rem_euclid(DIAL_SIZE),
            _ => panic!("Invalid direction: {}", direction),
        };

        // Count if we landed on 0
        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

/// Count how many times the dial passes through 0 during all rotations.
/// This includes both landing on 0 at the end of a rotation AND passing through 0 during the rotation.
///
/// For a right rotation from position `start` by `distance`:
///   - We pass through 0: floor((start + distance) / 100) - floor(start / 100) times
///
/// For a left rotation from position `start` by `distance`:
///   - If start == 0: floor(distance / 100) times (we only hit 0 at multiples of 100 steps)
///   - If start > 0 and distance >= start: 1 + floor((distance - start) / 100) times
///   - If start > 0 and distance < start: 0 times
fn count_zeros_during_rotations(input: &str) -> usize {
    const DIAL_SIZE: i32 = 100;
    const START_POS: i32 = 50;

    let mut position = START_POS;
    let mut zero_count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().unwrap();

        // Count zeros crossed during this rotation
        zero_count += match direction {
            'L' => {
                if position == 0 {
                    // Starting at 0, going left: hit 0 every 100 clicks
                    distance / DIAL_SIZE
                } else if distance >= position {
                    // We'll hit 0 after 'position' clicks, then every 100 after that
                    1 + (distance - position) / DIAL_SIZE
                } else {
                    // We don't reach 0
                    0
                }
            }
            'R' => {
                // Going right: count how many multiples of 100 we cross
                (position + distance) / DIAL_SIZE - position / DIAL_SIZE
            }
            _ => panic!("Invalid direction: {}", direction),
        };

        // Update position
        position = match direction {
            'L' => (position - distance).rem_euclid(DIAL_SIZE),
            'R' => (position + distance).rem_euclid(DIAL_SIZE),
            _ => panic!("Invalid direction: {}", direction),
        };
    }

    zero_count as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1_example() {
        let day = Day01;
        assert_eq!(day.part1(EXAMPLE), "3");
    }

    #[test]
    fn test_rotations() {
        // Test individual rotations from the example
        let mut pos: i32 = 50;

        // L68: 50 - 68 = -18 -> 82
        pos = (pos - 68).rem_euclid(100);
        assert_eq!(pos, 82);

        // L30: 82 - 30 = 52
        pos = (pos - 30).rem_euclid(100);
        assert_eq!(pos, 52);

        // R48: 52 + 48 = 100 -> 0
        pos = (pos + 48).rem_euclid(100);
        assert_eq!(pos, 0);

        // L5: 0 - 5 = -5 -> 95
        pos = (pos - 5).rem_euclid(100);
        assert_eq!(pos, 95);

        // R60: 95 + 60 = 155 -> 55
        pos = (pos + 60).rem_euclid(100);
        assert_eq!(pos, 55);

        // L55: 55 - 55 = 0
        pos = (pos - 55).rem_euclid(100);
        assert_eq!(pos, 0);
    }

    #[test]
    fn test_part2_example() {
        let day = Day01;
        assert_eq!(day.part2(EXAMPLE), "6");
    }

    #[test]
    fn test_part2_rotations() {
        // Test the example step by step
        // Start at 50, L68 -> 82, crosses 0 once
        let mut pos: i32 = 50;
        let mut count = 0;

        // L68 from 50
        let distance = 68;
        if distance >= pos {
            count += 1 + (distance - pos) / 100;
        }
        pos = (pos - 68).rem_euclid(100);
        assert_eq!(pos, 82);
        assert_eq!(count, 1);

        // L30 from 82 -> 52, no crossing
        let distance = 30;
        if distance >= pos {
            count += 1 + (distance - pos) / 100;
        }
        pos = (pos - 30).rem_euclid(100);
        assert_eq!(pos, 52);
        assert_eq!(count, 1);

        // R48 from 52 -> 0, crosses 0 once
        let distance = 48;
        count += (pos + distance) / 100 - pos / 100;
        pos = (pos + 48).rem_euclid(100);
        assert_eq!(pos, 0);
        assert_eq!(count, 2);

        // L5 from 0 -> 95, no crossing (special case: starting at 0)
        let distance = 5;
        count += distance / 100; // 0 times
        pos = (pos - 5).rem_euclid(100);
        assert_eq!(pos, 95);
        assert_eq!(count, 2);

        // R60 from 95 -> 55, crosses 0 once
        let distance = 60;
        count += (pos + distance) / 100 - pos / 100;
        pos = (pos + 60).rem_euclid(100);
        assert_eq!(pos, 55);
        assert_eq!(count, 3);

        // L55 from 55 -> 0, crosses 0 once
        let distance = 55;
        if distance >= pos {
            count += 1 + (distance - pos) / 100;
        }
        pos = (pos - 55).rem_euclid(100);
        assert_eq!(pos, 0);
        assert_eq!(count, 4);

        // L1 from 0 -> 99, no crossing
        let distance = 1;
        count += distance / 100;
        pos = (pos - 1).rem_euclid(100);
        assert_eq!(pos, 99);
        assert_eq!(count, 4);

        // L99 from 99 -> 0, crosses 0 once
        let distance = 99;
        if distance >= pos {
            count += 1 + (distance - pos) / 100;
        }
        pos = (pos - 99).rem_euclid(100);
        assert_eq!(pos, 0);
        assert_eq!(count, 5);

        // R14 from 0 -> 14, no crossing
        let distance = 14;
        count += (pos + distance) / 100 - pos / 100;
        pos = (pos + 14).rem_euclid(100);
        assert_eq!(pos, 14);
        assert_eq!(count, 5);

        // L82 from 14 -> 32, crosses 0 once
        let distance = 82;
        if distance >= pos {
            count += 1 + (distance - pos) / 100;
        }
        pos = (pos - 82).rem_euclid(100);
        assert_eq!(pos, 32);
        assert_eq!(count, 6);
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
        let day = Day01;
        b.iter(|| day.part1(input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = include_str!("input/input.txt");
        let day = Day01;
        b.iter(|| day.part2(input));
    }
}
