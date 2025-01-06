//! Submodule defining a download task.

use futures_util::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressIterator};
use reqwest::{Client, Response};
use std::io::Write;
use std::{fmt::Debug, fs::File};

use crate::{errors::DownloaderError, Task};

/// Download task.
#[derive(Debug, Clone)]
pub struct Downloader {
    /// Whether to delete the partially downloaded file upon CTRL+C or failure.
    pub delete_on_cancel: bool,
    /// Whether to show a loading bar.
    pub show_loading_bar: bool,
    /// The tasks to download.
    pub tasks: Vec<Task>,
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
    pub async fn execute(self) -> Result<(), DownloaderError> {
        // We build the composite progress bar.
        let composite: MultiProgress = MultiProgress::new();

        // We add a progress bar for the primary task.
        let primary: ProgressBar = composite.add(ProgressBar::new(self.len() as u64));
        primary.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );

        for task in self.tasks.into_iter().progress_with(primary) {
            // We obtain a client.
            let client: Client = Client::new();
            let response: Response = client.get(task.url).send().await?;
            let total_size = response.content_length().unwrap_or(0);
            let internal_loading_bar = composite.add(ProgressBar::new(total_size));
            internal_loading_bar.set_style(
                indicatif::ProgressStyle::default_bar()
                    .template(
                        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
                    )
                    .unwrap()
                    .progress_chars("#>-"),
            );
            // Open the output file for writing
            let mut file = File::create(&task.target_path)?;
            let mut downloaded = 0;

            let mut stream = response.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                file.write_all(&chunk)?;
                downloaded += chunk.len() as u64;
                internal_loading_bar.set_position(downloaded);
            }
        }

        Ok(())
    }
}
