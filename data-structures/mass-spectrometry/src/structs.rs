//! Structures for mass spectrometry data.

pub mod generic_spectrum;
pub mod iterators;
pub mod hungarian_cosine;

pub use generic_spectrum::GenericSpectrum;
pub use iterators::GreedySharedPeaks;
pub use hungarian_cosine::HungarianCosine;