//! Solution implementation for Day 6
//!
//! Solves a math worksheet where numbers are arranged vertically in columns.
//! Each column represents a problem with an operation at the bottom row.

use crate::runner::Day;

/// Solver for Day 6
pub struct Day06;

impl Day for Day06 {
    fn part1(&self, input: &str) -> String {
        let lines: Vec<&str> = input.lines().collect();
        if lines.is_empty() {
            return "0".to_string();
        }

        let num_rows = lines.len();
        let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        let mut grand_total: u64 = 0;
        let mut col = 0;

        while col < max_width {
            // Skip separator columns (all spaces)
            if is_separator_column(&lines, col, num_rows) {
                col += 1;
                continue;
            }

            // Find where this column ends
            let col_end = find_column_end(&lines, col, max_width, num_rows);

            // Parse and calculate this column
            let result = parse_and_calculate_column(&lines, col, col_end, num_rows);
            grand_total += result;

            col = col_end;
        }

        grand_total.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let lines: Vec<&str> = input.lines().collect();
        if lines.is_empty() {
            return "0".to_string();
        }

        let num_rows = lines.len();
        let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        if max_width == 0 {
            return "0".to_string();
        }

        let mut grand_total: u64 = 0;
        let mut col = max_width - 1;

        loop {
            // Skip separator columns
            while is_separator_column(&lines, col, num_rows) {
                if col == 0 {
                    return grand_total.to_string();
                }
                col -= 1;
            }

            // Find problem boundaries (moving left)
            let problem_end = col;
            while col > 0 && !is_separator_column(&lines, col, num_rows) {
                col -= 1;
            }
            let problem_start = if is_separator_column(&lines, col, num_rows) {
                col + 1
            } else {
                col
            };

            // Parse and calculate this problem
            let result =
                parse_and_calculate_problem_rtl(&lines, problem_start, problem_end, num_rows);
            grand_total += result;

            // Move to next problem
            if problem_start == 0 {
                break;
            }
            col = problem_start - 1;
        }

        grand_total.to_string()
    }
}

// Helper functions

/// Check if a column contains only spaces (is a separator)
#[inline]
fn is_separator_column(lines: &[&str], col: usize, num_rows: usize) -> bool {
    for line in lines.iter().take(num_rows) {
        if col < line.len() {
            let ch = line.as_bytes()[col];
            if ch != b' ' {
                return false;
            }
        }
    }
    true
}

/// Find the end position of the current column (next separator or end of line)
fn find_column_end(lines: &[&str], start: usize, max_width: usize, num_rows: usize) -> usize {
    for col in (start + 1)..=max_width {
        if col == max_width || is_separator_column(lines, col, num_rows) {
            return col;
        }
    }
    max_width
}

/// Parse a number from a specific range within a line
#[inline]
fn parse_number_from_range(line: &str, start: usize, end: usize) -> Option<u64> {
    if start >= line.len() {
        return None;
    }

    let end = end.min(line.len());
    let slice = &line[start..end];
    let trimmed = slice.trim();

    if trimmed.is_empty() {
        None
    } else {
        trimmed.parse::<u64>().ok()
    }
}

/// Calculate result based on operation (sum or product)
#[inline]
fn calculate_result(numbers: &[u64], operation: char) -> u64 {
    if numbers.is_empty() {
        return 0;
    }

    match operation {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => 0,
    }
}

/// Parse a column and calculate its result
fn parse_and_calculate_column(lines: &[&str], start: usize, end: usize, num_rows: usize) -> u64 {
    let mut numbers = Vec::new();
    let operation_row = num_rows - 1;
    let mut operation = '+';

    for (row_idx, line) in lines.iter().enumerate() {
        if row_idx == operation_row {
            // Parse operation from the last row
            for col in start..end {
                if col < line.len() {
                    let ch = line.as_bytes()[col];
                    if ch == b'+' || ch == b'*' {
                        operation = ch as char;
                        break;
                    }
                }
            }
        } else {
            // Parse number from this row
            if let Some(num) = parse_number_from_range(line, start, end) {
                numbers.push(num);
            }
        }
    }

    calculate_result(&numbers, operation)
}

