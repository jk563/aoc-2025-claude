//! Solution implementation for Day 11: Reactor
//!
//! ## Part 1: Unconstrained Path Counting
//! Count all paths from "you" to "out" in a directed graph.
//! Uses DFS with backtracking to enumerate simple paths.
//! Performance: ~257 µs
//!
//! ## Part 2: Constrained Path Counting
//! Count paths from "svr" to "out" that visit both "dac" and "fft".
//! Uses DFS with memoization and state tracking (bit flags for visited required nodes).
//! Performance: ~551 µs
//!
//! ## Algorithm Complexity
//! - Time: O((V+E) × P) where P = number of paths
//! - Space: O(V) for visited/constraint tracking
//! - Both parts use DFS with backtracking

use crate::runner::Day;
use std::collections::{HashMap, HashSet};

/// Solver for Day 11
pub struct Day11;

impl Day for Day11 {
    fn part1(&self, input: &str) -> String {
        let graph = parse_graph(input);
        let mut visited = HashSet::with_capacity(graph.len());
        count_paths_dfs(&graph, "you", "out", &mut visited).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let graph = parse_graph(input);

        // Use memoized version for better performance
        let mut visited = HashSet::with_capacity(graph.len());
        let mut memo = HashMap::new();

        count_paths_memoized(&graph, "svr", "out", &mut visited, &mut memo, 0).to_string()
    }
}

// Helper functions

/// Parse the input into a directed graph represented as an adjacency list
fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (device, outputs) = line.split_once(": ").expect("Invalid line format");
            let neighbors = outputs.split_whitespace().collect();
            (device, neighbors)
        })
        .collect()
}

/// Count all paths from start to target using DFS with backtracking
fn count_paths_dfs<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    target: &'a str,
    visited: &mut HashSet<&'a str>,
) -> usize {
    // Base case: reached the target
    if current == target {
        return 1;
    }

    // Get neighbors or return 0 if node has no outgoing edges
    let Some(neighbors) = graph.get(current) else {
        return 0;
    };

    // Mark current node as visited
    visited.insert(current);

    // Count paths through all unvisited neighbors
    let mut count = 0;
    for &neighbor in neighbors {
        if !visited.contains(neighbor) {
            count += count_paths_dfs(graph, neighbor, target, visited);
        }
    }

    // Backtrack: remove current node from visited set
    visited.remove(current);

    count
}

/// Count paths with memoization and state tracking for required nodes
///
/// Uses DFS with memoization to handle graphs with exponentially many paths.
/// State is represented as bit flags: 0=neither, 1=dac only, 2=fft only, 3=both.
/// Memoizes (node, state) → count to avoid recomputing overlapping subproblems.
fn count_paths_memoized<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    target: &'a str,
    visited: &mut HashSet<&'a str>,
    memo: &mut HashMap<(&'a str, u8), usize>,
    state: u8,
) -> usize {
    // Base case: reached target with both required nodes
    if current == target {
        return if state == 3 { 1 } else { 0 };
    }

    // Check memo (only if not in current path to avoid cycle issues)
    if !visited.contains(current) {
        if let Some(&cached) = memo.get(&(current, state)) {
            return cached;
        }
    }

    // Dead end
    let Some(neighbors) = graph.get(current) else {
        return 0;
    };

    visited.insert(current);

    // Update state based on current node
    let new_state = match current {
        "dac" => state | 1, // Set bit 0
        "fft" => state | 2, // Set bit 1
        _ => state,
    };

    let mut count = 0;
    for &neighbor in neighbors {
        if !visited.contains(neighbor) {
            count += count_paths_memoized(graph, neighbor, target, visited, memo, new_state);
        }
    }

    visited.remove(current);

    // Cache result (only if we weren't in a cycle)
    if !visited.contains(current) {
        memo.insert((current, state), count);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn test_part1_example() {
        let day = Day11;
        assert_eq!(day.part1(EXAMPLE), "5");
    }

    #[test]
    fn test_parse_graph() {
        let graph = parse_graph(EXAMPLE);
        assert_eq!(graph.get("you"), Some(&vec!["bbb", "ccc"]));
        assert_eq!(graph.get("bbb"), Some(&vec!["ddd", "eee"]));
        assert_eq!(graph.get("ccc"), Some(&vec!["ddd", "eee", "fff"]));
        assert_eq!(graph.get("eee"), Some(&vec!["out"]));
    }

    #[test]
    fn test_simple_path() {
        let input = "you: out";
        let day = Day11;
        assert_eq!(day.part1(input), "1");
    }

    #[test]
    fn test_no_path() {
        let input = "you: bbb\nbbb: ccc";
        let day = Day11;
        assert_eq!(day.part1(input), "0");
    }

    #[test]
    fn test_multiple_paths_diamond() {
        // Diamond shape: you -> {a,b} -> {c,c} -> out
        // Should find 2 paths: you->a->c->out, you->b->c->out
        let input = "you: a b\na: c\nb: c\nc: out";
        let day = Day11;
        assert_eq!(day.part1(input), "2");
    }

    // Part 2 example from puzzle
    const PART2_EXAMPLE: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part2_example() {
        let day = Day11;
        assert_eq!(day.part2(PART2_EXAMPLE), "2");
    }

    #[test]
    fn test_part2_no_required_nodes() {
        // Graph without dac or fft
        let input = "svr: out";
        let day = Day11;
        assert_eq!(day.part2(input), "0");
    }

    #[test]
    fn test_part2_only_one_required() {
        // Path visits only dac
        let input = "svr: dac\ndac: out";
        let day = Day11;
        assert_eq!(day.part2(input), "0");
    }

    #[test]
    fn test_part2_both_required_sequential() {
        // Single path: svr -> dac -> fft -> out
        let input = "svr: dac\ndac: fft\nfft: out";
        let day = Day11;
        assert_eq!(day.part2(input), "1");
    }

    #[test]
    fn test_part2_both_required_reverse_order() {
        // Single path: svr -> fft -> dac -> out
        let input = "svr: fft\nfft: dac\ndac: out";
        let day = Day11;
        assert_eq!(day.part2(input), "1");
    }
}

// Define benchmarks using the common macro
crate::define_day_benches!(Day11);
