# Day 1: Secret Entrance

## Problem Statement

We need to determine the password to open a secret entrance to the North Pole base. The password is found by following a sequence of rotations on a circular dial with positions 0-99, starting at position 50. Each rotation is specified as a direction (L for left/decreasing or R for right/increasing) and a distance.

### Example Input
```
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
```

### Example Output
- Part 1: `3`
- Part 2: `6`

## Algorithm & Approach

### Part 1

The task is to count how many times the dial lands on position 0 after completing each rotation.

**Strategy:**
- Maintain current position as we process each rotation
- Use modular arithmetic with `rem_euclid(100)` to handle circular wrapping
- For left rotations: `position = (position - distance) % 100`
- For right rotations: `position = (position + distance) % 100`
- Count each time the final position equals 0

**Key Insight:** Rust's `rem_euclid` correctly handles negative remainders, ensuring the dial wraps from 0 to 99 when going left.

**Complexity:**
- Time: O(n) where n is the number of rotations
- Space: O(1) - only tracking current position and count

### Part 2

Part 2 changes the counting method: instead of counting only when we *land* on 0, we count every time we *pass through* 0 during a rotation. A rotation of R1000 from position 50 would pass through 0 ten times.

**Strategy:**
The key is to count how many multiples of 100 we cross during each rotation.

For **right rotations** from position `start` by `distance`:
- Count = `(start + distance) / 100 - start / 100`
- This counts how many times we cross a multiple of 100

For **left rotations** from position `start` by `distance`:
- If `start == 0`: Count = `distance / 100` (only hit 0 at multiples of 100)
- If `start > 0` and `distance >= start`: Count = `1 + (distance - start) / 100`
  - We hit 0 after `start` steps, then every 100 steps after
- Otherwise: Count = `0` (don't reach 0)

**Complexity:**
- Time: O(n) where n is the number of rotations
- Space: O(1) - same as Part 1

## Implementation Notes

**Modular Arithmetic in Rust:**
Using `rem_euclid` instead of the `%` operator is crucial for correct circular behavior. In Rust, `-5 % 100` returns `-5`, but `(-5).rem_euclid(100)` returns `95`, which is what we need for a circular dial.

**Integer Division for Counting:**
The solution cleverly uses integer division to count boundary crossings. For example, going right from position 52 by 48 means we go from 52 to 100, crossing the 0-boundary once: `(52+48)/100 - 52/100 = 1 - 0 = 1`.

**Edge Case Handling:**
Part 2 requires special handling when starting at position 0 for left rotations, since going left from 0 immediately wraps to 99 without crossing 0.

**Performance:**
Both parts run in under 200 microseconds total, demonstrating that arithmetic operations are extremely fast compared to simulation-based approaches.

## Performance

| Part | Time | Answer |
|------|------|--------|
| Part 1 | 103 µs | 1043 |
| Part 2 | 95 µs | 5963 |
| **Total** | **199 µs** | - |

## Alternative Approaches Considered

1. **Simulation-based approach**: Actually simulate each click of the dial (4,108 total rotations * average distance). This would work but would be much slower and unnecessary given we can calculate crossings mathematically.

2. **String parsing with regex**: Using regex to parse "L68" format. Not chosen because simple character indexing (`line.chars().next()` and `line[1..].parse()`) is more efficient and clearer for this simple format.

---

**Key Takeaways:**
- Modular arithmetic with `rem_euclid` is essential for circular data structures in Rust
- Integer division can elegantly count boundary crossings without simulation
- Simple arithmetic solutions (O(n)) can be orders of magnitude faster than simulation
