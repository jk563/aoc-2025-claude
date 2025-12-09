//! Solution implementation for Day 9

use crate::runner::Day;

/// Solver for Day 9
pub struct Day09;

impl Day for Day09 {
    fn part1(&self, input: &str) -> String {
        let tiles = parse_tiles(input);
        let max_area = find_largest_rectangle(&tiles);
        max_area.to_string()
    }

    fn part2(&self, _input: &str) -> String {
        // TODO: Implement Part 2
        String::from("Not yet implemented")
    }
}

// Helper functions

/// Parse the input to extract tile coordinates
fn parse_tiles(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().trim().parse().unwrap();
            let y = parts.next().unwrap().trim().parse().unwrap();
            (x, y)
        })
        .collect()
}

/// Find the largest rectangle area using any two tiles as opposite corners
fn find_largest_rectangle(tiles: &[(i32, i32)]) -> i64 {
    let mut max_area = 0i64;

    // Check all pairs of tiles
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];

            // Calculate rectangle area (inclusive bounds)
            let width = (x2 - x1).abs() as i64 + 1;
            let height = (y2 - y1).abs() as i64 + 1;
            let area = width * height;

            max_area = max_area.max(area);
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1_example() {
        let day = Day09;
        assert_eq!(day.part1(EXAMPLE), "50");
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
