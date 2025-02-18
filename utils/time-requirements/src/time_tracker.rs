//! Submodule defining the task tracker.

use crate::task::{CompletedTask, Task};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
/// A tracker for tasks.
pub struct TimeTracker {
    /// Name of the overall project.
    name: String,
    /// The tasks being tracked.
    tasks: Vec<CompletedTask>,
    /// Start of the project.
    start: chrono::NaiveDateTime,
}

impl TimeTracker {
    /// Creates a new time tracker for the given project name.
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
            tasks: Vec::new(),
            start: chrono::Local::now().naive_local(),
        }
    }

    /// Adds a task to the tracker.
    pub fn add_task(&mut self, task: Task) {
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
