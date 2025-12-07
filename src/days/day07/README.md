# Day 7: Tachyon Manifold Splitters

## Problem Statement

A tachyon beam travels through a manifold that contains splitters. When a beam hits a splitter, it branches into two beams traveling left and right. Part 1 counts how many times beams split (beams merge when at the same position). Part 2 counts the total number of distinct timelines when a particle doesn't merge—each path through the manifold is a separate timeline.

### Example Input
```
.......S.......
...............
.......^.......
...............
......^.^......
```

### Example Output
- Part 1: `21` (splits)
- Part 2: `40` (timelines)

## Algorithm & Approach

### Part 1: Split Counting

**Problem:** Count the number of times beams hit splitters.

**Approach:**
- Track active beam columns using `HashSet<usize>`
- For each row with splitters, find which beams hit splitters (set intersection)
- Count the hits, then update beam positions: remove hit column, add left and right neighbors
- Beams naturally merge because duplicate columns in the set become one

**Data Structures:**
- `HashSet<usize>`: Which columns have active beams (deduplicates automatically)
- `Vec<HashSet<usize>>`: Splitter positions per row (fast lookup)

**Key Insight:** Beams merge at the same column, so HashSet deduplication handles this automatically. This makes the solution simple and efficient.

**Complexity:**
- Time: O(rows × active_beams) ≈ O(rows × √cols) in practice
- Space: O(active_beams) ≈ O(√cols)

### Part 2: Timeline Counting

**Problem:** Count distinct timelines where particles don't merge—each split doubles the timeline count.

**Approach:**
- Replace `HashSet<usize>` with `HashMap<usize, u64>` to track particle *counts* at each column
- For each row, process particles at each column:
  - If a splitter is hit, add the particle count to **both** left and right neighbors (both timelines created)
  - Otherwise, pass through unchanged
- Sum all particle counts at the end for total timelines

**Key Difference from Part 1:**
- Part 1: Remove hit column, add neighbors (beam merges elsewhere)
- Part 2: Add count to both neighbors (no merging, both timelines continue)

**Complexity:**
- Time: O(rows × active_columns) ≈ O(rows × √cols)
- Space: O(active_columns) ≈ O(√cols)

**Note on Large Numbers:** With 1594 splits, timeline counts can reach 15+ trillion, but fit comfortably in `u64`.

## Implementation Notes

**Code Reuse:** Part 1 and Part 2 share the same helper functions (`find_start`, `build_splitter_map`) and differ only in the main simulation loop. Both use similar HashSet-based lookups.

**Performance Optimizations:**
- Pre-build splitter map once, reuse for both parts
- Skip rows with no splitters (many empty rows in input)
- Early exit if no active beams/particles remain

**Interesting Patterns:**
- Used `or_default()` on HashMap entries for clean accumulation syntax
- Set intersection operation (`active_beams.intersection(splitters)`) for efficient hit detection
- Simple iterator-based sum for final timeline count

**Edge Cases Handled:**
- Empty input returns 0
- Boundary checks on col ± 1 to avoid index out of bounds
- Rows with no splitters are skipped efficiently

## Benchmark Results

Both parts run efficiently on the full 143×141 input:

| Part | Time | Result |
|------|------|--------|
| Part 1 | 238 µs | 1594 |
| Part 2 | 270 µs | 15,650,261,281,478 |
| **Total** | **509 µs** | - |

**Why it's fast:**
- Sparse splitter distribution (only ~20% of positions have splitters per row)
- Hash operations are O(1) average case
- HashMap/HashSet stay small due to beam dispersion

## Alternative Approaches Considered

1. **Bitset simulation:** Would use 3×u64 for 141 columns with bitwise operations. Faster due to cache locality but more complex bit manipulation. Unnecessary given current performance.

2. **Sorted Vec instead of HashSet:** Could maintain columns in sorted order. Slightly better cache behavior but requires sort/dedup per row. Not worth the overhead for sparse active beam sets.

3. **Mathematical approach (Pascal's triangle):** Regular splitter patterns follow binomial expansion, but irregular placement breaks this. Simulation is clearer and handles all patterns.

## Key Takeaways

- **Algorithmic insight:** Beam merging (Part 1) vs. timeline branching (Part 2) requires fundamentally different tracking—set membership vs. count accumulation.
- **Rust pattern:** `HashMap::or_default()` enables elegant count accumulation without explicitly checking for entry existence.
- **Design principle:** Reusing infrastructure (splitter map, parsing) between similar problems reduces code duplication while maintaining clarity.
