# Day 6: Trash Compactor

## Problem Statement

Parse a math worksheet where numbers are arranged vertically in columns. Each problem consists of numbers and an operation (+ or *). Part 1 reads the worksheet left-to-right treating each column group as complete numbers arranged vertically. Part 2 reads the same worksheet right-to-left where each single-character column represents one complete number formed by reading top-to-bottom.

### Example Input
```
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
```

### Example Output
- Part 1: `4277556` (123×45×6 + 328+64+98 + 51×387×215 + 64+23+314)
- Part 2: `3263827` (356×24×1 + 8+248+369 + 175×581×32 + 4+431+623)

## Algorithm & Approach

### Part 1

**Data Structures:**
- `Vec<&str>` for zero-copy line references
- `Vec<u64>` for temporary number storage per problem
- Direct calculation without intermediate problem storage

**Algorithm:**
1. Iterate left-to-right through column positions
2. Skip separator columns (all spaces)
3. Find column boundaries for each problem (next separator)
4. Parse numbers from the column range using substring extraction
5. Extract operation from last row
6. Calculate result (sum or product) and add to grand total

**Key Insight:** Process problems immediately without storing intermediate structures, enabling single-pass calculation with minimal allocations.

**Complexity:**
- Time: O(rows × cols) - single pass through the input
- Space: O(rows) - temporary storage for numbers in current problem

### Part 2

**Algorithm Change:** Instead of treating column groups as numbers, each single character column represents ONE complete number formed by reading vertically.

**Algorithm:**
1. Iterate right-to-left through column positions
2. Skip separator columns (same logic as Part 1)
3. Find problem boundaries moving leftward
4. For each column in problem range:
   - Read vertically (top-to-bottom) to form one number
   - Add to problem's number list
5. Extract operation and calculate result

**Reused Components:**
- `is_separator_column()` - unchanged
- `calculate_result()` - unchanged
- Column boundary detection - adapted for right-to-left

**New Challenge:** Reversing iteration direction while maintaining correct problem boundary detection and handling edge cases at column 0.

**Complexity:**
- Time: O(rows × cols) - same as Part 1
- Space: O(cols) - one number per column in worst case (all problems same row count)

## Implementation Notes

**Rust Patterns:**
- **Zero-copy parsing:** Used `&str` slices throughout to avoid String allocations
- **Byte-level operations:** `as_bytes()` for fast character checking
- **Iterator adapters:** `take()` for safe iteration without indexing
- **Functional composition:** `filter` and `map` for clean number collection

**Performance Optimizations:**
- Inline hints on hot-path functions (`is_separator_column`)
- Single-pass processing for both parts
- Immediate calculation without intermediate Problem structs
- Byte access instead of char iterators
- Reusable helper functions to minimize code duplication

**Edge Cases:**
- Empty input handling
- Lines with different lengths (trailing spaces)
- Problems at column 0 (leftmost edge for Part 2)
- Sparse digits (rows missing characters in some columns)
- Single-column problems
- Unsigned subtraction safety in right-to-left iteration

**Key Challenge:** Part 2's right-to-left iteration required careful loop termination logic to avoid underflow on unsigned column indices. The solution uses explicit checks for `col == 0` before decrementing.

**Design Decision:** Part 2 uses String accumulation for vertical digit concatenation. While a mathematical approach (multiplying by 10) was considered, String parsing proved simpler and equally performant given the small number of digits per column.

## Benchmark Results

| Part | Time | Input Characteristics |
|------|------|---------------------|
| Part 1 | 86 µs | ~1000 columns × 5 rows |
| Part 2 | 139 µs | Same input, different parsing |
| **Total** | **225 µs** | |

**Performance Notes:**
- Part 2 is ~61% slower than Part 1 due to String allocation for vertical concatenation
- Both solutions maintain O(rows × cols) time complexity
- Memory usage remains minimal with no large intermediate structures
- Combined runtime under 250µs meets performance target

## Alternative Approaches Considered

1. **Matrix Transpose (Part 2)**: Build a full character matrix and transpose it for easier column access
   - **Why not chosen**: Requires O(rows × cols) extra space and two passes through data
   - **Trade-off**: Simpler logic but worse memory footprint and no performance gain

2. **Mathematical Digit Building (Part 2)**: Build numbers using `n = n * 10 + digit` instead of String
   - **Why not chosen**: More complex with tracking whether each number has started
   - **Trade-off**: Marginal performance gain (~5-10µs) not worth the complexity

3. **Regex Column Parsing**: Use regular expressions to identify column boundaries
   - **Why not chosen**: Regex compilation overhead and less readable
   - **Trade-off**: More elegant pattern matching but significantly slower

---

**Key Takeaways:**
- **Direction matters**: Same data structure, completely different interpretation based on iteration direction
- **Zero-copy parsing**: Extensive use of `&str` slices avoided unnecessary allocations while maintaining readability
- **Premature optimization**: String approach in Part 2 was simpler and "good enough" - mathematical optimization would add complexity for minimal gain
