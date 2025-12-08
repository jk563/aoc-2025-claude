# Day 8: Playground

## Problem Statement

Elves are connecting junction boxes in 3D space with strings of lights. Each junction box has coordinates (X, Y, Z). When two boxes are connected, they form a circuit. The goal is to connect boxes optimally based on their Euclidean distances to minimize the number of extension cables needed.

### Example Input
```
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
425,690,689
```

### Example Output
- Part 1: `40` (5 × 4 × 2, product of 3 largest circuits after 10 connections)
- Part 2: `25272` (216 × 117, X coordinates of last MST edge)

## Algorithm & Approach

### Part 1: Partial Graph Connection

**Problem:** After connecting the 1000 closest pairs of junction boxes, find the product of the sizes of the three largest circuits.

**Approach: Kruskal's Algorithm with Partial Sort**
1. Compute all pairwise distances (499,500 edges for 1000 nodes)
2. Use `select_nth_unstable` to partially sort—only find the smallest 1000 edges
3. Process these edges using Union-Find to form circuits
4. Calculate component sizes and return the product of the three largest

**Data Structures:**
- `Vec<(i64, usize, usize)>`: Edge list with (distance², node_i, node_j)
- `UnionFind`: Disjoint set data structure with path compression and union by size
  - Path compression: Flattens tree during `find()` for O(α(n)) amortized time
  - Union by size: Attaches smaller tree to larger tree root to minimize height

**Key Insight:** We don't need a fully sorted edge list. Using `select_nth_unstable` partitions edges around the 1000th smallest in O(n) average time, avoiding the full O(n log n) sort.

**Complexity:**
- Time: O(n²) for distance computation + O(n) for partial select + O(k log k) for sorting k=1000 edges + O(k·α(n)) for Union-Find operations ≈ O(n²)
- Space: O(n²) for edge storage (but only ~500k edges)

With n=1000 junction boxes, this computes ~500k distances and processes just 1000 edges.

### Part 2: Minimum Spanning Tree

**Problem:** Connect all junction boxes into a single circuit using minimum total cable length. Return the product of X coordinates of the last edge added.

**Approach: Prim's Algorithm (Array-Based)**
1. Start with arbitrary node (vertex 0)
2. Maintain minimum edge weight to reach each unvisited vertex
3. Greedily add the closest vertex to the MST
4. Track the last edge added—this completes the MST
5. Return product of X coordinates of that edge's endpoints

**Why Prim's over Kruskal's?**
- Kruskal's requires sorting all 499,500 edges: O(E log E)
- Prim's lazily explores edges: O(V²) with array-based implementation
- For dense graphs (E ≈ V²), array-based Prim's is faster than heap-based variants

**Data Structures:**
- `Vec<bool>`: Tracks which vertices are in the MST
- `Vec<i64>`: Minimum distance from MST to each vertex
- `Vec<usize>`: Parent pointers to reconstruct edges

**Key Insight:** The last edge added to an MST is the edge that finally connects the last isolated vertex. For Part 2, we only care about this final connection, not the entire MST structure.

**Complexity:**
- Time: O(V²) for V=1000 vertices
  - Each of V iterations finds minimum (O(V)) and updates distances (O(V))
- Space: O(V) for tracking state

This is optimal for dense graphs where computing all edge weights on-demand is cheaper than precomputing and sorting them.

## Implementation Notes

**Union-Find Optimizations:**
- **Path compression:** During `find()`, flatten the tree by making nodes point directly to the root. Reduces future lookups to near-constant time.
- **Union by size:** Always attach the smaller tree to the larger tree's root. This keeps tree height logarithmic.
- **Component size tracking:** Use a `Vec` indexed by root instead of `HashMap` for cache-friendly counting (saves ~50µs).

**Part 1 Partial Sorting:**
```rust
edges.select_nth_unstable_by_key(pairs_to_process - 1, |e| e.0);
edges[..pairs_to_process].sort_unstable_by_key(|e| e.0);
```
- `select_nth_unstable` partitions edges around the 1000th smallest in O(n) average time
- Only sort the first 1000 elements instead of all 499,500
- Avoids allocation by operating in-place

**Inline Distance Calculations:**
Part 1 inlines distance computation directly in the edge loop:
```rust
let dx = ci.0 - cj.0;
let dy = ci.1 - cj.1;
let dz = ci.2 - cj.2;
edges.push((dx * dx + dy * dy + dz * dz, i, j));
```
This improves cache locality by avoiding function call overhead and keeping coordinate data hot.

