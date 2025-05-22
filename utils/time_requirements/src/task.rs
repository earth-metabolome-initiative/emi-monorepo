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
    pub fn new<S: ToString + ?Sized>(name: &S) -> Self {
        Self { name: name.to_string(), start: chrono::Local::now().naive_local() }
    }

    #[must_use]
    /// Marks the task as completed.
    pub fn complete(self) -> CompletedTask {
        CompletedTask {
            name: self.name,
            start: self.start,
            end: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash)]
/// A completed task.
pub struct CompletedTask {
    /// The name of the task.
    pub(crate) name: String,
    /// The start time of the task.
    pub(crate) start: chrono::NaiveDateTime,
    /// The end time of the task.
    pub(crate) end: chrono::NaiveDateTime,
}

impl CompletedTask {
    #[must_use]
    /// Returns the name of the task.
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
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
