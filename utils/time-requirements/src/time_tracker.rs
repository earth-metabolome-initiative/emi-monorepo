//! Submodule defining the task tracker.

use crate::task::{CompletedTask, Task};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// A tracker for tasks.
pub struct TimeTracker {
    /// Name of the overall project.
    name: String,
    /// The tasks being tracked.
    tasks: Vec<CompletedTask>,
    /// The sub-trackers being tracked.
    sub_trackers: Vec<TimeTracker>,
    /// Start of the project.
    start: chrono::NaiveDateTime,
}

impl TimeTracker {
    /// Creates a new time tracker for the given project name.
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
            tasks: Vec::new(),
            sub_trackers: Vec::new(),
            start: chrono::Local::now().naive_local(),
        }
    }

    /// Returns the sub-trackers.
    pub(crate) fn sub_trackers(&self) -> &[TimeTracker] {
        &self.sub_trackers
    }

    /// Returns a reference to the requested sub-tracker, if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the sub-tracker to retrieve.
    pub(crate) fn sub_tracker_by_name(&self, name: &str) -> Option<&TimeTracker> {
        self.sub_trackers.iter().find(|tracker| tracker.name() == name)
    }

    /// Converts the tracker into a completed task.
    pub fn to_completed_task(&self) -> CompletedTask {
        CompletedTask {
            name: self.name.clone(),
            start: self.start,
            end: self.start + self.total_time(),
        }
    }

    /// Extends the tracker from another tracker.
    pub fn extend(&mut self, other: TimeTracker) {
        self.tasks.push(other.to_completed_task());
        self.sub_trackers.push(other);
    }

    /// Adds a task to the tracker.
    pub fn add_completed_task(&mut self, task: Task) {
        self.tasks.push(task.complete());
    }

    /// Returns the name of the project.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the start time of the project.
    pub fn start(&self) -> chrono::NaiveDateTime {
        self.start
    }

    /// Iterates the task from the tracker.
    pub fn tasks(&self) -> impl Iterator<Item = &CompletedTask> {
        self.tasks.iter()
    }

    /// Returns a reference to the slowest task.
    pub fn slowest_task(&self) -> Option<&CompletedTask> {
        self.tasks.iter().max()
    }

    /// Returns the total amount of time spent on all tasks.
    pub fn total_time(&self) -> chrono::TimeDelta {
        self.tasks.iter().map(|task| task.time()).sum()
    }

    /// Saves the report as a JSON in the provided directory.
    pub fn save(&self, directory: &std::path::Path) -> std::io::Result<()> {
        let file = std::fs::File::create(directory.join(format!("{}.json", self.name)))?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }
}