**Array-Based Prim's vs Heap-Based:**
- Heap-based Prim's: O((V + E) log V), better for sparse graphs
- Array-based Prim's: O(V²), better for dense graphs where E ≈ V²
- With V=1000 and E≈500k, array-based wins due to simpler operations and better cache behavior

**Avoiding Square Roots:**
All distance comparisons use squared Euclidean distance (dx² + dy² + dz²) to avoid expensive `sqrt()` calls. This preserves ordering while being ~10× faster.

**Edge Cases:**
- Empty input returns 0
- Component size calculation handles variable number of components (1, 2, or 3+)
- Part 2 tracks the last edge explicitly to avoid reconstructing MST

## Performance Journey

### Initial Implementation
- Part 1: 121 ms using BinaryHeap and full sort
- Part 2: 123 ms using Kruskal's algorithm
- Total: 244 ms

### Optimization 1: Quick Wins (8.5× speedup)
**Changes:**
- Remove BinaryHeap push/pop overhead, use Vec directly
- Use `select_nth_unstable` for partial sorting in Part 1
- Change `sort()` to `sort_unstable()` (no stability needed)

**Results:**
- Part 1: 121 ms → 8.14 ms (14.9× faster)
- Part 2: 123 ms → 20.43 ms (6.0× faster)
- Total: 244 ms → 28.57 ms (8.5× improvement)

### Optimization 2: Prim's Algorithm (57× speedup for Part 2)
**Changes:**
- Part 2: Switch from Kruskal's to heap-based Prim's algorithm
- Lazy edge exploration instead of precomputing all edges
- Eliminate sorting overhead for MST construction

**Results:**
- Part 1: 8.14 ms (unchanged)
- Part 2: 123 ms → 2.14 ms (57.5× faster)
- Total: 244 ms → 10.23 ms (23.8× improvement)

### Optimization 3: Final Micro-Optimizations (23× total speedup)
**Changes:**
- Replace heap-based Prim's with O(V²) array-based version (better for dense graphs)
- Inline distance calculations in Part 1 for cache locality
- Replace HashMap with Vec in Union-Find component size tracking

**Results:**
- Part 1: 121 ms → 8.72 ms (13.9× faster)
- Part 2: 123 ms → 1.89 ms (65.1× faster)
- Total: 244 ms → 10.61 ms (23.0× improvement)

## Final Performance

| Part | Time | Result | Algorithm |
|------|------|--------|-----------|
| Part 1 | ~8.7 ms | Product of 3 largest circuits | Kruskal's with partial sort |
| Part 2 | ~1.9 ms | X₁ × X₂ of final MST edge | Array-based Prim's MST |
| **Total** | **~10.6 ms** | - | **23× faster than initial** |

**Performance Characteristics:**
- Part 1 dominated by O(n²) distance computation (500k edges)
- Part 2 dominated by O(n²) Prim's iterations (1M comparisons)
- Both scale quadratically but with small constants due to optimizations
- Cache-friendly data structures (Vec over HashMap/BinaryHeap) provide significant gains

## Alternative Approaches Considered

1. **Kruskal's for Part 2 (original approach):**
   - Requires sorting all 499,500 edges
   - Time: O(E log E) ≈ 500k × log(500k) ≈ 9M operations
   - **Why it lost:** Prim's O(V²) ≈ 1M operations is 9× fewer for dense graphs
   - Kept for Part 1 where we only need 1000 edges (partial sort is optimal)

2. **Heap-based Prim's:**
   - Time: O((V + E) log V) ≈ 500k × log(1000) ≈ 5M operations
   - **Why it lost:** Array-based O(V²) ≈ 1M operations has simpler overhead
   - Heap would win for sparse graphs (E << V²), but not here

3. **Dijkstra-like MST:**
   - Similar to Prim's but using priority queue for all edges
   - **Why not chosen:** Same asymptotic complexity as heap-based Prim's, more complex

4. **Borůvka's Algorithm:**
   - Parallel-friendly MST algorithm
   - **Why not chosen:** More complex to implement, no performance advantage for single-threaded execution on this input size

## Key Takeaways

- **Algorithm selection matters:** Prim's vs Kruskal's choice depends on graph density—array-based Prim's dominates for dense graphs (E ≈ V²)
- **Partial sorting optimization:** `select_nth_unstable` partitions data in O(n) vs full sort O(n log n) when you only need the k smallest elements
- **Cache locality:** Inline calculations, Vec over HashMap, and array-based algorithms reduce memory indirection
- **Union-Find optimizations:** Path compression + union by size achieves near-constant amortized time for disjoint-set operations
- **Premature optimization isn't evil if you measure:** Three rounds of optimization (8.5×, 57×, 23×) each targeted profiled bottlenecks
