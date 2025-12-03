# Day 3: Lobby

## Problem Statement

Given banks of batteries (lines of digits 1-9), select a specific number of batteries from each bank while maintaining their order to maximize the numeric value formed by the selected digits. Sum the maximum values across all banks.

### Example Input
```
987654321111111
811111111111119
234234234234278
818181911112111
```

### Example Output
- Part 1: `357`
- Part 2: `3121910778619`

## Algorithm & Approach

### Part 1

For part 1, we need to select exactly 2 batteries from each bank to maximize the 2-digit number formed. The approach is straightforward: try all possible pairs (i, j) where i < j and find the maximum.

**Data structures:**
- Parse each character as a digit
- Iterate through all valid pairs

**Algorithm:**
1. For each bank (line), try all pairs of positions (i, j) where i < j
2. Calculate the joltage as `digit[i] * 10 + digit[j]`
3. Track the maximum joltage for each bank
4. Sum all maximum joltages

**Complexity:**
- Time: O(n²) per bank, where n is the bank length (typically ~15)
- Space: O(n) for storing digits

With small bank sizes (~15 digits), the O(n²) approach is efficient enough.

### Part 2

Part 2 requires selecting exactly 12 batteries from each bank. With 15-digit banks, trying all combinations would be C(15,12) = 455 combinations per bank, but we can do much better with a greedy algorithm.

**Key insight:** To maximize a k-digit number, we greedily select the largest available digit at each position, ensuring we leave enough digits remaining for future positions.

**Algorithm (Greedy Selection):**
1. For position p in the result (0 to k-1):
   - We need to select (k - p) more digits
   - We have (n - start) digits remaining
   - We can search the first (n - k + p + 1 - start) positions
   - Find the maximum digit in this range
   - Add it to the result and continue from the next position

**Example:** From `987654321111111` (15 digits), select 12:
- Position 0: Search indices [0..4), find '9' at index 0
- Position 1: Search indices [1..5), find '8' at index 1
- Continue greedily...
- Result: `987654321111` (skipped three 1s at the end)

**Complexity:**
- Time: O(k × n) where k=12 and n≈15, simplifies to O(n²)
- Space: O(n) for storing digits + O(k) for building result string

The greedy approach is optimal because selecting the largest digit early always produces a larger final number (since higher positions have greater place value).

## Implementation Notes

**Rust patterns used:**
- Used `char` vectors for Part 2 to avoid repeated digit conversions
- String building with `String::new()` and `push()` for the result
- Parsed final string to `u64` for large 12-digit numbers

**Type choices:**
- Part 1: `u32` sufficient for 2-digit results
- Part 2: `u64` required for 12-digit numbers (max ~10¹²)

**Performance optimizations:**
- Part 1 uses the simple O(n²) exhaustive search since n is small
- Part 2 uses greedy O(k×n) algorithm instead of trying all C(n,k) combinations
- No caching needed as each bank is independent

**Edge cases handled:**
- All banks in the input have sufficient digits (15+) for selecting 12
- Digits are guaranteed to be 1-9 (no 0s to handle)

## Benchmark Results

| Part | Time | Notes |
|------|------|-------|
| Part 1 | 352 µs | Exhaustive pair search across ~200 banks |
| Part 2 | 183 µs | Greedy selection across ~200 banks |
| **Total** | **535 µs** | Fast enough for the input size |

Part 2 is actually faster than Part 1 despite producing larger numbers, because the greedy algorithm (O(k×n) with k=12) is more efficient than trying all pairs in Part 1's longer search range.

## Alternative Approaches Considered

1. **Dynamic Programming for Part 2**: Could use DP to select k digits, but the greedy approach is both simpler and optimal for this problem. DP would be O(n×k) space and similar time complexity without benefit.

2. **Sliding window maximum for Part 2**: Could maintain a sliding window of valid search ranges, but the greedy approach is already efficient enough and more readable.

3. **Part 1 optimization with suffix maximum**: Could precompute maximum digit in each suffix, reducing Part 1 to O(n) per bank, but with n≈15 the optimization isn't worth the complexity.

---

**Key Takeaways:**
- Greedy algorithms work when local optimal choices lead to global optimality (maximizing leftmost digits maximizes the number)
- Simple O(n²) algorithms are perfectly fine for small inputs - premature optimization wastes time
- Rust's string parsing (`parse::<u64>()`) cleanly handles large numeric conversions

