#!/bin/bash
# Script to scaffold a new Advent of Code day
# Usage: ./scripts/new_day.sh <day_number>

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if day number is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Day number required${NC}"
    echo "Usage: $0 <day_number>"
    echo "Example: $0 1"
    exit 1
fi

DAY_NUM=$1
DAY_PADDED=$(printf "%02d" "$DAY_NUM")
DAY_DIR="src/days/day$DAY_PADDED"

# Validate day number
if ! [[ "$DAY_NUM" =~ ^[0-9]+$ ]] || [ "$DAY_NUM" -lt 1 ] || [ "$DAY_NUM" -gt 25 ]; then
    echo -e "${RED}Error: Day must be between 1 and 25${NC}"
    exit 1
fi

# Check if day already exists
if [ -d "$DAY_DIR" ]; then
    echo -e "${YELLOW}Warning: Day $DAY_PADDED already exists at $DAY_DIR${NC}"
    read -p "Overwrite? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted"
        exit 0
    fi
fi

# Create directory structure
echo -e "${GREEN}Creating directory structure...${NC}"
mkdir -p "$DAY_DIR/input"

# Create mod.rs
echo -e "${GREEN}Creating mod.rs...${NC}"
cat > "$DAY_DIR/mod.rs" << EOF
//! Day $DAY_NUM: [Puzzle Title]
//!
//! [Brief description of the puzzle]
//!
//! ## Problem Summary
//!
//! TODO: Add problem summary
//!
//! ## Algorithm
//!
//! TODO: Describe algorithm approach

mod solution;

pub use solution::Day$DAY_PADDED;
EOF

# Create solution.rs
echo -e "${GREEN}Creating solution.rs...${NC}"
cat > "$DAY_DIR/solution.rs" << 'EOF'
//! Solution implementation for Day DAY_NUM

use crate::runner::Day;

/// Solver for Day DAY_NUM
pub struct DayDAY_PADDED;

impl Day for DayDAY_PADDED {
    fn part1(&self, input: &str) -> String {
        // TODO: Implement Part 1
        todo!("Implement part1")
    }

    fn part2(&self, input: &str) -> String {
        // TODO: Implement Part 2
        todo!("Implement part2")
    }
}

// Helper functions

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
TODO: Add example input from puzzle";

    #[test]
    fn test_part1_example() {
        let day = DayDAY_PADDED;
        assert_eq!(day.part1(EXAMPLE), "TODO");
    }

    #[test]
    #[ignore] // Remove this when Part 2 is unlocked
    fn test_part2_example() {
        let day = DayDAY_PADDED;
        assert_eq!(day.part2(EXAMPLE), "TODO");
    }
}

#[cfg(test)]
mod benches {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = include_str!("input/input.txt");
        let day = DayDAY_PADDED;
        b.iter(|| day.part1(input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = include_str!("input/input.txt");
        let day = DayDAY_PADDED;
        b.iter(|| day.part2(input));
    }
}
EOF

# Replace placeholders in solution.rs
sed -i.bak "s/DAY_NUM/$DAY_NUM/g" "$DAY_DIR/solution.rs"
sed -i.bak "s/DAY_PADDED/$DAY_PADDED/g" "$DAY_DIR/solution.rs"
rm "$DAY_DIR/solution.rs.bak"

# Create README.md from template
echo -e "${GREEN}Creating README.md...${NC}"
if [ -f ".claude/templates/day_readme.md" ]; then
    cp ".claude/templates/day_readme.md" "$DAY_DIR/README.md"
    # Replace day number placeholders
    sed -i.bak "s/NN/$DAY_PADDED/g" "$DAY_DIR/README.md"
    sed -i.bak "s/N:/$DAY_NUM:/g" "$DAY_DIR/README.md"
    rm "$DAY_DIR/README.md.bak"
else
    # Create basic README if template doesn't exist
    cat > "$DAY_DIR/README.md" << EOF
# Day $DAY_NUM: [Puzzle Title]

## Problem Statement

TODO: Add concise summary of the puzzle

### Example Input
\`\`\`
TODO: Add example
\`\`\`

### Example Output
- Part 1: \`TODO\`
- Part 2: \`TODO\`

## Algorithm & Approach

### Part 1
TODO: Explain approach

**Complexity:**
- Time: O(?)
- Space: O(?)

### Part 2
TODO: Explain approach

**Complexity:**
- Time: O(?)
- Space: O(?)

## Implementation Notes

TODO: Highlight interesting Rust patterns or techniques

## Benchmark Results

TODO: Add benchmark results if multiple approaches tested

## Alternative Approaches Considered

TODO: Discuss other approaches and trade-offs
EOF
fi

# Create placeholder input files
echo -e "${GREEN}Creating placeholder input files...${NC}"
cat > "$DAY_DIR/input/puzzle.txt" << EOF
TODO: Paste the puzzle description from https://adventofcode.com/2025/day/$DAY_NUM
EOF

cat > "$DAY_DIR/input/input.txt" << EOF
TODO: Paste your puzzle input from https://adventofcode.com/2025/day/$DAY_NUM/input
EOF

# Success message
echo -e "${GREEN}✓ Day $DAY_PADDED scaffolding created successfully!${NC}"
echo
echo "Next steps:"
echo "  1. Add puzzle description to: $DAY_DIR/input/puzzle.txt"
echo "  2. Add your input to: $DAY_DIR/input/input.txt"
echo "  3. Register day in: src/days/mod.rs"
echo "     - Add: pub mod day$DAY_PADDED;"
echo "     - Register in get_days() function"
echo "  4. Implement solution in: $DAY_DIR/solution.rs"
echo "  5. Run tests: just test-day $DAY_PADDED"
echo
echo -e "${YELLOW}Remember: Commit input files before implementing!${NC}"
