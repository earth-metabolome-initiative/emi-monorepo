#![doc = include_str!("../README.md")]

pub mod task;
pub mod time_tracker;
pub mod report;

/// Prelude module to re-export commonly used items.
pub mod prelude {
    pub use crate::task::Task;
    pub use crate::time_tracker::TimeTracker;
    pub use crate::report::Report;
}