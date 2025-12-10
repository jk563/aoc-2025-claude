//! Solution implementation for Day 10: Factory Initialization
//!
//! ## Part 1: Lights-Out Puzzle
//! Configure indicator lights by toggling them with buttons.
//! Uses BFS on bitmask states (XOR operations).
//!
//! ## Part 2: Joltage Counter Problem
//! Configure numeric counters by incrementing them with buttons.
//! Uses BFS on counter states (addition operations).
//!
//! Both parts use BFS to guarantee minimum button presses.
//!
//! ## Complexity
//! - Part 1: O(2^n * b) where n = lights, b = buttons
//! - Part 2: O(product(targets) * b) but heavily pruned

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

    fn part2(&self, input: &str) -> String {
        let machines = parse_machines_part2(input);
        let total: usize = machines
            .iter()
            .map(|m| min_presses_part2(&m.targets, &m.buttons))
            .sum();
        total.to_string()
    }
}

/// Represents a machine with its target configuration and buttons (Part 1)
struct Machine {
    target: u32,       // Target light pattern as bitmask
    buttons: Vec<u32>, // Each button as bitmask of lights it toggles
}

/// Represents a machine for Part 2 with joltage counters
struct MachinePart2 {
    targets: Vec<u32>,        // Target joltage values for each counter
    buttons: Vec<Vec<usize>>, // Each button lists which counters it increments
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

//=============================================================================
// Part 2: Joltage Counter Problem
//=============================================================================

/// Parse all machines for Part 2 (joltage counters)
fn parse_machines_part2(input: &str) -> Vec<MachinePart2> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_machine_line_part2)
        .collect()
}

/// Parse a single machine line for Part 2
///
/// Format: `[pattern] (button1) (button2) ... {joltage1,joltage2,...}`
/// - Pattern: ignored in Part 2
/// - Buttons: comma-separated counter indices
/// - Joltage: target values for each counter
fn parse_machine_line_part2(line: &str) -> MachinePart2 {
    // Extract button configurations between ( and )
    let mut buttons = Vec::new();
    let mut pos = 0;

    while let Some(button_start) = line[pos..].find('(') {
        let button_start = pos + button_start;
        if let Some(button_end) = line[button_start..].find(')') {
            let button_end = button_start + button_end;
            let button_str = &line[button_start + 1..button_end];

            // Parse comma-separated counter indices
            let counters: Vec<usize> = button_str
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            buttons.push(counters);
            pos = button_end + 1;
        } else {
            break;
        }
    }

    // Extract joltage requirements between { and }
    let joltage_start = line.find('{').unwrap() + 1;
    let joltage_end = line.find('}').unwrap();
    let joltage_str = &line[joltage_start..joltage_end];

    let targets: Vec<u32> = joltage_str
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    MachinePart2 { targets, buttons }
}

/// Find minimum button presses for Part 2 using BFS
///
/// This is a multi-dimensional counter problem where each button press
/// increments specific counters by 1. We use BFS with aggressive pruning
/// to find the minimum presses needed to reach target values.
fn min_presses_part2(targets: &[u32], buttons: &[Vec<usize>]) -> usize {
    // Edge case: all targets already at zero
    if targets.iter().all(|&t| t == 0) {
        return 0;
    }

    let n = targets.len();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // Start state: all counters at zero
    let start = vec![0u32; n];
    queue.push_back((start.clone(), 0usize));
    visited.insert(start);

    while let Some((state, presses)) = queue.pop_front() {
        // Check if we've reached the target
        if state.iter().zip(targets).all(|(a, b)| a == b) {
            return presses;
        }

        // Try pressing each button
        for button in buttons {
            let mut next_state = state.clone();
            let mut valid = true;

            // Increment counters affected by this button
            for &idx in button {
                if idx < n {
                    next_state[idx] += 1;
                    // Prune: don't explore states where any counter exceeds target
                    if next_state[idx] > targets[idx] {
                        valid = false;
                        break;
                    }
                }
            }

            // Only explore valid, unvisited states
            if valid && visited.insert(next_state.clone()) {
                queue.push_back((next_state, presses + 1));
            }
        }
    }

    unreachable!("No solution found for targets: {:?}", targets)
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
    fn test_part2_example() {
        let day = Day10;
        assert_eq!(day.part2(EXAMPLE), "33");
    }

    #[test]
    fn test_machine_1_part2() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_machine_line_part2(line);
        let presses = min_presses_part2(&machine.targets, &machine.buttons);
        assert_eq!(presses, 10);
    }

    #[test]
    fn test_machine_2_part2() {
        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = parse_machine_line_part2(line);
        let presses = min_presses_part2(&machine.targets, &machine.buttons);
        assert_eq!(presses, 12);
    }

    #[test]
    fn test_machine_3_part2() {
        let line = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = parse_machine_line_part2(line);
        let presses = min_presses_part2(&machine.targets, &machine.buttons);
        assert_eq!(presses, 11);
    }

    #[test]
    fn test_parse_joltage() {
        let line = "[.##.] (3) (1,3) {3,5,4,7}";
        let machine = parse_machine_line_part2(line);
        assert_eq!(machine.targets, vec![3, 5, 4, 7]);
        assert_eq!(machine.buttons, vec![vec![3], vec![1, 3]]);
    }
}

// Define benchmarks using the common macro
crate::define_day_benches!(Day10);
