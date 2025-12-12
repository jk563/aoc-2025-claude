//! Solution implementation for Day 12: Christmas Tree Farm
//!
//! ## Part 1
//! 2D bin packing problem with polyominoes (tetris-like shapes).
//! Uses backtracking with constraint propagation to determine which regions
//! can fit all their required shapes.
//!
//! Algorithm:
//! 1. Parse 6 polyomino shapes and 1000+ regions from input
//! 2. Precompute all unique transformations (rotations/flips) for each shape
//! 3. For each region, use backtracking to attempt placement
//! 4. Count regions that successfully pack all shapes
//!
//! Optimizations:
//! - MRV heuristic: Place shapes with fewer variants first
//! - Early rejection: Check area feasibility before backtracking
//! - Bounding box pruning: Skip variants that exceed grid dimensions
//! - First solution termination: Don't enumerate all packings
//! - Flat grid layout: Vec<u8> with row-major indexing for cache locality
//!
//! Time: O(regions × W × H × V × branches) with heavy pruning
//! Space: O(W × H) for grid + O(shapes × variants) for cache
//!
//! ## Part 2
//! No Part 2 for this day

use crate::runner::Day;
use std::collections::HashSet;

/// Solver for Day 12
pub struct Day12;

impl Day for Day12 {
    fn part1(&self, input: &str) -> String {
        let (transforms, regions) = parse_input(input);
        let count = regions
            .iter()
            .filter(|region| can_fit_all_shapes(region, &transforms))
            .count();
        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        // No Part 2 for Day 12 - return same result as Part 1
        self.part1(input)
    }
}

// Data Structures

/// A polyomino shape with normalized coordinates
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Polyomino {
    cells: Vec<(i8, i8)>,
    width: i8,
    height: i8,
}

/// A transformed variant of a shape
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ShapeVariant {
    cells: Vec<(i8, i8)>,
    width: i8,
    height: i8,
}

/// Cache of all unique transformations for each shape
type TransformCache = Vec<Vec<ShapeVariant>>;

/// Grid for tracking occupied cells
struct Grid {
    occupied: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            occupied: vec![0u8; width * height],
            width,
            height,
        }
    }
}

/// A region specification
struct Region {
    width: usize,
    height: usize,
    required_counts: [usize; 6],
}

// Parsing

fn parse_input(input: &str) -> (TransformCache, Vec<Region>) {
    let lines: Vec<&str> = input.lines().collect();

    // Parse 6 shapes (each is 4 lines: header + 3 grid lines, separated by blank lines)
    let mut shapes = Vec::with_capacity(6);
    let mut line_idx = 0;

    for _ in 0..6 {
        // Skip header line (e.g., "0:")
        line_idx += 1;

        // Parse 3 lines of grid
        let grid_lines = &lines[line_idx..line_idx + 3];
        shapes.push(parse_shape(grid_lines));
        line_idx += 3;

        // Skip blank line (if present)
        if line_idx < lines.len() && lines[line_idx].trim().is_empty() {
            line_idx += 1;
        }
    }

    // Precompute all transformations
    let transforms = precompute_transforms(&shapes);

    // Parse regions (remaining lines)
    let regions = lines[line_idx..]
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_region(line))
        .collect();

    (transforms, regions)
}

fn parse_shape(lines: &[&str]) -> Polyomino {
    let mut cells = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                cells.push((x as i8, y as i8));
            }
        }
    }

    // Normalize to (0, 0) origin
    if !cells.is_empty() {
        let min_x = cells.iter().map(|&(x, _)| x).min().unwrap();
        let min_y = cells.iter().map(|&(_, y)| y).min().unwrap();

        for cell in &mut cells {
            cell.0 -= min_x;
            cell.1 -= min_y;
        }
    }

    let width = cells.iter().map(|&(x, _)| x).max().unwrap_or(0) + 1;
    let height = cells.iter().map(|&(_, y)| y).max().unwrap_or(0) + 1;

    Polyomino {
        cells,
        width,
        height,
    }
}

