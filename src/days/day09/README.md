# Day 9: Movie Theater

## Problem Statement

The movie theater has a tile floor with red tiles scattered across it. Part 1 asks for the largest rectangle using any two red tiles as opposite corners. Part 2 constrains rectangles to only include red or green tiles, where red tiles form a connected polygon and green tiles are on the polygon edges or inside it.

### Example Input
```
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
```

### Example Output
- Part 1: `50`
- Part 2: `24`

## Algorithm & Approach

### Part 1

Part 1 is a straightforward brute force approach: check all pairs of red tiles as potential opposite corners and calculate the area.

**Data Structures:**
- `Vec<(i32, i32)>` for storing red tile coordinates

**Algorithm:**
1. Parse input to extract tile coordinates
2. For each pair of tiles (i, j):
   - Calculate rectangle width and height (inclusive bounds)
   - Compute area = width × height
3. Track and return the maximum area found

**Complexity:**
- Time: O(n²) where n is the number of red tiles
- Space: O(n) for storing tile coordinates

The quadratic time comes from checking all possible pairs of tiles. Since we only need to calculate areas (no validation), this is very fast in practice.

### Part 2

Part 2 introduces geometric constraints: rectangles can only contain red or green tiles. The red tiles form a polygon (connected in order), and green tiles are on the polygon edges or inside it. This requires sophisticated computational geometry.

**Data Structures:**
- `Vec<(i32, i32)>` for the ordered polygon vertices (red tiles)
- `HashSet<(i32, i32)>` for O(1) red tile lookup
- `HashMap<(i32, i32), bool>` for caching expensive point validation results

**Algorithm:**
1. Parse input to get red tiles in order (forming polygon)
2. Create HashSet of tiles for fast lookups
3. Compute polygon bounding box for early rejection
4. For each pair of red tiles as corners:
   - Determine rectangle bounds
   - Early reject if outside bounding box
   - Validate all rectangle points are red or green:
     - Check all four corners (critical!)
     - Sample boundary points adaptively
     - Sample interior points at key locations
   - If valid, update maximum area
5. Return maximum area found

**Key Geometric Techniques:**

**Ray Casting** for point-in-polygon:
```rust
// Count intersections of horizontal ray with polygon edges
if ((y1 > y) != (y2 > y)) && (x < (x2 - x1) * (y - y1) / (y2 - y1) + x1) {
    inside = !inside;
}
```

**Cross Product** for point-on-segment:
```rust
// Check collinearity: cross product should be zero
dx1 as i64 * dy2 as i64 == dy1 as i64 * dx2 as i64
```

**Adaptive Sampling Strategy:**
- Small rectangles (≤10,000 area): Check all boundary points
- Large rectangles: Sample every ~200th point along edges
- Always check all four corners and key interior points (center, quadrants)

**Caching:**
Point validation is expensive (involves polygon edge checks and ray casting). The cache prevents redundant calculations when the same point is checked multiple times across different rectangle candidates.

**Complexity:**
- Time: O(n² × (w + h + m)) where:
  - n = number of red tiles
  - w, h = rectangle dimensions
  - m = polygon complexity
- Space: O(n + k) where k is cache size (bounded by unique points checked)

The actual runtime is dominated by geometric validation rather than the O(n²) pair enumeration.

## Implementation Notes

This problem showcases several computational geometry algorithms and Rust optimization patterns:

**Computational Geometry:**
- **Ray Casting Algorithm**: Classic technique for point-in-polygon testing. Cast a horizontal ray from the point and count edge intersections; odd count = inside
- **Cross Product Collinearity**: Using `dx1 * dy2 == dy1 * dx2` elegantly determines if three points are collinear, critical for point-on-segment checks
- **Integer Arithmetic**: All calculations use i32/i64, avoiding floating-point precision issues

**Rust Patterns:**
- **HashSet Optimization**: Replacing O(n) linear search with O(1) hash lookups for tile membership testing
- **Memoization Pattern**: `HashMap<Point, bool>` caches expensive geometric calculations. Simple but effective
- **Integer Overflow Prevention**: Using `as i64` casts in multiplication to prevent overflow when computing areas and cross products

**Adaptive Algorithms:**
- Rectangle validation uses different strategies based on size:
  - Small rectangles: Exhaustive boundary check
  - Large rectangles: Sparse sampling to avoid O(w × h) costs
- Always validates critical points (corners, center, quadrants) regardless of size

**Edge Cases Handled:**
1. **Points on polygon edges**: Requires separate line segment intersection check
2. **Points exactly on vertices**: Handled by both vertex set and edge checks
3. **Collinear polygon edges**: Cross product correctly handles this
4. **Large coordinate ranges**: Using i64 for intermediate calculations prevents overflow

**Alternative Implementations Included:**

The codebase includes two experimental approaches (currently WIP with failing tests):

