//! A pipeline to process a [`Spectra`] library.

use super::{SpectralFilter, SpectralProcessor, Spectrum};

/// A pipeline to process a [`Spectra`] library.
pub trait SpectralPipeline {
    /// The type of [`Spectrum`] processed by the pipeline.
    type Spectrum: Spectrum;
    /// The type of the [`SpectralFilter`] iterator.
    type Filters<'a>: Iterator<Item = &'a dyn SpectralFilter<Spectrum = Self::Spectrum>>
    where
        Self: 'a;
    /// The type of the [`SpectralProcessor`] iterator.
    type Processors<'a>: Iterator<Item = &'a dyn SpectralProcessor<Spectrum = Self::Spectrum>>
    where
        Self: 'a;

    /// Returns an iterator over the [`SpectralFilter`] objects in the pipeline.
    fn filters(&self) -> Self::Filters<'_>;

    /// Returns an iterator over the [`SpectralProcessor`] objects in the
    /// pipeline.
    fn processors(&self) -> Self::Processors<'_>;

    /// Processes a spectra library.
    fn process<I>(&self, spectra: I) -> impl Iterator<Item = Self::Spectrum>
    where
        I: IntoIterator<Item = Self::Spectrum>,
    {
        spectra
            .into_iter()
            .filter(|spectrum| self.filters().all(|filter| filter.filter(spectrum)))
            .map(|spectrum| {
                self.processors().fold(spectrum, |spectrum, processor| processor.process(&spectrum))
            })
    }
}
