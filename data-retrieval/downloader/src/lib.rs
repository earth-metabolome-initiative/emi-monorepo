#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(unused_macro_rules)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![deny(unused_import_braces)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

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
