//! Solution implementation for Day 9

use crate::runner::Day;

/// Solver for Day 9
pub struct Day09;

impl Day for Day09 {
    fn part1(&self, _input: &str) -> String {
        // TODO: Implement Part 1
        String::from("Not yet implemented")
    }

    fn part2(&self, _input: &str) -> String {
        // TODO: Implement Part 2
        String::from("Not yet implemented")
    }
}

// Helper functions

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
TODO: Add example input from puzzle";

    #[test]
    #[ignore] // Remove this when implementing Part 1
    fn test_part1_example() {
        let day = Day09;
        assert_eq!(day.part1(EXAMPLE), "TODO");
    }

    #[test]
    #[ignore] // Remove this when Part 2 is unlocked
    fn test_part2_example() {
        let day = Day09;
        assert_eq!(day.part2(EXAMPLE), "TODO");
    }
}

// Define benchmarks using the common macro
crate::define_day_benches!(Day09);
