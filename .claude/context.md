# Advent of Code 2025 - Claude Code Context

This file provides comprehensive guidance for Claude Code sessions working on this project. Read this file at the start of each session to understand the project structure, workflow, and quality standards.

## Project Overview

This is a Rust CLI application that solves Advent of Code 2025 puzzles with a focus on:
- **Performance**: Solutions optimized for speed without sacrificing readability
- **Education**: Comprehensive documentation explaining algorithms and trade-offs
- **Code Quality**: Well-tested, well-commented, maintainable code
- **Benchmarking**: When multiple algorithms exist, benchmark them to choose the best

## Session Initialization Protocol

When starting a new session to implement a day's puzzle:

1. **Read this context file first** to understand project guidelines
2. **Read README.md** for project structure overview
3. **Determine which day to implement**: Check user input or find the next unimplemented day
4. **Verify input files exist**: Check that `src/days/dayNN/input/puzzle.txt` and `input.txt` exist
   - If missing, STOP and ask the user to provide them
   - DO NOT proceed without input files
5. **Review recent implementations**: Read 1-2 recent day solutions to understand patterns
6. **Check for common utilities**: Review `src/common/mod.rs` for existing helpers

Your context window will be automatically compacted as it approaches its limit, allowing you to continue working indefinitely from where you left off.

## Core Behavioral Directives

<default_to_action>
When solving AoC puzzles, default to implementing solutions directly after reading the problem
and creating a brief implementation plan. Do not over-explain before coding. The user values
action over discussion. Write code first, explain after if needed.
</default_to_action>

<code_quality>
Prioritize readable, well-commented code over cleverness. Balance performance with clarity.

Guidelines:
- Add inline comments for non-obvious logic (bit manipulation, math tricks, etc.)
- Use descriptive variable names (prefer `target_sum` over `t`)
- Break complex expressions into intermediate variables with clear names
- Document time/space complexity in module-level comments
- Add doc comments for public functions explaining what they do and why

Why this matters: Future developers (including you in later sessions) need to understand the
code quickly. AoC solutions can be dense; comments make them educational.
</code_quality>

<testing_approach>
Test-driven development is expected for all puzzle solutions.

Process:
1. Write tests for example inputs FIRST (from puzzle description)
2. Ensure tests pass before running on actual input
3. Add unit tests for helper functions
4. Run benchmarks when multiple approaches exist
5. Keep losing benchmark implementations in code (as alternate solutions)

Why this matters: Example inputs catch logic errors early. Real puzzle inputs are large and
hard to debug. Tests also serve as documentation of expected behavior.
</testing_approach>

<refactoring_directive>
Proactively move common patterns to src/common/.

When you encounter code that could be reused across days (parsing patterns, grid utilities,
graph algorithms, etc.), abstract it immediately into a common module. Then update prior days
to use the new abstraction.

Why this matters: AoC puzzles reuse patterns. Abstracting common code reduces duplication,
makes solutions more maintainable, and speeds up future implementations.
</refactoring_directive>

<dependency_philosophy>
Evaluate each dependency critically: utility vs size vs maintenance burden.

Decision framework:
- If a library does one simple thing we can implement in <50 lines, implement it ourselves
- If a library provides substantial value (complex algorithms, well-tested), use it
- Document dependency decisions in commits or DEVELOPMENT.md
- Prefer std library when possible

Why this matters: Dependencies increase compile time, binary size, and maintenance burden.
AoC solutions should build fast and remain maintainable for years.
</dependency_philosophy>

<git_workflow>
Use git commits to checkpoint progress at logical milestones.

Required commit sequence for each day:
1. Commit input files (puzzle.txt, input.txt) BEFORE starting implementation
2. Commit working Part 1 solution (with tests passing)
3. Commit working Part 2 solution (with tests passing)
4. Commit refactoring separately (if moving code to common/)
5. Commit documentation (README.md for the day)

Commit message format: "dayNN: brief description"
Examples: "day01: implement part1", "day01: refactor grid parsing to common", "day01: add documentation"

Why this matters: Clear git history makes it easy to review changes, understand what broke,
and revert if needed. It's also educational for others reading the repository.
</git_workflow>

## Daily Implementation Checklist

⚠️ **CRITICAL: ALL 8 PHASES ARE REQUIRED** ⚠️

Do not stop after Phase 3 just because the code works. Documentation (Phase 6) and Quality Checks (Phase 7) are mandatory, not optional. A day is not complete until all phases are done.

When implementing a day's puzzle, follow this sequence:

### Phase 1: Setup & Input Verification
- [ ] Verify `src/days/dayNN/input/puzzle.txt` exists (contains problem statement)
- [ ] Verify `src/days/dayNN/input/input.txt` exists (contains actual puzzle input)
- [ ] Commit input files with message: "dayNN: add puzzle input"
- [ ] Create day module structure: `src/days/dayNN/mod.rs` and `solution.rs`

### Phase 2: Part 1 Implementation
- [ ] Read puzzle.txt and understand Part 1 requirements
- [ ] Extract example input/output from puzzle description
- [ ] Create test for example input in `solution.rs`
- [ ] Implement Part 1 solution
- [ ] Run tests until example passes
- [ ] Run Part 1 on actual input
- [ ] Verify answer is correct (submit to AoC if first attempt)
- [ ] Commit with message: "dayNN: implement part1"

