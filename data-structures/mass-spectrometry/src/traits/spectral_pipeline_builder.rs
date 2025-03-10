//! Submodule defining a builder for a [`SpectralPipeline`].

use crate::prelude::{SpectralFilter, SpectralPipeline, Spectrum};

use super::SpectralProcessor;

/// A builder for a [`SpectralPipeline`].
pub trait SpectralPipelineBuilder {
    /// The type of [`SpectralPipeline`] built by the builder.
    type Pipeline: SpectralPipeline;
    /// The type of [`Spectrum`] processed by the pipeline.
    type Spectrum: Spectrum;

    /// Adds a filter to the pipeline.
    fn filter(self, filter: Box<dyn SpectralFilter<Spectrum = Self::Spectrum>>) -> Self;

	/// Adds a processor to the pipeline.
	fn processor(self, processor: Box<dyn SpectralProcessor<Spectrum = Self::Spectrum>>) -> Self;

	/// Builds the pipeline.
	fn build(self) -> Self::Pipeline;
}
