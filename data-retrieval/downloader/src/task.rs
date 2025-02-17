//! Submodule defining a struct with a download task.

use url::Url;

use crate::{DownloaderConfig, DownloaderError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Download task.
pub struct Task {
    /// The url from which to download the file.
    pub url: Url,
    /// The target path to save the downloaded file.
    pub target_path: String,
}

impl Task {
    #[must_use]
    /// Set the target path to save the downloaded file.
    ///
    /// # Arguments
    ///
    /// * `target_path` - The target path to save the downloaded file.
    ///
    /// # Returns
    ///
    /// * Self, with the target path set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use downloader::Task;
    ///
    /// let task: Task =
    ///     Task::try_from("https://example.com/file.zip").unwrap().target_path(&"file.zip");
    ///
    /// assert_eq!(task.target_path, "file.zip");
    ///
    /// let task: Task = Task::try_from("https://example.com/file.zip").unwrap();
    ///
    /// assert_eq!(task.target_path, "file.zip");
    /// ```
    pub fn target_path<S: ToString>(mut self, target_path: &S) -> Self {
        self.target_path = target_path.to_string();
        self
    }
}

impl TryFrom<Url> for Task {
    type Error = DownloaderError;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        let Some(target_path) = value.path_segments().and_then(Iterator::last).map(String::from)
        else {
            return Err(DownloaderConfig::NoInferrablePath(value).into());
        };

        Ok(Self { url: value, target_path })
    }
}

impl<'a> TryFrom<&'a str> for Task {
    type Error = DownloaderError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Url::parse(value)?.try_into()
    }
}

impl<'a> TryFrom<&&'a str> for Task {
    type Error = DownloaderError;

    fn try_from(value: &&'a str) -> Result<Self, Self::Error> {
        Url::parse(value)?.try_into()
    }
}

impl TryFrom<String> for Task {
    type Error = DownloaderError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}
