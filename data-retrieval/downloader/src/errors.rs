//! Submodule defining the possible errors encountered during a download task.
use reqwest::Error as ReqwestError;
use std::fmt::Debug;
use url::{ParseError, Url};
use std::io::Error as IoError;

#[derive(Debug)]
/// Possible errors encountered during a download task.
pub enum DownloaderError {
    /// Subset of errors relative to a download task configuration.
    DownloaderConfig(DownloaderConfig),
    /// When there is an error with the request.
    RequestError(ReqwestError),
    /// When there is an error with the request.
    IoError(IoError),
}

impl From<DownloaderConfig> for DownloaderError {
    fn from(config: DownloaderConfig) -> Self {
        Self::DownloaderConfig(config)
    }
}

#[derive(Debug)]
/// Possible enums relative to the configuration of a download task.
pub enum DownloaderConfig {
    /// When the provided number of workers is zero.
    ZeroWorkers,
    /// When the provided URL is not a valid URL.
    InvalidUrl(ParseError),
    /// When it is not possible to determine a path from the provided URL.
    NoInferrablePath(Url),
    /// When the provided target path is not a valid writable path.
    InvalidTargetPath(String),
}

impl From<ParseError> for DownloaderConfig {
    fn from(error: ParseError) -> Self {
        Self::InvalidUrl(error)
    }
}

impl From<ParseError> for DownloaderError {
    fn from(error: ParseError) -> Self {
        DownloaderError::DownloaderConfig(DownloaderConfig::InvalidUrl(error))
    }
}

impl From<ReqwestError> for DownloaderError {
    fn from(error: ReqwestError) -> Self {
        Self::RequestError(error)
    }
}

impl From<IoError> for DownloaderError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}