1. **Sweep Line (`Day09SweepLine`)**: Sort tiles by x-coordinate to reduce pairs checked
2. **Spatial Grid (`Day09SpatialGrid`)**: Pre-compute validity grid for faster point queries

These demonstrate different optimization strategies but need debugging before use.

## Performance

Benchmarked on the full puzzle input:

| Part | Time | Notes |
|------|------|-------|
| Part 1 | 209 μs | Simple O(n²) pair enumeration |
| Part 2 | 1.17 s | Dominated by geometric validation |

**Part 1 Performance:**
The ~200 μs time comes from checking ~40,000 pairs (n ≈ 200 tiles). Area calculation is trivial, so this is essentially the cost of iteration.

**Part 2 Performance:**
The 1.17 second runtime reflects the computational cost of geometric validation:
- Same O(n²) pair enumeration as Part 1
- Each pair requires validating rectangle bounds:
  - Point-in-polygon tests (ray casting)
  - Point-on-segment tests (cross product)
  - Multiple sample points per rectangle
- Caching provides significant speedup (without it, runtime would be several seconds)
- Large rectangles benefit from sparse sampling

**Why Part 2 is ~5,500× slower:**
Not due to algorithmic complexity (same O(n²) pairs), but per-pair cost:
- Part 1: Simple arithmetic (width × height)
- Part 2: Geometric validation involving polygon edge iteration, ray casting, and multiple point checks

## Alternative Approaches Considered

### 1. Sweep Line Algorithm
**Concept**: Sort tiles by x-coordinate and only check pairs where one tile is to the left of another.

**Implementation**: Included in the codebase as `Day09SweepLine` but currently buggy (returns 8 instead of 24 on test case).

**Trade-offs:**
- **Potential benefit**: Fewer pairs to check if sorting enables pruning
- **Reality**: Still O(n²) in worst case; sorting overhead (O(n log n)) is negligible compared to geometric validation
- **When useful**: If we had additional spatial pruning heuristics based on x-coordinate ordering

**Why not used**: Tests fail; even if fixed, unlikely to provide significant speedup given validation dominates runtime.

### 2. Spatial Grid Pre-computation
**Concept**: Pre-compute a grid where each cell stores whether points in that region are valid (red/green). Use grid for O(1) point lookups.

**Implementation**: Included as `Day09SpatialGrid` with 500×500 cell size, but currently buggy (returns 0 on test case).

**Trade-offs:**
- **Pre-computation cost**: O(grid_rows × grid_cols) point-in-polygon checks
- **Query benefit**: O(1) lookups instead of O(m) ray casting per point
- **Grid granularity**: Too coarse = inaccurate; too fine = expensive pre-computation
- **Memory**: O(rows × cols) for grid storage

**Why not used**: Tests fail; even if fixed, the pre-computation cost might exceed savings unless validating many rectangles.

### 3. Exact Validation (All Points)
**Concept**: Instead of sampling, validate every single point within the rectangle.

**Trade-offs:**
- **Accuracy**: Perfect (no false positives)
- **Cost**: O(w × h) per rectangle - prohibitive for large rectangles
- **When useful**: Tiny rectangles where sampling overhead dominates

**Why not used**: Adaptive sampling provides good accuracy with much better performance for large rectangles.

### 4. Convex Hull Optimization
**Concept**: If the polygon is convex, use faster point-in-polygon tests.

**Why not considered**: The problem doesn't guarantee convex polygons. Red tiles form a general polygon.

### 5. Dynamic Programming / Memoization by Rectangle
**Concept**: Cache validation results for entire rectangles, not just points.

**Why not used**: Rectangles are defined by arbitrary pairs of corners - unlikely to see repeated rectangles. Point-level caching is more reusable.

## Benchmark Results

Only the main implementation (`Day09`) is benchmarked as the alternative implementations have failing tests:

| Implementation | Part 1 | Part 2 | Status |
|----------------|--------|--------|--------|
| **Day09** (Default + Caching) | 209 μs | 1.17 s | ✓ Working |
| Day09SweepLine | - | - | ✗ WIP (failing tests) |
| Day09SpatialGrid | - | - | ✗ WIP (failing tests) |

**Winner**: Day09 (default implementation)

The working implementation uses HashSet optimization and HashMap caching. The alternative implementations demonstrate interesting optimization ideas but need debugging before they can be properly benchmarked.

---

**Key Takeaways:**
- **Computational Geometry Fundamentals**: Ray casting and cross product are essential building blocks for polygon operations
- **Caching for Expensive Operations**: Memoizing geometric calculations provides substantial speedup when points are checked multiple times
- **Adaptive Algorithms**: Adjusting strategy based on input characteristics (rectangle size) balances accuracy and performance
- **Integer Arithmetic for Geometry**: Using integer types avoids floating-point precision issues while maintaining exactness