### Phase 3: Part 2 Implementation
- [ ] Read Part 2 requirements (unlocked after Part 1 correct)
- [ ] Add example test for Part 2
- [ ] Implement Part 2 solution
- [ ] Run tests until example passes
- [ ] Run Part 2 on actual input
- [ ] Verify answer is correct
- [ ] Commit with message: "dayNN: implement part2"

**⚠️ STOP: Before continuing, create todos for Phases 6 & 7 using TodoWrite. The day is NOT complete yet.**

### Phase 4: Optimization & Benchmarking (if applicable)
- [ ] If solution is slow (>1s), profile and optimize
- [ ] If multiple algorithms considered, implement alternatives
- [ ] Write benchmark tests comparing approaches
- [ ] Run benchmarks: `just bench-day NN`
- [ ] Keep losing implementations in code (commented or feature-gated)
- [ ] Document benchmark results in day's README.md
- [ ] Commit with message: "dayNN: optimize and benchmark"

### Phase 5: Refactoring
- [ ] Identify common patterns that could be reused
- [ ] Move reusable code to appropriate module in `src/common/`
- [ ] Update current day to use new common module
- [ ] Check if previous days could use new abstractions
- [ ] Update previous days if beneficial
- [ ] Commit with message: "dayNN: refactor parsing to common" (or similar)