fn parse_region(line: &str) -> Region {
    // Format: "50x44: 49 45 27 41 32 30"
    let (dims, counts) = line.split_once(": ").unwrap();
    let (w, h) = dims.split_once('x').unwrap();

    let mut required_counts = [0; 6];
    for (i, count_str) in counts.split_whitespace().enumerate() {
        required_counts[i] = count_str.parse().unwrap();
    }

    Region {
        width: w.parse().unwrap(),
        height: h.parse().unwrap(),
        required_counts,
    }
}

// Transformation Generation

fn precompute_transforms(shapes: &[Polyomino]) -> TransformCache {
    shapes
        .iter()
        .map(|shape| {
            let mut variants = HashSet::new();

            // Try both non-flipped and flipped versions
            for flip in [false, true] {
                let mut current = shape.cells.clone();

                if flip {
                    // Horizontal flip: negate x coordinates
                    for cell in &mut current {
                        cell.0 = -cell.0;
                    }
                }

                // Try 4 rotations: 0°, 90°, 180°, 270°
                for _ in 0..4 {
                    // Normalize and add to set
                    let normalized = normalize_cells(&current);
                    variants.insert(normalized);

                    // Rotate 90° clockwise: (x, y) → (y, -x)
                    current = current.iter().map(|&(x, y)| (y, -x)).collect();
                }
            }

            // Convert HashSet to Vec of ShapeVariants
            variants
                .into_iter()
                .map(|cells| {
                    let width = cells.iter().map(|&(x, _)| x).max().unwrap_or(0) + 1;
                    let height = cells.iter().map(|&(_, y)| y).max().unwrap_or(0) + 1;
                    ShapeVariant {
                        cells,
                        width,
                        height,
                    }
                })
                .collect()
        })
        .collect()
}

fn normalize_cells(cells: &[(i8, i8)]) -> Vec<(i8, i8)> {
    if cells.is_empty() {
        return Vec::new();
    }

    let min_x = cells.iter().map(|&(x, _)| x).min().unwrap();
    let min_y = cells.iter().map(|&(_, y)| y).min().unwrap();

    let mut normalized: Vec<_> = cells.iter().map(|(x, y)| (x - min_x, y - min_y)).collect();

    normalized.sort_unstable(); // For HashSet deduplication
    normalized
}

// Grid Operations

fn can_place(grid: &Grid, variant: &ShapeVariant, base_x: usize, base_y: usize) -> bool {
    for &(dx, dy) in &variant.cells {
        let x = base_x + dx as usize;
        let y = base_y + dy as usize;

        // Check bounds
        if x >= grid.width || y >= grid.height {
            return false;
        }

        // Check occupancy with flat indexing
        let idx = y * grid.width + x;
        if grid.occupied[idx] != 0 {
            return false;
        }
    }
    true
}

fn place(grid: &mut Grid, variant: &ShapeVariant, base_x: usize, base_y: usize, occupy: bool) {
    let val = if occupy { 1u8 } else { 0u8 };
    for &(dx, dy) in &variant.cells {
        let x = base_x + dx as usize;
        let y = base_y + dy as usize;
        let idx = y * grid.width + x;
        grid.occupied[idx] = val;
    }
}

// Backtracking Algorithm

fn can_fit_all_shapes(region: &Region, transforms: &TransformCache) -> bool {
    // Build placement list (flattened: each shape repeated by its count)
    let mut placements = Vec::new();
    for (shape_id, &count) in region.required_counts.iter().enumerate() {
        for _ in 0..count {
            placements.push(shape_id);
        }
    }

    // Early rejection: Check if total area is feasible
    let total_cells: usize = placements
        .iter()
        .map(|&shape_id| transforms[shape_id][0].cells.len())
        .sum();

    if total_cells > region.width * region.height {
        return false;
    }

    // Sort by MRV heuristic: shapes with fewer variants first (more constrained)
    placements.sort_by_key(|&shape_id| transforms[shape_id].len());

    // Initialize empty grid and attempt backtracking
    let mut grid = Grid::new(region.width, region.height);
    backtrack(&mut grid, &placements, 0, transforms)
}

