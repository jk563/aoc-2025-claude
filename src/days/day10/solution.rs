//! Solution implementation for Day 10

use crate::runner::Day;

/// Solver for Day 10
pub struct Day10;

impl Day for Day10 {
    fn part1(&self, input: &str) -> String {
        // TODO: Implement Part 1
        todo!("Implement part1")
    }

    fn part2(&self, input: &str) -> String {
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
    fn test_part1_example() {
        let day = Day10;
        assert_eq!(day.part1(EXAMPLE), "TODO");
    }

    #[test]
    #[ignore] // Remove this when Part 2 is unlocked
    fn test_part2_example() {
        let day = Day10;
        assert_eq!(day.part2(EXAMPLE), "TODO");
    }
}

// Define benchmarks using the common macro
crate::define_day_benches!(Day10);
