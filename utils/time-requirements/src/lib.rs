#![doc = include_str!("../README.md")]

pub mod report;
pub mod task;
pub mod time_tracker;

/// Prelude module to re-export commonly used items.
pub mod prelude {
    pub use crate::{report::Report, task::Task, time_tracker::TimeTracker};
}
