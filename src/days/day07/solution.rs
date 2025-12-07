//! Solution implementation for Day 7
//!
//! Part 1: Tachyon beam splitter simulation. Beams merge at same column. Count splits.
//! Part 2: Quantum timeline counting. Particles don't merge. Count distinct timelines.

use crate::runner::Day;
use std::collections::{HashMap, HashSet};

/// Solver for Day 7
pub struct Day07;

impl Day for Day07 {
    fn part1(&self, input: &str) -> String {
        count_splits(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        count_timelines(input).to_string()
    }
}

fn count_splits(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Find starting position
    let (start_row, start_col) = find_start(&lines, width).unwrap_or_default();

    // Build splitter positions per row
    let splitter_map = build_splitter_map(&lines, width);

    // Simulate beam propagation
    let mut active_beams: HashSet<usize> = HashSet::new();
    active_beams.insert(start_col);
    let mut split_count = 0;

    for splitters in splitter_map.iter().skip(start_row + 1) {
        if active_beams.is_empty() {
            break;
        }

        if splitters.is_empty() {
            continue;
        }

        // Find beams that hit splitters
        let hits: Vec<usize> = active_beams.intersection(splitters).copied().collect();

        split_count += hits.len();

        // Update beams: remove hits, add left+right neighbors
        for col in hits {
            active_beams.remove(&col);
            if col > 0 {
                active_beams.insert(col - 1);
            }
            if col < width - 1 {
                active_beams.insert(col + 1);
            }
        }
    }

    split_count
}

fn count_timelines(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Find starting position
    let (start_row, start_col) = find_start(&lines, width).unwrap_or_default();

    // Build splitter positions per row
    let splitter_map = build_splitter_map(&lines, width);

    // Simulate particle propagation with timeline counting
    let mut particle_counts: HashMap<usize, u64> = HashMap::new();
    particle_counts.insert(start_col, 1);

    for splitters in splitter_map.iter().skip(start_row + 1) {
        if particle_counts.is_empty() {
            break;
        }

        if splitters.is_empty() {
            continue;
        }

        let mut new_counts: HashMap<usize, u64> = HashMap::new();

        // Process each particle at each column
        for (col, count) in particle_counts.iter() {
            if splitters.contains(col) {
                // Split: particles go left AND right (both timelines)
                if *col > 0 {
                    *new_counts.entry(col - 1).or_default() += count;
                }
                if *col < width - 1 {
                    *new_counts.entry(col + 1).or_default() += count;
                }
            } else {
                // Pass through: particles continue down at same column
                *new_counts.entry(*col).or_default() += count;
            }
        }

        particle_counts = new_counts;
    }

    // Sum all timelines
    particle_counts.values().sum()
}

fn find_start(lines: &[&str], _width: usize) -> Option<(usize, usize)> {
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                return Some((row, col));
            }
        }
    }
    None
}

fn build_splitter_map(lines: &[&str], _width: usize) -> Vec<HashSet<usize>> {
    let mut splitter_map = vec![HashSet::new(); lines.len()];

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '^' {
                splitter_map[row].insert(col);
            }
        }
    }

    splitter_map
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1_example() {
        let day = Day07;
        assert_eq!(day.part1(EXAMPLE), "21");
    }

    #[test]
    fn test_part2_example() {
        let day = Day07;
        assert_eq!(day.part2(EXAMPLE), "40");
    }
}

// Define benchmarks using the common macro
crate::define_day_benches!(Day07);
