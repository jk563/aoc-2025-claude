# Day 4: Printing Department

## Problem Statement

The elves are using forklifts to remove rolls of toilet paper from a warehouse. A roll can be accessed by a forklift if it has fewer than 4 adjacent rolls (including diagonals). The task is to determine how many rolls can be removed initially (Part 1) and how many total rolls can be removed through successive iterations (Part 2).

### Example Input
```
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
```

### Example Output
- Part 1: `13`
- Part 2: `43`

## Algorithm & Approach

### Part 1

Count all rolls (`@`) that have fewer than 4 neighboring rolls in any of the 8 adjacent positions (Moore neighborhood).

**Data Structures:**
- Flat `Vec<u8>` for the grid (memory efficient, cache-friendly)
- Pre-computed neighbor counts array for O(1) lookups

**Algorithm:**
1. Parse input into byte array
2. Pre-compute neighbor counts for all positions with rolls
3. Filter and count positions where `neighbor_count < 4`

**Complexity:**
- Time: O(n) where n = grid size
- Space: O(n) for grid and neighbor counts

### Part 2

Repeatedly remove accessible rolls until no more can be removed, counting total removals.

**Key Insight:** When a roll is removed, only its neighbors can potentially become newly accessible. We track neighbor counts incrementally instead of rescanning the entire grid.

**Algorithm (Neighbor-Count Tracking):**
1. Pre-compute neighbor counts for all rolls during parsing
2. Initialize a queue with all rolls that have `< 4` neighbors
3. While queue is not empty:
   - Remove roll from queue
   - Mark position as empty
   - For each of its 8 neighbors that still has a roll:
     - Decrement that neighbor's count
     - If count drops below 4, add neighbor to queue
4. Return total removals

**Complexity:**
- Time: O(r) where r = total rolls removed (each roll processed once)
- Space: O(n) for grid and neighbor counts

The key optimization is that each roll is only added to the queue once (when it first becomes accessible), avoiding redundant checks.

## Implementation Notes

### Three Implementations Provided

1. **Day04** (Default): Pre-computed neighbor counts with incremental updates
   - Uses VecDeque for BFS-style processing
   - ~0.97ms total runtime
   - **Fastest implementation**
   - Inspired by Reddit discussion solutions

2. **Day04DirtyTracking**: Maintains a HashSet of "dirty" positions to check
   - Only checks neighbors of removed rolls
   - ~1.5ms total runtime
   - Good middle-ground approach

3. **Day04Naive**: Naive approach that rescans entire grid each iteration
   - Simple and readable
   - ~11ms total runtime
   - Baseline for understanding the problem

### Rust Patterns Used

- **Flat indexing**: `idx = row * cols + col` for cache-friendly memory access
- **Manual loop unrolling**: Explicit neighbor checks instead of nested loops in hot paths
- **Inline functions**: `#[inline]` on neighbor lookups for compiler optimization
- **Byte arrays**: `Vec<u8>` instead of `Vec<char>` for smaller memory footprint
- **Array instead of Vec**: Fixed-size `[Option<usize>; 8]` for neighbor lists (stack allocation)

### Edge Cases Handled

- Grid boundaries (avoiding out-of-bounds accesses)
- Empty positions (dots) vs rolls (@ symbols)
- Duplicate queue entries (checking if roll still exists before processing)

## Benchmark Results

| Implementation | Part 1 Time | Part 2 Time | Total | Speedup |
|----------------|-------------|-------------|-------|---------|
| Day04 (default) | 348µs | 620µs | 0.97ms | **11x** |
| Day04DirtyTracking | 125µs | 1.40ms | 1.52ms | 7x |
| Day04Naive | 246µs | 10.88ms | 11.13ms | 1x |

**Winner:** `Day04` - Fastest overall due to:
- Pre-computation eliminates redundant neighbor counting
- Queue-based processing visits each roll exactly once
- No HashSet overhead (uses flat arrays)

### Performance Insights

- **Part 1** is fastest with `Day04DirtyTracking` due to simpler counting without queue overhead
- **Part 2** benefits massively from incremental neighbor count updates in `Day04`
- The default approach scales better with larger inputs and more iterations

## Alternative Approaches Considered

1. **Convolution-based (NumPy/SciPy style)**: Use 2D convolution with 3×3 kernel to count neighbors
   - Common in Python solutions with numpy
   - Not beneficial in Rust without SIMD (overhead outweighs gains)
   - Would add dependency complexity

2. **Cellular automata simulation**: Treat as Conway's Game of Life variant
   - Elegant conceptual model
   - Similar performance to original approach
   - Doesn't optimize the "only removals" constraint

3. **Union-Find for connected components**: Track groups of rolls
   - Overkill for this problem
   - Doesn't help with neighbor counting
   - More complex without clear benefit

---

**Key Takeaways:**
- Pre-computing invariants (neighbor counts) and updating them incrementally beats recalculation
- Queue-based BFS processing ensures each element is handled exactly once when dependencies exist
- Flat array indexing and byte arrays provide significant performance wins in Rust
