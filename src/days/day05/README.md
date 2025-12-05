# Day 5: Cafeteria

## Problem Statement

The Elves have a complicated inventory management system for their cafeteria ingredients. Each ingredient has a numeric ID, and certain ID ranges are considered "fresh" while others are "spoiled". The database consists of fresh ingredient ID ranges (inclusive) followed by a list of available ingredient IDs. We need to determine which ingredients are safe to use.

### Example Input
```
3-5
10-14
16-20
12-18

1
5
8
11
17
32
```

### Example Output
- Part 1: `3`
- Part 2: `14`

## Algorithm & Approach

### Part 1

Count how many of the available ingredient IDs fall within any of the fresh ingredient ID ranges.

**Strategy:**
- Parse the input into two sections: ranges and IDs to check
- For each ID, check if it falls within any range (inclusive bounds)
- An ID is fresh if `start <= id <= id <= end` for any range
- Ranges can overlap, but we only need to check if an ID is in ANY range

**Data Structures:**
- Vector of `(u64, u64)` tuples for ranges
- Vector of `u64` for IDs to check
- Simple linear search through ranges for each ID

**Key Insight:** With only ~182 ranges and ~1000 IDs, a simple O(n×m) approach is optimal. The overhead of optimizations like binary search or range merging would exceed their benefit.

**Complexity:**
- Time: O(n × m) where n = number of IDs (~1000) and m = number of ranges (~182)
- Space: O(n + m) for storing ranges and IDs

The actual number of comparisons is ~182,000 in the worst case, which modern CPUs handle in microseconds.

### Part 2

Count the total number of unique ingredient IDs covered by all the fresh ranges (ignoring the list of available IDs).

**Challenge:** Ranges can overlap (e.g., 16-20 and 12-18 both cover 16, 17, 18), so we must avoid double-counting.

**Strategy:**
1. Sort ranges by start position
2. Merge overlapping or adjacent ranges
3. Sum the size of each merged range: `(end - start + 1)`

**Merging Algorithm:**
- Start with the first range as the current merged range
- For each subsequent range:
  - If it overlaps or is adjacent (`start <= last_end + 1`), extend the current range
  - Otherwise, start a new merged range
- Adjacent ranges (e.g., 5-10 and 11-20) are merged into a single continuous range

**Example:**
- Input ranges: `3-5`, `10-14`, `16-20`, `12-18`
- After sorting: `3-5`, `10-14`, `12-18`, `16-20`
- After merging: `3-5` (3 IDs), `10-14` (5 IDs), `12-20` (9 IDs)
- Total: 3 + 5 + 9 = **14 IDs**

**Complexity:**
- Time: O(m log m) where m = number of ranges, dominated by sorting
- Space: O(m) for the sorted and merged ranges

## Implementation Notes

**Iterator Chaining:**
The solution uses Rust's iterator methods extensively for clean, functional-style code:
```rust
ranges.iter().any(|&(start, end)| id >= start && id <= end)
```

**Range Merging Logic:**
The key condition `start <= last_end + 1` handles both overlapping ranges and adjacent ranges:
- `10-14` and `12-18` overlap (12 ≤ 14), merge to `10-18`
- `10-14` and `15-20` are adjacent (15 ≤ 14+1), merge to `10-20`
- `10-14` and `16-20` have a gap (16 > 14+1), keep separate

**Integer Overflow Safety:**
Using `u64` handles the large ingredient IDs (up to ~560 trillion in the input). The addition `last_end + 1` is safe because ranges are well within u64 bounds.

**Performance Optimization:**
Part 1 uses early termination with `.any()` - stops checking ranges as soon as a match is found. Part 2 is actually faster (69 µs vs 147 µs) because it only processes ranges, not individual IDs.

## Performance

| Part | Time | Answer |
|------|------|--------|
| Part 1 | 147 µs | 615 |
| Part 2 | 69 µs | 353,716,783,056,994 |
| **Total** | **216 µs** | - |

Part 2 is faster despite having more logic because it processes far less data (182 ranges vs 1000 IDs).

## Alternative Approaches Considered

1. **Binary Search for Part 1**: Sort ranges and use binary search to find potential matches. Not chosen because:
   - Sorting overhead: O(m log m)
   - Still need to check overlapping ranges
   - The simple O(n×m) approach is already fast enough at 147 µs

2. **Range Merging for Part 1**: Merge ranges first to reduce comparisons. Not chosen because:
   - Merging cost: O(m log m) sorting + O(m) merging
   - Only saves comparisons if ranges heavily overlap
   - With 182 ranges, the simple approach wins

3. **HashSet for Part 2**: Expand all ranges into a HashSet of IDs. Not viable because:
   - Would require storing ~353 trillion integers
   - Memory would be astronomically large
   - The merging approach is O(m log m) and uses minimal memory

---

**Key Takeaways:**
- For small input sizes, simple O(n×m) algorithms often outperform complex optimizations
- Range merging is the classic solution for counting unique elements across overlapping ranges
- Rust's `.any()` provides elegant early termination for existence checks
