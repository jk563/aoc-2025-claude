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

    fn part2(&self, input: &str) -> String {
        let tiles = parse_tiles(input);
        let max_area = find_largest_rectangle_in_polygon(&tiles);
        max_area.to_string()
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

/// Find the largest rectangle that only contains red or green tiles
/// Red tiles form a polygon in order, green tiles are on edges and inside
fn find_largest_rectangle_in_polygon(tiles: &[(i32, i32)]) -> i64 {
    let mut max_area = 0i64;

    // Check all pairs of red tiles as corners
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];

            // Get rectangle bounds
            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            // Check if rectangle is valid (only contains red/green tiles)
            if is_rectangle_valid(tiles, min_x, max_x, min_y, max_y) {
                let width = (max_x - min_x) as i64 + 1;
                let height = (max_y - min_y) as i64 + 1;
                let area = width * height;
                max_area = max_area.max(area);
            }
        }
    }

    max_area
}

/// Check if a rectangle only contains red or green tiles
/// All four corners must be red or inside/on the polygon
fn is_rectangle_valid(
    polygon: &[(i32, i32)],
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) -> bool {
    // Critical: ALL FOUR corners must be valid (red, on edge, or inside)
    let corners = [
        (min_x, min_y),
        (min_x, max_y),
        (max_x, min_y),
        (max_x, max_y),
    ];

    for &(x, y) in &corners {
        if !is_point_valid(polygon, x, y) {
            return false;
        }
    }

    // Sample boundary and interior points more densely
    let sample_points = generate_sample_points(min_x, max_x, min_y, max_y);
    for (x, y) in sample_points {
        if !is_point_valid(polygon, x, y) {
            return false;
        }
    }

    true
}

/// Generate sample points within a rectangle to validate
/// Sample boundary points densely enough to catch polygon boundaries
fn generate_sample_points(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    // For small rectangles, check all boundary points
    if (width as i64) * (height as i64) <= 10000 {
        for x in min_x..=max_x {
            points.push((x, min_y));
            points.push((x, max_y));
        }
        for y in (min_y + 1)..max_y {
            points.push((min_x, y));
            points.push((max_x, y));
        }
    } else {
        // For large rectangles, sample densely along edges
        let step_x = (width / 200).max(1);
        let step_y = (height / 200).max(1);

        for x in (min_x..=max_x).step_by(step_x as usize) {
            points.push((x, min_y));
            points.push((x, max_y));
        }
        for y in (min_y + 1..max_y).step_by(step_y as usize) {
            points.push((min_x, y));
            points.push((max_x, y));
        }
    }

    // Always check interior sample points
    let mid_x = (min_x + max_x) / 2;
    let mid_y = (min_y + max_y) / 2;
    points.push((mid_x, mid_y));

    let q1_x = (min_x + mid_x) / 2;
    let q3_x = (mid_x + max_x) / 2;
    let q1_y = (min_y + mid_y) / 2;
    let q3_y = (mid_y + max_y) / 2;
    points.extend_from_slice(&[
        (q1_x, q1_y),
        (q1_x, q3_y),
        (q3_x, q1_y),
        (q3_x, q3_y),
        (mid_x, q1_y),
        (mid_x, q3_y),
        (q1_x, mid_y),
        (q3_x, mid_y),
    ]);

    points
}

/// Check if a point is red or green (inside/on the polygon)
fn is_point_valid(polygon: &[(i32, i32)], x: i32, y: i32) -> bool {
    // Check if point is a red tile
    if polygon.iter().any(|&(px, py)| px == x && py == y) {
        return true;
    }

    // Check if point is on a polygon edge (green)
    if is_on_polygon_edge(polygon, x, y) {
        return true;
    }

    // Check if point is inside polygon (green)
    is_inside_polygon(polygon, x, y)
}

/// Check if a point lies on any edge of the polygon
fn is_on_polygon_edge(polygon: &[(i32, i32)], x: i32, y: i32) -> bool {
    let n = polygon.len();
    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        if is_point_on_segment(x, y, x1, y1, x2, y2) {
            return true;
        }
    }
    false
}

/// Check if point (x, y) lies on line segment from (x1, y1) to (x2, y2)
fn is_point_on_segment(x: i32, y: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    // Check if point is within bounding box
    if x < x1.min(x2) || x > x1.max(x2) || y < y1.min(y2) || y > y1.max(y2) {
        return false;
    }

    // Check if point is collinear using cross product
    let dx1 = x - x1;
    let dy1 = y - y1;
    let dx2 = x2 - x1;
    let dy2 = y2 - y1;

    // Cross product should be zero for collinear points
    dx1 as i64 * dy2 as i64 == dy1 as i64 * dx2 as i64
}

/// Check if a point is inside the polygon using ray casting algorithm
fn is_inside_polygon(polygon: &[(i32, i32)], x: i32, y: i32) -> bool {
    let mut inside = false;
    let n = polygon.len();

    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        // Ray casting: count intersections with edges
        if ((y1 > y) != (y2 > y)) && (x < (x2 - x1) * (y - y1) / (y2 - y1) + x1) {
            inside = !inside;
        }
    }

    inside
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
    fn test_part2_example() {
        let day = Day09;
        assert_eq!(day.part2(EXAMPLE), "24");
    }
}

// Define benchmarks using the common macro
crate::define_day_benches!(Day09);
