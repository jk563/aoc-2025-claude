# Day 10: Factory Initialization

## Problem Statement

A factory has machines that need initialization through button presses. Part 1 is a "lights-out" puzzle where buttons toggle indicator lights (XOR operations). Part 2 switches to incrementing numeric counters instead of toggling lights. For both parts, we need to find the minimum number of button presses to configure all machines.

### Example Input
```
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
```

Each line contains:
- `[...]`: Target light pattern (Part 1) - `.` = off, `#` = on
- `(...)`: Button definitions - comma-separated indices of lights/counters they affect
- `{...}`: Target counter values (Part 2)

### Example Output
- Part 1: `7` (total minimum button presses for all machines)
- Part 2: `33` (total minimum button presses for counter configuration)

## Algorithm & Approach

### Part 1: Lights-Out with BFS

**Problem:** Toggle lights from all-off to match a target pattern, minimizing button presses.

**Key insight:** This is a shortest-path problem in state space where each state is a configuration of lights (on/off). We use BFS to explore states level-by-level, guaranteeing we find the minimum number of presses.

**Data structures:**
- **Bitmask representation**: Each light configuration is a `u32` where bit `i` represents light `i`
- **XOR operations**: Button press = `state ^ button_mask` (toggles all affected lights)
- **HashSet for visited states**: Prevents revisiting configurations
- **VecDeque for BFS queue**: FIFO exploration of state space

**Algorithm:**
1. Start at state 0 (all lights off)
2. For each state, try pressing each button (XOR operation)
3. If we reach the target, return the current depth (number of presses)
4. Mark states as visited to avoid cycles
5. BFS guarantees the first path found is shortest

**Complexity:**
- Time: O(2^n × b) where n = number of lights, b = number of buttons
  - Worst case visits all 2^n possible states
  - Each state tries b buttons
  - Typical input: n=6-9 lights, b=6-12 buttons → ~10^5 operations
- Space: O(2^n) for visited set
  - Stores at most all possible light configurations

**Why BFS over DFS or Dijkstra:**
- BFS finds shortest path without weights (all button presses cost 1)
- DFS would find *a* solution but not necessarily the shortest
- Dijkstra is overkill when all edges have equal weight

### Part 2: Counter Increment with Integer Linear Programming

**Problem:** Increment counters from 0 to target values, minimizing total button presses.

**Key insight:** This is fundamentally different from Part 1! Counters only increment (no toggles), and we need exact values. This maps perfectly to Integer Linear Programming (ILP).

**Why not BFS?**
- State space explodes: O(∏ target[i]) states
- Example: targets {168,164,176,171,51,173,194,30,168} = 10^18+ states
- BFS would run for years

**Mathematical model:**
- **Variables:** x[i] = number of times button i is pressed (integer ≥ 0)
- **Objective:** minimize Σ x[i] (total button presses)
- **Constraints:** For each counter j, Σ x[i] (where button i affects counter j) = target[j]

**Example:**
```
Buttons: (0,1), (1,2), (2)
Targets: {5, 7, 3}

Variables: x₀, x₁, x₂ ≥ 0 (integer)
Minimize: x₀ + x₁ + x₂
Subject to:
  x₀           = 5  (counter 0)
  x₀ + x₁      = 7  (counter 1)
       x₁ + x₂ = 3  (counter 2)

Solution: x₀=5, x₁=2, x₂=1 → total = 8 presses
```

**Implementation:**
1. Create integer variable for each button
2. Build constraint matrix: for each counter, sum of buttons affecting it = target
3. Solve using branch-and-bound (microlp solver)
4. Extract and sum solution values

**Complexity:**
- Time: O(b³) for simplex, × exponential factor for integer constraints via branch-and-bound
  - Typical input: b=8-12 buttons → solves in milliseconds
  - Branch-and-bound explores solution tree but prunes aggressively
- Space: O(b × c) where c = number of counters (constraint matrix)
  - Stores sparse matrix of button-counter relationships

**Critical bug fix:** Must use `.integer()` constraint! Without it:
- Solver finds continuous relaxation (e.g., x₀=2.5, x₁=3.7)
- Rounding these values doesn't guarantee constraints are satisfied
- Results in "too low" answers (our bug gave 16756 instead of 16757)

## Implementation Notes

### Part 1: Bit Manipulation Mastery

**Bitmask parsing:**
```rust
let target = pattern.chars().enumerate().fold(0u32, |acc, (i, ch)| {
    if ch == '#' { acc | (1 << i) } else { acc }
});
```
Uses fold to build bitmask left-to-right, setting bit `i` for each `#`.

**XOR for toggling:**
```rust
let next_state = state ^ button;  // Toggle all lights affected by button
```
XOR is perfect for toggle operations: 0^1=1 (off→on), 1^1=0 (on→off).

### Part 2: ILP with good_lp

**Creating integer variables:**
```rust
let button_vars: Vec<Variable> = (0..num_buttons)
    .map(|_| vars.add(variable().min(0).integer()))  // ← .integer() is critical!
    .collect();
```

