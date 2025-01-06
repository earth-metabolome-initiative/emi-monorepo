//! Submodule providing a report for a download task.

use crate::Task;

/// Report for a download task.
pub struct TaskReport {
	/// The task that the report is for.
	pub task: Task,
}
