//! Advent of Code 2025 CLI
//!
//! Command-line interface for running AoC 2025 solutions.
//!
//! Usage:
//!   aoc2025              # Run all implemented days
//!   aoc2025 --day N      # Run a specific day
//!   aoc2025 --help       # Show help message

use aoc2025::{days, runner, table};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse command line arguments
    let (day_filter, all_impls) = parse_args(&args);

    // Get days to run
    let all_days = if all_impls {
        days::get_all_implementations()
    } else {
        days::get_days()
    };
    let days_to_run: Vec<_> = match day_filter {
        Some(n) => all_days.into_iter().filter(|d| d.number == n).collect(),
        None => all_days,
    };

    // Check if any days are available
    if days_to_run.is_empty() {
        if let Some(n) = day_filter {
            eprintln!("Error: Day {} is not implemented yet.", n);
            process::exit(1);
        }
    }

    // Run the selected days
    let results = runner::run_all(&days_to_run);

    // Display results
    println!("\nAdvent of Code 2025 - Results\n");
    println!("{}", table::format_results(&results));
}

/// Parse command line arguments
///
/// Returns:
/// - A tuple of (day_filter, all_impls)
/// - day_filter: `None` to run all days, `Some(n)` to run day n
/// - all_impls: `true` to run all implementations, `false` for default only
fn parse_args(args: &[String]) -> (Option<usize>, bool) {
    let mut day_filter = None;
    let mut all_impls = false;
    let mut i = 1; // Skip program name

    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            "--day" | "-d" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --day requires a day number");
                    print_usage();
                    process::exit(1);
                }
                match args[i + 1].parse::<usize>() {
                    Ok(n) if (1..=25).contains(&n) => {
                        day_filter = Some(n);
                        i += 2;
                    }
                    Ok(_) => {
                        eprintln!("Error: Day must be between 1 and 25");
                        process::exit(1);
                    }
                    Err(_) => {
                        eprintln!("Error: Invalid day number: {}", args[i + 1]);
                        process::exit(1);
                    }
                }
            }
            "--all-impls" | "-a" => {
                all_impls = true;
                i += 1;
            }
            arg => {
                eprintln!("Error: Unknown argument: {}", arg);
                print_usage();
                process::exit(1);
            }
        }
    }

    (day_filter, all_impls)
}

fn print_usage() {
    eprintln!("Usage: aoc2025 [OPTIONS]");
    eprintln!("Try 'aoc2025 --help' for more information.");
}

fn print_help() {
    println!("Advent of Code 2025 - Solution Runner");
    println!();
    println!("USAGE:");
    println!("    aoc2025 [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -d, --day <N>     Run only day N (1-25)");
    println!("    -a, --all-impls   Run all implementations for each day");
    println!("    -h, --help        Print help information");
    println!();
    println!("EXAMPLES:");
    println!("    aoc2025               Run all implemented days");
    println!("    aoc2025 --day 1       Run only day 1");
    println!("    aoc2025 -d 5          Run only day 5");
    println!("    aoc2025 --all-impls   Run all implementations (e.g., math and string)");
    println!("    aoc2025 -d 2 -a       Run all implementations for day 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_no_filter() {
        let args = vec!["aoc2025".to_string()];
        assert_eq!(parse_args(&args), (None, false));
    }

    #[test]
    fn test_parse_args_with_day() {
        let args = vec!["aoc2025".to_string(), "--day".to_string(), "5".to_string()];
        assert_eq!(parse_args(&args), (Some(5), false));

        let args = vec!["aoc2025".to_string(), "-d".to_string(), "1".to_string()];
        assert_eq!(parse_args(&args), (Some(1), false));
    }

    #[test]
    fn test_parse_args_with_all_impls() {
        let args = vec!["aoc2025".to_string(), "--all-impls".to_string()];
        assert_eq!(parse_args(&args), (None, true));

        let args = vec!["aoc2025".to_string(), "-a".to_string()];
        assert_eq!(parse_args(&args), (None, true));
    }

    #[test]
    fn test_parse_args_with_day_and_all_impls() {
        let args = vec![
            "aoc2025".to_string(),
            "--day".to_string(),
            "2".to_string(),
            "--all-impls".to_string(),
        ];
        assert_eq!(parse_args(&args), (Some(2), true));
    }
}
