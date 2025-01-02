//! Submodule defining a download task.

use std::fmt::Debug;

use crate::{
    errors::{DownloaderConfig, DownloaderError},
    Task,
};

/// Download task.
#[derive(Debug, Clone)]
pub struct Downloader {
    /// Whether to delete the partially downloaded file upon CTRL+C or failure.
    pub delete_on_cancel: bool,
    /// Whether to show a loading bar.
    pub show_loading_bar: bool,
    /// The tasks to download.
    pub tasks: Vec<Task>,
    /// The maximal number of workers to use.
    pub max_workers: usize,
    /// Whether to cache the downloaded files, i.e. not download them again if they already exist.
    pub cache: bool,
    /// Whether to automatically extract the documents
    pub extract: bool,
}

impl Default for Downloader {
    fn default() -> Self {
        Self {
            delete_on_cancel: false,
            tasks: Vec::new(),
            max_workers: 1,
            show_loading_bar: false,
            cache: false,
            extract: false,
        }
    }
}

impl Downloader {
    /// Set whether to delete the partially downloaded file upon CTRL+C or failure.
    ///
    /// # Returns
    ///
    /// * Self, with the delete_on_cancel flag set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use downloader::Downloader;
    ///
    /// let task: Downloader = Downloader::default().delete_on_cancel();
    ///
    /// assert_eq!(task.delete_on_cancel, true);
    ///
    /// let task: Downloader = Downloader::default();
    ///
    /// assert_eq!(task.delete_on_cancel, false);
    /// ```
    ///
    pub fn delete_on_cancel(mut self) -> Self {
        self.delete_on_cancel = true;
        self
    }

    /// Set whether to cache the downloaded files.
    ///
    /// # Returns
    ///
    /// * Self, with the cache flag set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use downloader::Downloader;
    ///
    /// let task: Downloader = Downloader::default().cache();
    ///
    /// assert_eq!(task.cache, true);
    ///
    /// let task: Downloader = Downloader::default();
    ///
    /// assert_eq!(task.cache, false);
    ///
    /// ```
    pub fn cache(mut self) -> Self {
        self.cache = true;
        self
    }

    /// Set whether to automatically extract the downloaded files.
    ///
    /// # Returns
    ///
    /// * Self, with the extract flag set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use downloader::Downloader;
    ///
    /// let task: Downloader = Downloader::default().extract();
    ///
    /// assert_eq!(task.extract, true);
    ///
    /// let task: Downloader = Downloader::default();
    ///
    /// assert_eq!(task.extract, false);
    /// ```
    pub fn extract(mut self) -> Self {
        self.extract = true;
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
    /// * [`DownloaderConfig::ZeroWorkers`] if `max_workers` is 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use downloader::{Downloader, DownloaderError};
    ///
    /// let task: Downloader = Downloader::default().max_workers(4).unwrap();
    ///
    /// assert_eq!(task.max_workers, 4);
    ///
    /// let task: Result<Downloader, DownloaderError> = Downloader::default().max_workers(0);
    ///
    /// assert!(task.is_err());
    /// ```
    pub fn max_workers(mut self, max_workers: usize) -> Result<Self, DownloaderError> {
        if max_workers == 0 {
            return Err(DownloaderConfig::ZeroWorkers.into());
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
    /// use downloader::{Downloader, DownloaderError, DownloaderConfig};
    ///
    /// let mut task: Downloader = Downloader::default().task("https://example.com").unwrap();
    ///
    /// assert_eq!(task.tasks[0].url.as_str(), "https://example.com/");
    ///
    /// task = task.task("https://example.org").unwrap();
    ///
    /// assert_eq!(task.tasks.len(), 2);
    /// assert_eq!(task.tasks[0].url.as_str(), "https://example.com/");
    ///
    /// let error = task.task("invalid url").unwrap_err();
    ///
    /// assert!(matches!(error, DownloaderError::DownloaderConfig(DownloaderConfig::InvalidUrl(_))));
    /// ```
    ///
    pub fn task<T: TryInto<Task, Error = DownloaderError>>(
        mut self,
        task: T,
    ) -> Result<Self, DownloaderError> {
        let url = <T as TryInto<Task>>::try_into(task)?;
        if !self.tasks.contains(&url) {
            self.tasks.push(url);
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
    /// use downloader::{Downloader, DownloaderError};
    ///
    /// let mut task: Downloader = Downloader::default().tasks(&["https://example.com", "https://example.org"]).unwrap();
    ///
    /// assert_eq!(task.tasks.len(), 2);
    /// assert_eq!(task.tasks[0].url.as_str(), "https://example.com/");
    /// assert_eq!(task.tasks[1].url.as_str(), "https://example.org/");
    /// ```
    pub fn tasks<I, T>(mut self, tasks: I) -> Result<Self, DownloaderError>
    where
        I: IntoIterator<Item = T>,
        T: TryInto<Task, Error = DownloaderError>,
    {
        for task in tasks {
            self = self.task(task)?;
        }
        Ok(self)
    }

    /// Execute the download task.
    pub fn execute(&self) -> Result<(), DownloaderError> {
        Ok(())
    }
}
