#![doc = include_str!("../README.md")]

pub mod compression_extension;
pub mod downloader;
pub mod errors;
pub mod reports;
pub mod task;
pub mod utils;

pub use compression_extension::CompressionExtension;
pub use downloader::Downloader;
pub use errors::{DownloaderConfig, DownloaderError};
pub use reports::{ExtractionReport, TaskReport};
pub use task::Task;