fn backtrack(
    grid: &mut Grid,
    placements: &[usize],
    idx: usize,
    transforms: &TransformCache,
) -> bool {
    // Base case: all shapes placed successfully
    if idx == placements.len() {
        return true;
    }

    let shape_id = placements[idx];

    // Try each transformation variant
    for variant in &transforms[shape_id] {
        // Early rejection: variant too large for grid
        if variant.width as usize > grid.width || variant.height as usize > grid.height {
            continue;
        }

        // Try placing at each position (top-left scan order)
        for y in 0..=(grid.height - variant.height as usize) {
            for x in 0..=(grid.width - variant.width as usize) {
                if can_place(grid, variant, x, y) {
                    // Place shape
                    place(grid, variant, x, y, true);

                    // Recurse to next shape
                    if backtrack(grid, placements, idx + 1, transforms) {
                        return true; // Early termination on first valid solution
                    }

                    // Backtrack: remove placement
                    place(grid, variant, x, y, false);
                }
            }
        }
    }

    false
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part1_example() {
        let day = Day12;
        assert_eq!(day.part1(EXAMPLE), "2");
    }

    #[test]
    fn test_parse_shape() {
        let lines = ["###", "#.#", "#.#"];
        let shape = parse_shape(&lines);
        assert_eq!(shape.cells.len(), 7); // 7 '#' cells
        assert_eq!(shape.width, 3);
        assert_eq!(shape.height, 3);
        // Check normalization (should start at 0,0)
        assert!(shape.cells.contains(&(0, 0)));
    }

    #[test]
    fn test_normalize_cells() {
        let cells = vec![(-2, -1), (-1, -1), (0, -1)];
        let normalized = normalize_cells(&cells);
        assert_eq!(normalized[0], (0, 0));
        assert_eq!(normalized[2], (2, 0));
    }

    #[test]
    fn test_transformations() {
        // Shape 4 is a cross, should have rotational symmetry
        let lines = ["###", ".#.", "###"];
        let shape = parse_shape(&lines);
        let variants = precompute_transforms(&[shape]);

        // Cross has 90° rotational symmetry, so should have fewer unique variants
        assert!(variants[0].len() <= 8);
        assert!(!variants[0].is_empty());
    }

    #[test]
    fn test_can_place() {
        let grid = Grid::new(4, 4);
        let variant = ShapeVariant {
            cells: vec![(0, 0), (1, 0), (2, 0)],
            width: 3,
            height: 1,
        };

        assert!(can_place(&grid, &variant, 0, 0));
        assert!(can_place(&grid, &variant, 1, 3));
        assert!(!can_place(&grid, &variant, 2, 0)); // Would go out of bounds
    }

    #[test]
    fn test_small_packing() {
        // 4x4 grid with 2 copies of shape 4 (should succeed according to puzzle)
        let (transforms, _) = parse_input(EXAMPLE);
        let region = Region {
            width: 4,
            height: 4,
            required_counts: [0, 0, 0, 0, 2, 0],
        };

        assert!(can_fit_all_shapes(&region, &transforms));
    }

    #[test]
    fn test_impossible_packing() {
        let (transforms, _) = parse_input(EXAMPLE);
        // 12x5 with one extra shape 4 (should fail according to puzzle)
        let region = Region {
            width: 12,
            height: 5,
            required_counts: [1, 0, 1, 0, 3, 2],
        };

        assert!(!can_fit_all_shapes(&region, &transforms));
    }
}

// Define benchmarks using the common macro
crate::define_day_benches!(Day12);
