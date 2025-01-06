//! Submodule providing utilities used through the crate.

use indicatif::ProgressBar;

#[must_use]
#[allow(clippy::missing_panics_doc)]
/// Set the style of the progress bar.
/// 
/// # Arguments
/// 
/// * `bar` - The progress bar to style.
/// 
pub(crate) fn set_bar_style(bar: ProgressBar) -> ProgressBar {
	bar.set_style(
		indicatif::ProgressStyle::default_bar()
			.template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})").unwrap()
			.progress_chars("#>-"),
	);
	bar
}