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

    /// Returns the name of the task.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
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

    /// Extends the completed task by another completed task.
    pub fn extend(&mut self, other: &CompletedTask) {
        self.end += other.time();
    }

    /// Returns the most precise percentage over the provided `TimeDelta`.
    ///
    /// # Arguments
    ///
    /// * `total_time` - The total time to calculate the percentage over.
    ///
    /// # Implementation Note
    ///
    /// This methods attempts to use the most precise method available to
    /// calculate the percentage. It first tries to use nanoseconds, then
    /// microseconds, then milliseconds, and finally seconds, depending on
    /// whether the conversion is lossless.
    #[must_use]
    pub fn precise_percentage_over(&self, total_time: chrono::TimeDelta) -> f64 {
        if let Some(nanos) = self.time().num_nanoseconds() {
            if let Some(total_nanos) = total_time.num_nanoseconds() {
                return nanos as f64 / total_nanos as f64 * 100.0;
            }
        }
        if let Some(micros) = self.time().num_microseconds() {
            if let Some(total_micros) = total_time.num_microseconds() {
                return micros as f64 / total_micros as f64 * 100.0;
            }
        }
        self.time().num_milliseconds() as f64 / total_time.num_milliseconds() as f64 * 100.0
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
