//! Submodule defining the possible errors encountered during a download task.
use std::fmt::Debug;
use url::ParseError;

#[derive(Debug)]
/// Possible errors encountered during a download task.
pub enum TaskError {
    /// Subset of errors relative to a download task configuration.
    TaskConfig(TaskConfig),
}

impl From<TaskConfig> for TaskError {
    fn from(config: TaskConfig) -> Self {
        Self::TaskConfig(config)
    }
}

#[derive(Debug)]
/// Possible enums relative to the configuration of a download task.
pub enum TaskConfig {
    /// When the provided number of workers is zero.
    ZeroWorkers,
    /// When the provided URL is not a valid URL.
    InvalidUrl(ParseError),
}

impl From<ParseError> for TaskConfig {
    fn from(error: ParseError) -> Self {
        Self::InvalidUrl(error)
    }
}
