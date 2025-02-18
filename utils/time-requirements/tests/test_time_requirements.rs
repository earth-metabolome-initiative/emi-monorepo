//! Submodule defining the task tracker.
use time_requirements::prelude::*;

/// Generate a time tracker for testing.
pub fn time_tracker() -> TimeTracker {
    let mut tracker = TimeTracker::new("Test Project");

    let task1 = Task::new("Test Task 1");
    // We sleep for 1 second to simulate the task taking time.
    std::thread::sleep(std::time::Duration::from_secs(1));
    tracker.add_task(task1);

    // We add a second task
    let task2 = Task::new("Test Task 2");
    // We sleep for 2 seconds to simulate the task taking time.
    std::thread::sleep(std::time::Duration::from_secs(2));
    tracker.add_task(task2);

    tracker
}

#[test]
/// Test the time requirements.
pub fn test_time_requirements() {
    let tracker = time_tracker();

    assert_eq!(tracker.name(), "Test Project");
    assert_eq!(tracker.tasks().count(), 2);

    let task1 = tracker.tasks().nth(0).unwrap();
    assert_eq!(task1.name(), "Test Task 1");
    assert_eq!(task1.time().num_seconds(), 1);

    let task2 = tracker.tasks().nth(1).unwrap();
    assert_eq!(task2.name(), "Test Task 2");
    assert_eq!(task2.time().num_seconds(), 2);

    assert_eq!(tracker.total_time().num_seconds(), 3);
    assert_eq!(tracker.slowest_task().unwrap().name(), "Test Task 2");
    assert_eq!(tracker.slowest_task().unwrap().time().num_seconds(), 2);
}

#[test]
/// Test the time report generation.
pub fn test_time_report() {
    use std::path::Path;

    let tracker1 = time_tracker();
    let tracker2 = time_tracker();
    let tracker3 = time_tracker();

    let mut report = Report::new(tracker3);
    report.add_previous_report(tracker1);
    report.add_previous_report(tracker2);

    report.write(Path::new("time_report.md"), Path::new("plot.png")).unwrap();
}
