//! Submodule defining a download task.

use std::fmt::Debug;
use url::Url;

use crate::errors::{TaskConfig, TaskError};

/// Download task.
#[derive(Debug, Clone)]
pub struct Task {
    /// Whether to delete the partially downloaded file upon CTRL+C or failure.
    pub delete_on_cancel: bool,
    /// Whether to show a loading bar.
    pub show_loading_bar: bool,
    /// The URLs to download.
    pub urls: Vec<Url>,
    /// The maximal number of workers to use.
    pub max_workers: usize,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            delete_on_cancel: true,
            urls: Vec::new(),
            max_workers: 1,
            show_loading_bar: true,
        }
    }
}

impl Task {
    /// Set whether to delete the partially downloaded file upon CTRL+C or failure.
    ///
    /// # Arguments
    ///
    /// * `delete_on_cancel`: Whether to delete the partially downloaded file upon CTRL+C or failure.
    ///
    /// # Returns
    ///
    /// * Self, with the delete_on_cancel flag set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use downloader::Task;
    ///
    /// let task: Task = Task::default().delete_on_cancel(false);
    ///
    /// assert_eq!(task.delete_on_cancel, false);
    ///
    /// let task: Task = Task::default().delete_on_cancel(true);
    ///
    /// assert_eq!(task.delete_on_cancel, true);
    /// ```
    ///
    pub fn delete_on_cancel(mut self, delete_on_cancel: bool) -> Self {
        self.delete_on_cancel = delete_on_cancel;
        self
    }

    /// Set the number of workers to use.
    ///
    /// # Arguments
    ///
    /// * `max_workers`: The maximal number of workers to use.
    ///
    /// # Returns
    ///
    /// * Self, with the maximal number of workers set.
    ///
    /// # Errors
    ///
    /// * [`TaskConfig::ZeroWorkers`] if `max_workers` is 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use downloader::{Task, TaskError};
    ///
    /// let task: Task = Task::default().max_workers(4).unwrap();
    ///
    /// assert_eq!(task.max_workers, 4);
    ///
    /// let task: Result<Task, TaskError> = Task::default().max_workers(0);
    ///
    /// assert!(task.is_err());
    /// ```
    pub fn max_workers(mut self, max_workers: usize) -> Result<Self, TaskError> {
        if max_workers == 0 {
            return Err(TaskConfig::ZeroWorkers.into());
        }
        self.max_workers = max_workers;
        Ok(self)
    }

    /// Whether to show a loading bar.
    pub fn show_loading_bar(mut self) -> Self {
        self.show_loading_bar = true;
        self
    }

    /// Add a URL to the list of URLs to download.
    ///
    /// # Arguments
    ///
    /// * `url`: URL to add to the list of URLs to download.
    ///
    /// # Returns
    ///
    /// * Self, with the URL added to the list of URLs to download.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use downloader::{Task, TaskError, TaskConfig};
    ///
    /// let mut task: Task = Task::default().url("https://example.com").unwrap();
    ///
    /// assert_eq!(task.urls[0].as_str(), "https://example.com/");
    ///
    /// task = task.url("https://example.org").unwrap();
    ///
    /// assert_eq!(task.urls.len(), 2);
    /// assert_eq!(task.urls[0].as_str(), "https://example.com/");
    ///
    /// let error = task.url("invalid url").unwrap_err();
    ///
    /// assert!(matches!(error, TaskError::TaskConfig(TaskConfig::InvalidUrl(_))));
    /// ```
    ///
    pub fn url<S: AsRef<str>>(mut self, url: S) -> Result<Self, TaskError> {
        let url = Url::parse(url.as_ref()).map_err(TaskConfig::InvalidUrl)?;
        if !self.urls.contains(&url) {
            self.urls.push(url);
        }
        Ok(self)
    }

    /// Add multiple URLs to the list of URLs to download.
    ///
    /// # Arguments
    ///
    /// * `urls`: URLs to add to the list of URLs to download.
    ///
    /// # Returns
    ///
    /// * Self, with the URLs added to the list of URLs to download.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use downloader::{Task, TaskError};
    ///
    /// let mut task: Task = Task::default().urls(&["https://example.com", "https://example.org"]).unwrap();
    ///
    /// assert_eq!(task.urls.len(), 2);
    /// assert_eq!(task.urls[0].as_str(), "https://example.com/");
    /// assert_eq!(task.urls[1].as_str(), "https://example.org/");
    /// ```
    pub fn urls<I, S>(mut self, urls: I) -> Result<Self, TaskError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for url in urls {
            self = self.url(url)?;
        }
        Ok(self)
    }
}
