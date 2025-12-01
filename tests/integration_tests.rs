//! Integration tests for the AoC 2025 project
//!
//! These tests verify that the overall system works correctly,
//! including day registration, execution, and result formatting.

use aoc2025::{days, runner};

#[test]
fn test_runner_infrastructure() {
    // Test that we can get days (even if empty)
    let all_days = days::get_days();
    // Just verify we can call this without panicking
    let _ = all_days.len();
}

#[test]
fn test_runner_with_empty_days() {
    // Test that running with no days doesn't panic
    let days = days::get_days();
    let results = runner::run_all(&days);
    assert_eq!(results.len(), days.len());
}

#[test]
fn test_get_specific_day_nonexistent() {
    // Test that getting a non-existent day returns None
    let day = days::get_day(99);
    assert!(day.is_none());
}
