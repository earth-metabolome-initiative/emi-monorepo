//! Submodule defining a spectral similarity trait.

use functional_properties::similarity::ScalarSimilarity;

use crate::prelude::Spectrum;

/// Trait for calculating the similarity between two [`Spectrum`]s.
pub trait ScalarSpectralSimilarity<Left: Spectrum, Right: Spectrum>:
    ScalarSimilarity<Left, Right>
{
}
