//! Solution implementation for Day 4

use crate::runner::Day;

/// Solver for Day 4
pub struct Day04;

impl Day for Day04 {
    fn part1(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let rows = grid.len();
        let cols = grid[0].len();

        let mut accessible = 0;

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == '@' {
                    let adjacent_count = count_adjacent_rolls(&grid, row, col);
                    if adjacent_count < 4 {
                        accessible += 1;
                    }
                }
            }
        }

        accessible.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut total_removed = 0;

        loop {
            // Find all accessible rolls
            let mut to_remove = Vec::new();
            let rows = grid.len();
            let cols = grid[0].len();

            for row in 0..rows {
                for col in 0..cols {
                    if grid[row][col] == '@' {
                        let adjacent_count = count_adjacent_rolls(&grid, row, col);
                        if adjacent_count < 4 {
                            to_remove.push((row, col));
                        }
                    }
                }
            }

            // If no more rolls to remove, we're done
            if to_remove.is_empty() {
                break;
            }

            // Remove all accessible rolls
            for (row, col) in &to_remove {
                grid[*row][*col] = '.';
            }

            total_removed += to_remove.len();
        }

        total_removed.to_string()
    }
}

// Helper functions

/// Count how many adjacent positions contain rolls of paper
fn count_adjacent_rolls(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Check all 8 adjacent positions
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue; // Skip the current position
            }

            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                if grid[new_row as usize][new_col as usize] == '@' {
                    count += 1;
                }
            }
        }
    }

    count
}

// Optimized implementation using flat Vec<u8>

/// Optimized grid representation using flat byte array
struct Grid {
    data: Vec<u8>,
    cols: usize,
    rows: usize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let rows = lines.len();
        let cols = lines[0].len();
        let data = input.bytes().filter(|&b| b != b'\n').collect();

        Self { data, cols, rows }
    }

    #[inline]
    fn get(&self, idx: usize) -> u8 {
        self.data[idx]
    }

    #[inline]
    fn set(&mut self, idx: usize, val: u8) {
        self.data[idx] = val;
    }

    /// Count adjacent rolls using manual unrolling for performance
    fn count_adjacent(&self, idx: usize) -> usize {
        let row = idx / self.cols;
        let col = idx % self.cols;
        let mut count = 0;

        // Top row
        if row > 0 {
            let above = idx - self.cols;
            if col > 0 && self.data[above - 1] == b'@' { count += 1; }
            if self.data[above] == b'@' { count += 1; }
            if col < self.cols - 1 && self.data[above + 1] == b'@' { count += 1; }
        }

        // Same row
        if col > 0 && self.data[idx - 1] == b'@' { count += 1; }
        if col < self.cols - 1 && self.data[idx + 1] == b'@' { count += 1; }

        // Bottom row
        if row < self.rows - 1 {
            let below = idx + self.cols;
            if col > 0 && self.data[below - 1] == b'@' { count += 1; }
            if self.data[below] == b'@' { count += 1; }
            if col < self.cols - 1 && self.data[below + 1] == b'@' { count += 1; }
        }

        count
    }

    /// Get all valid neighbor indices for a position
    fn neighbors(&self, idx: usize) -> impl Iterator<Item = usize> + '_ {
        let row = idx / self.cols;
        let col = idx % self.cols;
        let mut neighbors = Vec::with_capacity(8);

        // Top row
        if row > 0 {
            let above = idx - self.cols;
            if col > 0 { neighbors.push(above - 1); }
            neighbors.push(above);
            if col < self.cols - 1 { neighbors.push(above + 1); }
        }

        // Same row
        if col > 0 { neighbors.push(idx - 1); }
        if col < self.cols - 1 { neighbors.push(idx + 1); }

        // Bottom row
        if row < self.rows - 1 {
            let below = idx + self.cols;
            if col > 0 { neighbors.push(below - 1); }
            neighbors.push(below);
            if col < self.cols - 1 { neighbors.push(below + 1); }
        }

        neighbors.into_iter()
    }
}

/// Optimized solver for Day 4 using flat arrays and dirty set tracking
pub struct Day04Optimized;

impl Day for Day04Optimized {
    fn part1(&self, input: &str) -> String {
        let grid = Grid::parse(input);
        let accessible = (0..grid.data.len())
            .filter(|&idx| grid.get(idx) == b'@' && grid.count_adjacent(idx) < 4)
            .count();

        accessible.to_string()
    }

    fn part2(&self, input: &str) -> String {
        use std::collections::HashSet;

        let mut grid = Grid::parse(input);
        let mut total_removed = 0;

        // Start by checking all positions with '@'
        let mut to_check: HashSet<usize> = (0..grid.data.len())
            .filter(|&idx| grid.get(idx) == b'@')
            .collect();

        // Reuse vectors to avoid allocations
        let mut accessible = Vec::new();
        let mut next_check = HashSet::new();

        loop {
            accessible.clear();

            // Only check candidate positions
            for &idx in &to_check {
                if grid.get(idx) == b'@' && grid.count_adjacent(idx) < 4 {
                    accessible.push(idx);
                }
            }

            if accessible.is_empty() {
                break;
            }

            // Remove accessible rolls and mark neighbors for next check
            next_check.clear();
            for &idx in &accessible {
                grid.set(idx, b'.');

                // Add neighbors to next check set
                for neighbor in grid.neighbors(idx) {
                    if grid.get(neighbor) == b'@' {
                        next_check.insert(neighbor);
                    }
                }
            }

            total_removed += accessible.len();
            std::mem::swap(&mut to_check, &mut next_check);
        }

        total_removed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1_example() {
        let day = Day04;
        assert_eq!(day.part1(EXAMPLE), "13");
    }

    #[test]
    fn test_part2_example() {
        let day = Day04;
        assert_eq!(day.part2(EXAMPLE), "43");
    }

    #[test]
    fn test_part1_example_optimized() {
        let day = Day04Optimized;
        assert_eq!(day.part1(EXAMPLE), "13");
    }

    #[test]
    fn test_part2_example_optimized() {
        let day = Day04Optimized;
        assert_eq!(day.part2(EXAMPLE), "43");
    }
}

// Define benchmarks using the common macro
crate::define_day_benches!(Day04);
