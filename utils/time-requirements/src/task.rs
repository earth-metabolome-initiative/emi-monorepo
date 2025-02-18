//! Submodule defining a task to be tracked.

#[derive(Debug)]
/// A task to be tracked.
pub struct Task {
    /// The name of the task.
    name: String,
    /// The start time of the task.
    start: chrono::NaiveDateTime,
}

impl Task {
    /// Create a new task with the given name.
    pub fn new<S: ToString>(name: S) -> Self {
        Self { name: name.to_string(), start: chrono::Local::now().naive_local() }
    }

    /// Marks the task as completed.
    pub fn complete(self) -> CompletedTask {
        CompletedTask { name: self.name, start: self.start, end: chrono::Local::now().naive_local() }
    }
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash)]
/// A completed task.
pub struct CompletedTask {
    /// The name of the task.
    name: String,
    /// The start time of the task.
    start: chrono::NaiveDateTime,
    /// The end time of the task.
    end: chrono::NaiveDateTime,
}

impl CompletedTask {
    /// Returns the name of the task.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the time required to complete the task.
    pub fn time(&self) -> chrono::TimeDelta {
        self.end - self.start
    }
}

impl Ord for CompletedTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time().cmp(&other.time())
    }
}

impl PartialOrd for CompletedTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
