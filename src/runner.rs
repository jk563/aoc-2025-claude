//! Day execution and timing logic

use std::time::{Duration, Instant};

/// Trait that all day solutions must implement
pub trait Day: Send + Sync {
    /// Solve part 1 of the puzzle
    fn part1(&self, input: &str) -> String;

    /// Solve part 2 of the puzzle
    fn part2(&self, input: &str) -> String;
}

/// Result of running a single day's solution
#[derive(Debug, Clone)]
pub struct DayResult {
    pub day_number: usize,
    pub part1_result: String,
    pub part2_result: String,
    pub part1_time: Duration,
    pub part2_time: Duration,
}

impl DayResult {
    /// Total time for both parts
    pub fn total_time(&self) -> Duration {
        self.part1_time + self.part2_time
    }
}

/// Metadata for a day's solution
pub struct DayInfo {
    pub number: usize,
    pub solver: Box<dyn Day>,
    pub input: &'static str,
}

/// Run a single day's solution and measure timing
pub fn run_day(info: &DayInfo) -> DayResult {
    let start = Instant::now();
    let part1_result = info.solver.part1(info.input);
    let part1_time = start.elapsed();

    let start = Instant::now();
    let part2_result = info.solver.part2(info.input);
    let part2_time = start.elapsed();

    DayResult {
        day_number: info.number,
        part1_result,
        part2_result,
        part1_time,
        part2_time,
    }
}

/// Run all registered days and return their results
pub fn run_all(days: &[DayInfo]) -> Vec<DayResult> {
    days.iter().map(run_day).collect()
}

/// Format a duration for display
pub fn format_duration(duration: Duration) -> String {
    let micros = duration.as_micros();

    if micros < 1_000 {
        format!("{} µs", micros)
    } else if micros < 1_000_000 {
        format!("{:.2} ms", micros as f64 / 1_000.0)
    } else {
        format!("{:.2} s", micros as f64 / 1_000_000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDay;

    impl Day for TestDay {
        fn part1(&self, input: &str) -> String {
            input.len().to_string()
        }

        fn part2(&self, input: &str) -> String {
            input.chars().filter(|c| c.is_numeric()).count().to_string()
        }
    }

    #[test]
    fn test_run_day() {
        let info = DayInfo {
            number: 1,
            solver: Box::new(TestDay),
            input: "test123",
        };

        let result = run_day(&info);
        assert_eq!(result.day_number, 1);
        assert_eq!(result.part1_result, "7");
        assert_eq!(result.part2_result, "3");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_micros(500)), "500 µs");
        assert_eq!(format_duration(Duration::from_micros(1500)), "1.50 ms");
        assert_eq!(format_duration(Duration::from_millis(1500)), "1.50 s");
    }
}
