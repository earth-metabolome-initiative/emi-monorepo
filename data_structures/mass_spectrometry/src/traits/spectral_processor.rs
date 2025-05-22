//! Submodule defining a trait for a processor operating on [`Spectrum`]
//! objects.

use crate::prelude::Spectrum;

/// Trait for a processor operating on [`Spectrum`] objects.
pub trait SpectralProcessor {
    /// The type of spectrum processed by the processor.
    type Spectrum: Spectrum;

    /// Processes a spectrum.
    fn process(&self, spectrum: &Self::Spectrum) -> Self::Spectrum;
}
