//! Submodule providing a report for a download task.

use crate::{CompressionExtension, Task};

#[derive(Clone)]
/// Report for a download task.
pub struct TaskReport {
	/// The task that the report is for.
	pub task: Task,
	/// The time required for the download.
	pub time: f64,
	/// Whether the task was cached.
	pub cached: bool,
	/// Informations regarding the extraction of the file, if any.
	pub extraction_report : Option<ExtractionReport>,
}

#[derive(Clone)]
/// Report on the extraction of a compressed file.
pub struct ExtractionReport {
	/// The path to the compressed file.
	pub source: String,
	/// The path to the extracted file.
	pub destination: String,
	/// The compression extension of the file.
	pub extension: CompressionExtension,
	/// The time required for the extraction.
	pub time: f64,
}

