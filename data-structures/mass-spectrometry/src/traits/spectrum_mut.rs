//! Submodule providing a trait for a mutable Spectrum.

use super::Spectrum;

/// Trait for a mutable Spectrum.
pub trait SpectrumMut: Spectrum {
    /// The type of error that can occur when mutating the Spectrum.
    type MutationError: core::error::Error;

    /// Add a peak to the Spectrum.
    fn add_peak(
        &mut self,
        mz: Self::Mz,
        intensity: Self::Intensity,
    ) -> Result<(), Self::MutationError>;
}

/// Trait for an allocable Spectrum.
pub trait SpectrumAlloc: SpectrumMut {
    /// Create a new Spectrum with a given capacity.
    ///
    /// # Arguments
    ///
    /// * `precursor_mz`: The precursor mass over charge.
    /// * `capacity`: The capacity of the Spectrum.
    fn with_capacity(precursor_mz: Self::Mz, capacity: usize) -> Self;
}