### Phase 6: Documentation
- [ ] Write day's README.md using template (`.claude/templates/day_readme.md`)
- [ ] Include: problem summary, algorithm explanation, complexity analysis
- [ ] Document interesting Rust patterns or techniques used
- [ ] Add benchmark results (if applicable)
- [ ] Include mermaid diagrams if they aid understanding
- [ ] Keep documentation concise (respect reader's time)
- [ ] Commit with message: "dayNN: add documentation"

### Phase 7: Quality Check

**⚠️ ALWAYS use `just` commands, not raw `cargo` commands.**

- [ ] **Run `just ci`** - This runs fmt-check, lint, test, and build in one command
- [ ] If fmt-check fails, run `just fmt` to fix formatting
- [ ] Run benchmarks: `just bench` (or `just bench-day N`)
- [ ] Generate and review rustdoc: `just doc`
- [ ] Verify no `todo!()` or `unimplemented!()` in committed code
- [ ] Final commit if fixes needed: "dayNN: final cleanup"

The `just ci` command is equivalent to the full CI pipeline and should be your default quality check.

### Phase 8: Register Day
- [ ] Update `src/days/mod.rs` to include new day module
- [ ] Add day to registry in `get_days()` function
- [ ] Run `just run-day NN` to verify day executes correctly
- [ ] Run `just run` to verify integration with other days
- [ ] Commit with message: "dayNN: register in day registry"

## Performance Evaluation Criteria

After implementing a day, evaluate against these criteria:

### Speed
- ✅ Both parts complete in <1s combined (ideal)
- ⚠️ Both parts complete in <5s (acceptable)
- ❌ Either part takes >5s (requires optimization)

### Algorithmic Correctness
- ✅ Time complexity is optimal for input size
- ✅ Space complexity is reasonable (no unnecessary allocations)
- ✅ Algorithm handles edge cases correctly

### Code Quality
- ✅ Code is readable and well-commented
- ✅ Variable names are descriptive
- ✅ Complex logic has explanatory comments
- ✅ No unnecessary cleverness

### Testing
- ✅ Example inputs have tests
- ✅ Tests pass
- ✅ Helper functions have unit tests (if non-trivial)

### Documentation (in day's README.md)
- ✅ Problem clearly explained
- ✅ Algorithm approach described with rationale
- ✅ Time/space complexity documented
- ✅ If benchmarked: results summarized

## Documentation Quality Checklist

When writing a day's README.md, ensure it includes:

### Required Sections
1. **Problem Statement**: 2-3 sentence summary of the puzzle
2. **Example Input/Output**: Show example from puzzle description
3. **Algorithm & Approach**: Explain the solution strategy
4. **Complexity Analysis**: Document time/space complexity
5. **Implementation Notes**: Highlight interesting Rust patterns or challenges

### Optional Sections (include when relevant)
6. **Benchmark Results**: If multiple approaches were tested, show comparison
7. **Alternative Approaches**: Briefly discuss other solutions considered
8. **Visualizations**: Mermaid diagrams for complex algorithms

### Quality Standards
- **Concise**: Respect reader's time; be thorough but not verbose
- **Educational**: Explain *why* choices were made, not just *what* was done
- **Accurate**: Ensure complexity analysis is correct
- **Well-formatted**: Use markdown properly; code blocks with language tags
- **Complete**: Don't leave TODOs or placeholders in documentation

## Common Patterns in AoC Puzzles

Be aware of these recurring patterns and consider abstracting them to `src/common/`:

### Parsing
- Line-by-line input processing
- Regex-based parsing
- Split-and-parse patterns
- Grid/2D array parsing

### Data Structures
- 2D grids (with neighbor iteration, bounds checking)
- Graphs (adjacency list/matrix representations)
- Trees (traversal, manipulation)

### Algorithms
- BFS/DFS (pathfinding, reachability)
- Dijkstra/A* (shortest path)
- Dynamic programming
- Binary search
- Simulation (state machines, game loops)

### Math
- GCD/LCM
- Modular arithmetic
- Prime factorization
- Combinatorics (permutations, combinations)

### String Manipulation
- Pattern matching
- String transformations
- Hashing

## Common Pitfalls to Avoid

### Implementation Pitfalls
- ❌ Implementing before input files exist
- ❌ Not testing with example inputs first
- ❌ Over-engineering simple problems
- ❌ Leaving `todo!()` or panics in production code
- ❌ Forgetting to register day in `src/days/mod.rs`

### Performance Pitfalls
- ❌ Using naive algorithms without considering input size
- ❌ Unnecessary allocations in hot loops
- ❌ Not benchmarking when performance matters
- ❌ Optimizing before profiling (premature optimization)

### Documentation Pitfalls
- ❌ Writing docs longer than the code for simple problems
- ❌ Skipping complexity analysis
- ❌ Not documenting benchmark results
- ❌ Using jargon without explanation

### Git Pitfalls
- ❌ Committing multiple unrelated changes together
- ❌ Vague commit messages ("fix stuff", "wip")
- ❌ Not committing input files separately
- ❌ Forgetting to commit documentation

## Reflection After Tool Use

After reading files, running tests, or executing benchmarks, pause to evaluate results:

**Questions to ask:**
- Did the output match expectations?
- Are there errors or edge cases to handle?
- What's the optimal next step?
- Should any assumptions be revised?
- Is the performance acceptable?

This reflection prevents rushing ahead when something unexpected occurred and ensures quality throughout the implementation process.

## Project Structure Reference

```
rust/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library root
│   ├── runner.rs            # Day execution and timing
│   ├── table.rs             # ASCII table formatting
│   ├── common/              # Shared utilities
│   │   └── mod.rs          # Common module root
│   └── days/
│       ├── mod.rs           # Day registry
│       └── dayNN/           # Each day's solution
│           ├── mod.rs       # Day module root
│           ├── solution.rs  # Part 1 & 2 implementations
│           ├── README.md    # Educational documentation
│           └── input/
│               ├── puzzle.txt   # Problem statement
│               └── input.txt    # Actual puzzle input
├── benches/
│   └── day_benches.rs       # Benchmark harness
├── tests/
│   └── integration_tests.rs # Integration tests
├── scripts/
│   └── new_day.sh           # Scaffold new day
├── .claude/
│   ├── context.md           # This file
│   └── templates/
│       └── day_readme.md    # Template for daily docs
├── Cargo.toml               # Project manifest
├── rust-toolchain.toml      # Nightly Rust requirement
├── Justfile                 # Development tasks
└── README.md                # Project overview
```

## Useful Commands (via Justfile)

**⚠️ ALWAYS use `just` commands instead of raw `cargo` commands.**

```bash
# Daily workflow
just new-day N     # Scaffold new day structure
just run-day N     # Run specific day's solution
just test-day N    # Run tests for specific day

# Quality & CI
just ci            # ⭐ Full CI check (fmt-check, lint, test, build) - USE THIS!
just fmt           # Format code (use if fmt-check fails in ci)
just lint          # Run clippy with warnings as errors
just test          # Run all tests

# Other useful commands
just run           # Run all implemented days
just bench         # Run all benchmarks
just bench-day N   # Run benchmarks for specific day
just doc           # Generate and open documentation
```

**Most important:** Use `just ci` before committing to ensure code passes all checks.

## Template Reference

When creating a new day, follow this structure:

**src/days/dayNN/mod.rs:**
```rust
//! Day NN: [Puzzle Title]
//!
//! [Brief description of the puzzle]

mod solution;

pub use solution::DayNN;
```

**src/days/dayNN/solution.rs:**
```rust
//! Solution implementation for Day NN

use crate::runner::Day;

/// Solver for Day NN
pub struct DayNN;

impl Day for DayNN {
    fn part1(&self, input: &str) -> String {
        // Implementation
        todo!()
    }

    fn part2(&self, input: &str) -> String {
        // Implementation
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "example input";

    #[test]
    fn test_part1() {
        let day = DayNN;
        assert_eq!(day.part1(EXAMPLE), "expected");
    }

    #[test]
    fn test_part2() {
        let day = DayNN;
        assert_eq!(day.part2(EXAMPLE), "expected");
    }
}
```

## Communication Style

- Be concise and action-oriented
- Report progress factually without unnecessary celebration
- Explain errors clearly with proposed fixes
- Ask questions when requirements are unclear
- Provide summaries after tool use when helpful

## Questions or Issues?

If you encounter situations not covered by this context:
1. Check README.md for project structure details
2. Review recent day implementations for patterns
3. Ask the user for clarification rather than making assumptions
4. Document the decision in this context file for future sessions

---

This context file should evolve as the project grows. Update it when new patterns emerge or guidelines need clarification.
