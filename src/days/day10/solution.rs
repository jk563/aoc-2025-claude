//! Solution implementation for Day 10: Factory Initialization
//!
//! This problem is essentially a "lights-out" puzzle where we need to find
//! the minimum number of button presses to configure indicator lights.
//!
//! ## Algorithm: BFS State Space Search
//!
//! - State: Current light configuration (represented as bitmask)
//! - Start: All lights off (0)
//! - Goal: Target pattern
//! - Actions: Press a button (XOR with button mask)
//!
//! BFS guarantees we find the minimum number of presses.
//!
//! ## Complexity
//! - Time: O(2^n * b) per machine, where n = lights, b = buttons
//! - Space: O(2^n) for visited states
//! - With n ≤ 10, this is very manageable (~1024 states max)

use crate::runner::Day;
use std::collections::{HashSet, VecDeque};

/// Solver for Day 10
pub struct Day10;

impl Day for Day10 {
    fn part1(&self, input: &str) -> String {
        let machines = parse_machines(input);
        let total: usize = machines
            .iter()
            .map(|m| min_presses(m.target, &m.buttons))
            .sum();
        total.to_string()
    }

    fn part2(&self, _input: &str) -> String {
        // Part 2 not yet available
        "0".to_string()
    }
}

/// Represents a machine with its target configuration and buttons
struct Machine {
    target: u32,       // Target light pattern as bitmask
    buttons: Vec<u32>, // Each button as bitmask of lights it toggles
}

/// Parse all machines from input
fn parse_machines(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_machine_line)
        .collect()
}

/// Parse a single machine line
///
/// Format: `[pattern] (button1) (button2) ... {joltage}`
/// - Pattern: `.` = off, `#` = on
/// - Buttons: comma-separated light indices
/// - Joltage: ignored
fn parse_machine_line(line: &str) -> Machine {
    // Extract target pattern between [ and ]
    let pattern_start = line.find('[').unwrap() + 1;
    let pattern_end = line.find(']').unwrap();
    let pattern = &line[pattern_start..pattern_end];

    // Convert pattern to bitmask: '#' = 1, '.' = 0
    let target =
        pattern.chars().enumerate().fold(
            0u32,
            |acc, (i, ch)| {
                if ch == '#' {
                    acc | (1 << i)
                } else {
                    acc
                }
            },
        );

    // Extract all button configurations between ( and )
    let mut buttons = Vec::new();
    let mut remaining = &line[pattern_end + 1..];

    while let Some(button_start) = remaining.find('(') {
        if let Some(button_end) = remaining.find(')') {
            let button_str = &remaining[button_start + 1..button_end];

            // Parse comma-separated indices and convert to bitmask
            let button_mask = button_str
                .split(',')
                .filter_map(|s| s.trim().parse::<u32>().ok())
                .fold(0u32, |acc, idx| acc | (1 << idx));

            buttons.push(button_mask);
            remaining = &remaining[button_end + 1..];
        } else {
            break;
        }
    }

    Machine { target, buttons }
}

/// Find minimum button presses using BFS
///
/// BFS guarantees we find the shortest path (minimum presses) from
/// the start state (all lights off) to the target state.
fn min_presses(target: u32, buttons: &[u32]) -> usize {
    // Edge case: already at target
    if target == 0 {
        return 0;
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // Start state: all lights off (0), 0 presses
    queue.push_back((0u32, 0usize));
    visited.insert(0);

    while let Some((state, presses)) = queue.pop_front() {
        // Try pressing each button
        for &button in buttons {
            let next_state = state ^ button; // XOR toggles the lights

            if next_state == target {
                return presses + 1;
            }

            // Only visit each state once
            if visited.insert(next_state) {
                queue.push_back((next_state, presses + 1));
            }
        }
    }

    unreachable!("No solution found for target: {}", target)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example from puzzle
    const EXAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1_example() {
        let day = Day10;
        assert_eq!(day.part1(EXAMPLE), "7");
    }

    #[test]
    fn test_parse_pattern() {
        let line = "[.##.] (3) {1,2,3}";
        let machine = parse_machine_line(line);
        // Pattern [.##.] = 0b0110 = 6
        assert_eq!(machine.target, 0b0110);
    }

    #[test]
    fn test_parse_buttons() {
        let line = "[.##.] (3) (1,3) (0,2) {1,2,3}";
        let machine = parse_machine_line(line);
        // Button (3) = 0b1000 = 8
        // Button (1,3) = 0b1010 = 10
        // Button (0,2) = 0b0101 = 5
        assert_eq!(machine.buttons, vec![0b1000, 0b1010, 0b0101]);
    }

    #[test]
    fn test_machine_1() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_machine_line(line);
        let presses = min_presses(machine.target, &machine.buttons);
        assert_eq!(presses, 2);
    }

    #[test]
    fn test_machine_2() {
        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = parse_machine_line(line);
        let presses = min_presses(machine.target, &machine.buttons);
        assert_eq!(presses, 3);
    }

    #[test]
    fn test_machine_3() {
        let line = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = parse_machine_line(line);
        let presses = min_presses(machine.target, &machine.buttons);
        assert_eq!(presses, 2);
    }

    #[test]
    #[ignore] // Remove this when Part 2 is unlocked
    fn test_part2_example() {
        let day = Day10;
        assert_eq!(day.part2(EXAMPLE), "0");
    }
}

// Define benchmarks using the common macro
crate::define_day_benches!(Day10);