**Building constraints functionally:**
```rust
for (counter_idx, &target) in targets.iter().enumerate() {
    let constraint: Expression = buttons
        .iter()
        .enumerate()
        .filter(|(_, button)| button.contains(&counter_idx))
        .map(|(button_idx, _)| button_vars[button_idx])
        .sum();
    constraints.push(constraint.eq(target as f64));
}
```
Elegant use of iterators to build constraint for each counter.

**Why `.round()` is still necessary:**
Even with `.integer()` constraint, the solver returns `f64` values (e.g., 5.0) which may have floating-point precision errors (5.0000000001 or 4.9999999999). Rounding handles this gracefully.

### Rust Patterns Showcased

1. **Functional iteration:** Heavy use of `.map()`, `.filter()`, `.fold()`, `.sum()`
2. **Type safety:** Separate `Machine` and `MachinePart2` structs prevent mixing incompatible data
3. **Parse-don't-validate:** Parsing directly into appropriate representations (bitmasks vs. index vectors)
4. **Zero-cost abstractions:** Bitmask operations compile to single CPU instructions

### Edge Cases Handled

- **Already at target:** Part 1 checks `target == 0` early (line 120)
- **All zeros:** Part 2 checks `targets.iter().all(|&t| t == 0)` (line 217)
- **Empty button lists:** Parsing handles variable-length button lists gracefully

## Performance Results

| Part | Time | Algorithm | Key Operation |
|------|------|-----------|---------------|
| Part 1 | ~0.9 ms | BFS on bitmasks | XOR for state transitions |
| Part 2 | ~7 ms | ILP with branch-and-bound | Simplex + integer constraints |
| **Total** | **~8 ms** | | |

**Why Part 2 is slower:**
- Branch-and-bound explores solution tree (exponential worst-case)
- However, the constraint matrix structure allows aggressive pruning
- Still very fast for problem size (30 machines × 8-12 buttons)

**Optimization note:** Compiled with `--release` is essential. Debug mode is 10-100× slower for both algorithms.

## Alternative Approaches Considered

### Part 1 Alternatives

1. **Gaussian elimination over GF(2):** Treat as system of linear equations over binary field
   - **Why not:** More complex to implement, same complexity, BFS is clearer
   - **When useful:** If you need to determine *solvability* without finding solution

2. **A\* search:** Use heuristic like Hamming distance to target
   - **Why not:** Overhead of priority queue isn't justified for small state spaces
   - BFS is simpler and fast enough (sub-millisecond)

### Part 2 Alternatives

1. **Dynamic programming:** Build up solutions for increasing counter values
   - **Why not:** State space O(∏ target[i]) is exponentially large
   - Example: targets ~200^8 = 10^18 states (infeasible)

2. **Greedy approach:** Always press button that makes most progress
   - **Why not:** Doesn't guarantee optimal solution
   - Can get stuck in local minima (like pressing buttons that overshoot targets)

3. **External MILP solvers** (CBC, GLPK, Gurobi):
   - **Why not:** Adds C dependencies, harder to cross-compile
   - **When useful:** Problems with thousands of variables/constraints
   - Our problem size (8-12 variables, 4-10 constraints) is tiny for ILP

4. **Minimum cost flow:** Model as flow network with capacities
   - **Why not:** Problem structure doesn't perfectly fit flow model
   - ILP is more natural and equally fast

## Lessons Learned

### The Integer Constraint Bug

**Original bug:** Variables defined without `.integer()` constraint
```rust
vars.add(variable().min(0))  // ❌ Continuous LP
```

**Consequence:** Solver found solutions like x₀=2.5, x₁=3.7
- Rounding gave invalid solutions (constraints not satisfied)
- Produced "too low" answer (16756 instead of 16757)

**Fix:** Add `.integer()` to force MILP
```rust
vars.add(variable().min(0).integer())  // ✅ Integer LP
```

**Key insight:** LP relaxation provides *lower bound* on integer optimum. Rounding this bound doesn't give a valid integer solution!

### When to Use ILP

ILP shines when:
- ✅ Problem has clear objective function (minimize/maximize)
- ✅ Constraints are linear equations or inequalities
- ✅ Variables should be integers
- ✅ State space is too large for search algorithms

Not ideal when:
- ❌ Problem is naturally graph-based (use graph algorithms)
- ❌ Constraints are non-linear (need non-linear programming)
- ❌ Problem size is massive (thousands of variables)

### Algorithm Selection Strategy

1. **Small state space + shortest path** → BFS/Dijkstra
2. **Large state space + linear constraints** → ILP/LP
3. **Optimization with complex structure** → Dynamic programming
4. **Feasibility only** → Constraint satisfaction (SAT/SMT)

---

**Key Takeaways:**
- **Part 1:** Bitmasks + XOR + BFS is the elegant solution for toggle-based puzzles
- **Part 2:** Recognize when a problem maps to ILP - the solver does the heavy lifting
- **Integer constraints matter:** Always use `.integer()` for discrete optimization problems
- **Choose the right tool:** BFS for small spaces, ILP for large spaces with linear structure
