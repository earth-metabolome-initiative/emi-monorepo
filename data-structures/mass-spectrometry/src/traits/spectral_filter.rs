//! Submodule defining a trait for a filter operating on [`Spectrum`] objects.

use crate::prelude::Spectrum;

/// Trait for a filter operating on [`Spectrum`] objects.
pub trait SpectralFilter {
    /// The type of spectrum processed by the filter.
    type Spectrum: Spectrum;

    /// Applies the filter to a spectrum.
    fn filter(&self, spectrum: &Self::Spectrum) -> bool;
}
