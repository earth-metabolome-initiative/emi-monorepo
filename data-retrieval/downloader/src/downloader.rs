//! Submodule defining a download task.

use std::{fmt::Debug, fs::File, io::Write, path::Path};

use futures_util::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressIterator};
use reqwest::{Client, Response};

use crate::{
    CompressionExtension, Task, errors::DownloaderError, reports::TaskReport, utils::set_bar_style,
};

/// Download task.
#[derive(Debug, Clone, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Downloader {
    /// Whether to delete the partially downloaded file upon CTRL+C or failure.
    pub delete_on_cancel: bool,
    /// Whether to show a loading bar.
    pub show_loading_bar: bool,
    /// The tasks to download.
    pub tasks: Vec<Task>,
    /// Whether to cache the downloaded files, i.e. not download them again if
    /// they already exist.
    pub cache: bool,
    /// Whether to automatically extract the documents
    pub extract: bool,
    /// Whether to delete the compressed file after extraction.
    pub delete_compressed: bool,
}

impl Downloader {
    #[must_use]
    /// Set whether to delete the partially downloaded file upon CTRL+C or
    /// failure.
    ///
    /// # Returns
    ///
    /// * Self, with the `delete_on_cancel` flag set.
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
    pub fn delete_on_cancel(mut self) -> Self {
        self.delete_on_cancel = true;
        self
    }

    #[must_use]
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
    /// ```
    pub fn cache(mut self) -> Self {
        self.cache = true;
        self
    }

    #[must_use]
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

    #[must_use]
    /// Set whether to delete the compressed file after extraction.
    ///
    /// # Returns
    ///
    /// * Self, with the `delete_compressed` flag set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use downloader::Downloader;
    ///
    /// let task: Downloader = Downloader::default().delete_compressed();
    ///
    /// assert_eq!(task.delete_compressed, true);
    ///
    /// let task: Downloader = Downloader::default();
    ///
    /// assert_eq!(task.delete_compressed, false);
    /// ```
    pub fn delete_compressed(mut self) -> Self {
        self.delete_compressed = true;
        self
    }

    #[must_use]
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
    /// use downloader::{Downloader, DownloaderConfig, DownloaderError};
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
    /// # Errors
    ///
    /// * If the URL is invalid.
    pub fn task<T>(mut self, task: T) -> Result<Self, DownloaderError>
    where
        T: TryInto<Task>,
        DownloaderError: From<<T as TryInto<Task>>::Error>,
    {
        let url = <T as TryInto<Task>>::try_into(task)?;
        if !self.tasks.contains(&url) {
            self.tasks.push(url);
        }
        Ok(self)
    }

    #[must_use]
    /// Returns the number of tasks to download.
    ///
    /// # Returns
    ///
    /// * The number of tasks to download.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use downloader::Downloader;
    ///
    /// let mut task: Downloader = Downloader::default().task("https://example.com").unwrap();
    ///
    /// assert_eq!(task.len(), 1);
    ///
    /// task = task.task("https://example.org").unwrap();
    ///
    /// assert_eq!(task.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    #[must_use]
    /// Returns whether there are no tasks to download.
    ///
    /// # Returns
    ///
    /// * Whether there are no tasks to download.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use downloader::Downloader;
    ///
    /// let mut task: Downloader = Downloader::default();
    ///
    /// assert!(task.is_empty());
    ///
    /// task = task.task("https://example.com").unwrap();
    ///
    /// assert!(!task.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
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
    /// let mut task: Downloader =
    ///     Downloader::default().tasks(&["https://example.com", "https://example.org"]).unwrap();
    ///
    /// assert_eq!(task.tasks.len(), 2);
    /// assert_eq!(task.tasks[0].url.as_str(), "https://example.com/");
    /// assert_eq!(task.tasks[1].url.as_str(), "https://example.org/");
    /// ```
    ///
    /// # Errors
    ///
    /// * If any of the URLs are invalid.
    pub fn tasks<I, T>(mut self, tasks: I) -> Result<Self, DownloaderError>
    where
        I: IntoIterator<Item = T>,
        DownloaderError: From<<T as TryInto<Task>>::Error>,
        T: TryInto<Task>,
    {
        for task in tasks {
            self = self.task(task)?;
        }
        Ok(self)
    }

    /// Execute the download task.
    ///
    /// # Errors
    ///
    /// * If the download fails.
    ///
    /// # Returns
    ///
    /// * A vector of reports for each task.
    pub async fn execute(self) -> Result<Vec<TaskReport>, DownloaderError> {
        // We build the composite progress bar.
        let composite: MultiProgress = MultiProgress::new();

        // We add a progress bar for the primary task.
        let primary: ProgressBar =
            set_bar_style(composite.add(if self.show_loading_bar && self.len() > 1 {
                ProgressBar::new(self.tasks.len() as u64)
            } else {
                ProgressBar::hidden()
            }));

        let mut reports = Vec::with_capacity(self.len());
        // We obtain a client.
        let client: Client = Client::new();

        for task in self.tasks.into_iter().progress_with(primary) {
            if self.cache && Path::new(&task.target_path).exists() {
                reports.push(TaskReport { task, time: 0.0, cached: true, extraction_report: None });
                continue;
            }
            let task_start = std::time::Instant::now();
            let response: Response = client.get(task.url.clone()).send().await?;
            let total_size = response.content_length().unwrap_or(0);
            let internal_loading_bar = set_bar_style(composite.add(if self.show_loading_bar {
                ProgressBar::new(total_size)
            } else {
                ProgressBar::hidden()
            }));
            // Open the output file for writing
            let path = Path::new(&task.target_path);
            let mut file = File::create(path)?;
            let mut downloaded = 0;

            let mut stream = response.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                file.write_all(&chunk)?;
                downloaded += chunk.len() as u64;
                internal_loading_bar.set_position(downloaded);
            }

            // We finish the progress bar.
            internal_loading_bar.finish();

            let download_finish = task_start.elapsed().as_secs_f64();

            // We check if the file should be extracted.
            let mut extraction_report = None;

            if self.extract {
                extraction_report =
                    Some(CompressionExtension::extract(path, &composite, self.show_loading_bar)?);
                if self.delete_compressed {
                    std::fs::remove_file(path)?;
                }
            }

            reports.push(TaskReport {
                task,
                time: download_finish,
                cached: false,
                extraction_report,
            });
        }

        Ok(reports)
    }
}
