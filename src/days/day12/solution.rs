//! Solution implementation for Day 12

use crate::runner::Day;

/// Solver for Day 12
pub struct Day12;

impl Day for Day12 {
    fn part1(&self, _input: &str) -> String {
        // TODO: Implement Part 1
        todo!("Implement part1")
    }

    fn part2(&self, _input: &str) -> String {
        // TODO: Implement Part 2
        todo!("Implement part2")
    }
}

// Helper functions

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
TODO: Add example input from puzzle";

    #[test]
    #[ignore] // Remove this when implementation is ready
    fn test_part1_example() {
        let day = Day12;
        assert_eq!(day.part1(EXAMPLE), "TODO");
    }

    #[test]
    #[ignore] // Remove this when Part 2 is unlocked
    fn test_part2_example() {
        let day = Day12;
        assert_eq!(day.part2(EXAMPLE), "TODO");
    }
}

// Define benchmarks using the common macro
crate::define_day_benches!(Day12);
