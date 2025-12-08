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
        // Use array instead of HashMap for better performance
        let mut counts = vec![0usize; self.parent.len()];
        for i in 0..self.parent.len() {
            let root = self.find(i);
            counts[root] += 1;
        }

        // Collect non-zero counts and sort descending
        let mut sizes: Vec<usize> = counts.into_iter().filter(|&c| c > 0).collect();
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

fn solve_part1(coordinates: &[(i64, i64, i64)], pairs_to_process: usize) -> i64 {
    let n = coordinates.len();

    // Pre-allocate exact size needed
    let total_edges = (n * (n - 1)) / 2;
    let mut edges = Vec::with_capacity(total_edges);

    // Compute all pairwise distances in one pass
    // Note: clippy suggests enumerate() but we need pairwise combinations (i,j) where i<j
    #[allow(clippy::needless_range_loop)]
    for i in 0..n {
        let ci = coordinates[i];
        for j in (i + 1)..n {
            let cj = coordinates[j];
            // Inline distance calculation for speed
            let dx = ci.0 - cj.0;
            let dy = ci.1 - cj.1;
            let dz = ci.2 - cj.2;
            edges.push((dx * dx + dy * dy + dz * dz, i, j));
        }
    }

    // Use partial sort - only need the smallest pairs_to_process edges
    edges.select_nth_unstable_by_key(pairs_to_process - 1, |e| e.0);
    edges[..pairs_to_process].sort_unstable_by_key(|e| e.0);

    // Process the specified number of closest pairs with Union-Find
    let mut uf = UnionFind::new(n);
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

/// Solve Part 2 using optimized Prim's algorithm
fn solve_part2_prim(coordinates: &[(i64, i64, i64)]) -> i64 {
    let n = coordinates.len();
    if n == 0 {
        return 0;
    }

    let mut in_mst = vec![false; n];
    let mut min_dist = vec![i64::MAX; n];
    let mut parent = vec![0usize; n];

    // Start from vertex 0
    min_dist[0] = 0;

    let mut edges_added = 0;
    let mut last_edge = (0i64, 0i64);

    // Simple O(n²) Prim's without heap - faster for dense graphs
    for _ in 0..n {
        // Find minimum distance vertex not in MST
        let mut min_idx = 0;
        let mut min_val = i64::MAX;

        for v in 0..n {
            if !in_mst[v] && min_dist[v] < min_val {
                min_val = min_dist[v];
                min_idx = v;
            }
        }

        let u = min_idx;
        in_mst[u] = true;

        if u != 0 {
            edges_added += 1;
            last_edge = (coordinates[parent[u]].0, coordinates[u].0);

            if edges_added == n - 1 {
                break;
            }
        }

        // Update distances to non-MST vertices
        for v in 0..n {
            if !in_mst[v] {
                let dist = distance_squared(coordinates[u], coordinates[v]);
                if dist < min_dist[v] {
                    min_dist[v] = dist;
                    parent[v] = u;
                }
            }
        }
    }

    last_edge.0 * last_edge.1
}

impl Day for Day08 {
    fn part1(&self, input: &str) -> String {
        let coordinates = parse_coordinates(input);
        solve_part1(&coordinates, 1000).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let coordinates = parse_coordinates(input);
        solve_part2_prim(&coordinates).to_string()
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
