# Day 2: Gift Shop

## Problem Statement

Identify and sum invalid product IDs in given ranges. An invalid ID is one where the digits form a repeating pattern. Part 1 requires exactly 2 repetitions, while Part 2 allows 2 or more repetitions.

### Example Input
```
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
```

### Example Output
- Part 1: `1227775554`
- Part 2: `4174379265`

## Algorithm & Approach

### Part 1

Check if a number's digits can be split into two equal halves. Convert the number to a string, verify it has an even number of digits, then compare the first and second halves for equality.

**Complexity:**
- Time: O(n × d) where n is the total count of IDs in all ranges and d is the average number of digits
- Space: O(d) for the string representation

For each ID, we convert to string (O(d)) and compare halves (O(d)).

### Part 2

Extend the pattern matching to detect any repeating subsequence. Try all possible pattern lengths from 1 to len/2, checking if repeating each pattern produces the original number. This captures patterns repeated 2, 3, 4, or more times.

**Complexity:**
- Time: O(n × d²) where n is the total count of IDs and d is the average number of digits
- Space: O(d) for string operations

For each ID, we try up to d/2 pattern lengths, and for each we do O(d) string operations.

## Implementation Notes

**String-based pattern matching**: Converting numbers to strings simplifies pattern detection. While this adds overhead compared to pure arithmetic, it makes the logic clearer and handles arbitrary-length patterns elegantly.

**Efficient iteration**: Using `flat_map` with `filter` allows processing all ranges in a functional pipeline, summing only invalid IDs in a single pass.

**Pattern reuse**: Part 2 naturally extends Part 1 - a pattern repeated exactly twice is just a special case of a pattern repeated at least twice. However, Part 1's optimized half-split check is kept for reference.

**Edge cases handled**:
- Numbers with odd digit counts (can't have even split for Part 1)
- Single-digit patterns (1111 = "1" × 4)
- Large numbers (using u64 for IDs up to ~10^19)

## Alternative Approaches Considered

1. **Arithmetic pattern detection**: Instead of strings, could use modulo and division to extract pattern. More efficient but significantly more complex for variable-length patterns.

2. **Precompute invalid IDs**: Could generate all possible invalid IDs up to a limit. Would be faster for multiple queries but impractical given the large range (up to 10^10+).

---

**Key Takeaways:**
- String manipulation can simplify pattern-matching problems even with numeric input
- Functional iteration with `flat_map` and `filter` creates clean, composable pipelines
- Part 2's general solution (any repetition count) subsumes Part 1's specific case (exactly 2 repetitions)
