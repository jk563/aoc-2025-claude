//! Solution implementation for Day 8

use crate::runner::Day;

/// Solver for Day 8
pub struct Day08;

/// Union-Find data structure with path compression and union by size
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        // Union by size
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }

        self.components -= 1;
        true
    }

    fn get_component_sizes(&mut self) -> Vec<usize> {
        let mut components = std::collections::HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *components.entry(root).or_insert(0) += 1;
        }
        let mut sizes: Vec<usize> = components.values().copied().collect();
        sizes.sort_unstable_by(|a, b| b.cmp(a));
        sizes
    }
}

fn parse_coordinates(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            (
                parts[0].parse().unwrap_or(0),
                parts[1].parse().unwrap_or(0),
                parts[2].parse().unwrap_or(0),
            )
        })
        .collect()
}

fn distance_squared(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> i64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    let dz = p1.2 - p2.2;
    dx * dx + dy * dy + dz * dz
}

fn compute_sorted_edges(coordinates: &[(i64, i64, i64)]) -> Vec<(i64, usize, usize)> {
    let n = coordinates.len();
    let mut edges = Vec::with_capacity((n * (n - 1)) / 2);

    // Compute all pairwise distances directly into vector
    for i in 0..n {
        for j in (i + 1)..n {
            let dist_sq = distance_squared(coordinates[i], coordinates[j]);
            edges.push((dist_sq, i, j));
        }
    }

    // Sort edges by distance (smallest first)
    edges.sort_unstable_by_key(|e| e.0);
    edges
}

fn solve_part1(coordinates: &[(i64, i64, i64)], pairs_to_process: usize) -> i64 {
    let n = coordinates.len();
    let mut edges = Vec::with_capacity((n * (n - 1)) / 2);

    // Compute all pairwise distances
    for i in 0..n {
        for j in (i + 1)..n {
            let dist_sq = distance_squared(coordinates[i], coordinates[j]);
            edges.push((dist_sq, i, j));
        }
    }

    // Use partial sort - only need the smallest pairs_to_process edges
    if pairs_to_process < edges.len() {
        edges.select_nth_unstable_by_key(pairs_to_process - 1, |e| e.0);
        edges[..pairs_to_process].sort_unstable_by_key(|e| e.0);
    } else {
        edges.sort_unstable_by_key(|e| e.0);
    }

    // Process the specified number of closest pairs with Union-Find
    let mut uf = UnionFind::new(coordinates.len());
    for (_, i, j) in edges.iter().take(pairs_to_process) {
        uf.union(*i, *j);
    }

    // Find the 3 largest component sizes
    let sizes = uf.get_component_sizes();
    if sizes.len() >= 3 {
        sizes[0] as i64 * sizes[1] as i64 * sizes[2] as i64
    } else if sizes.len() == 2 {
        sizes[0] as i64 * sizes[1] as i64
    } else if sizes.len() == 1 {
        sizes[0] as i64
    } else {
        0
    }
}

fn solve_part2(coordinates: &[(i64, i64, i64)]) -> i64 {
    let edges = compute_sorted_edges(coordinates);

    // Process edges until all nodes are in one component
    let mut uf = UnionFind::new(coordinates.len());
    let mut last_edge = (0i64, 0i64);

    for (_, i, j) in &edges {
        if uf.union(*i, *j) {
            // Track the last successful union
            last_edge = (coordinates[*i].0, coordinates[*j].0);
        }
        if uf.components == 1 {
            break;
        }
    }

    last_edge.0 * last_edge.1
}

impl Day for Day08 {
    fn part1(&self, input: &str) -> String {
        let coordinates = parse_coordinates(input);
        // Process 1000 pairs for the full puzzle input
        solve_part1(&coordinates, 1000).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let coordinates = parse_coordinates(input);
        // Process all edges until fully connected
        solve_part2(&coordinates).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1_example() {
        // The example uses 10 pairs, not 1000
        let coordinates = parse_coordinates(EXAMPLE);
        assert_eq!(solve_part1(&coordinates, 10), 40);
    }

    #[test]
    fn test_part2_example() {
        // The last connection in the example is between (216,146,977) and (117,168,530)
        // Product of X coordinates: 216 * 117 = 25272
        let day = Day08;
        assert_eq!(day.part2(EXAMPLE), "25272");
    }
}

// Define benchmarks using the common macro
crate::define_day_benches!(Day08);
