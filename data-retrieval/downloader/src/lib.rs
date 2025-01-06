#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(unused_macro_rules)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![deny(unused_import_braces)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub mod errors;
pub mod downloader;
pub mod compression_extension;
pub mod task;
pub mod report;

pub use errors::{DownloaderError, DownloaderConfig};
pub use downloader::Downloader;
pub use compression_extension::CompressionExtension;
pub use task::Task;