/// Extract operation symbol from a problem range
fn extract_operation(lines: &[&str], start: usize, end: usize, num_rows: usize) -> char {
    let operation_row = num_rows - 1;
    for col in start..=end {
        if col < lines[operation_row].len() {
            let ch = lines[operation_row].as_bytes()[col];
            if ch == b'+' || ch == b'*' {
                return ch as char;
            }
        }
    }
    '+' // default
}

/// Parse and calculate a problem reading right-to-left
/// Each column represents one complete number (reading vertically top-to-bottom)
fn parse_and_calculate_problem_rtl(
    lines: &[&str],
    problem_start: usize,
    problem_end: usize,
    num_rows: usize,
) -> u64 {
    let mut all_numbers = Vec::new();

    // Each column in the problem range represents ONE number
    for col in problem_start..=problem_end {
        let mut number_str = String::new();

        // Read this column top-to-bottom (excluding operation row) to form one number
        for line in lines.iter().take(num_rows - 1) {
            if col < line.len() {
                let ch = line.as_bytes()[col];
                if ch.is_ascii_digit() {
                    number_str.push(ch as char);
                }
            }
        }

        // If this column produced a number, add it to our list
        if !number_str.is_empty() {
            if let Ok(num) = number_str.parse::<u64>() {
                all_numbers.push(num);
            }
        }
    }

    // Extract operation
    let operation = extract_operation(lines, problem_start, problem_end, num_rows);

    calculate_result(&all_numbers, operation)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1_example() {
        let day = Day06;
        assert_eq!(day.part1(EXAMPLE), "4277556");
    }

    #[test]
    fn test_single_column_addition() {
        let input = "1\n2\n3\n+";
        let day = Day06;
        assert_eq!(day.part1(input), "6");
    }

    #[test]
    fn test_single_column_multiplication() {
        let input = "2\n3\n4\n*";
        let day = Day06;
        assert_eq!(day.part1(input), "24");
    }

    #[test]
    fn test_empty_input() {
        let day = Day06;
        assert_eq!(day.part1(""), "0");
    }

    #[test]
    fn test_column_parsing() {
        // Test that we correctly parse individual columns from example
        let lines = vec!["123 328", " 45 64 ", "  6 98 ", "*   +  "];

        // Column 0-3: 123 * 45 * 6 = 33210
        let result = parse_and_calculate_column(&lines, 0, 3, 4);
        assert_eq!(result, 33210);

        // Column 4-7: 328 + 64 + 98 = 490
        let result = parse_and_calculate_column(&lines, 4, 7, 4);
        assert_eq!(result, 490);
    }

    #[test]
    fn test_part2_example() {
        let day = Day06;
        assert_eq!(day.part2(EXAMPLE), "3263827");
    }

    #[test]
    fn test_part2_single_column() {
        let input = "1 2\n3 4\n+ *";
        let day = Day06;
        // Column 0: '1','3' -> 13 (with +)
        // Column 2: '2','4' -> 24 (with *)
        // Total: 13 + 24 = 37
        assert_eq!(day.part2(input), "37");
    }

    #[test]
    fn test_part2_single_problem_multi_digit() {
        // Two columns form two 2-digit numbers
        let input = "12\n34\n*";
        let day = Day06;
        // Col 0: '1','3' -> 13
        // Col 1: '2','4' -> 24
        // Result: 13 * 24 = 312
        assert_eq!(day.part2(input), "312");
    }

    #[test]
    fn test_part2_empty_input() {
        let day = Day06;
        assert_eq!(day.part2(""), "0");
    }
}

// Define benchmarks using the common macro
crate::define_day_benches!(Day06);
