//! Submodule defining a spectral similarity trait.

use functional_properties::similarity::ScalarSimilarity;

use crate::prelude::Spectrum;

/// Trait for calculating the similarity between two [`Spectrum`]s.
pub trait ScalarSpectralSimilarity<OtherSpectrum: Spectrum>:
    ScalarSimilarity<OtherSpectrum>
{
}
