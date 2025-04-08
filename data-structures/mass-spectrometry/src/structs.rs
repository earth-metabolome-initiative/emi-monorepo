//! Structures for mass spectrometry data.

pub mod generic_spectrum;
pub mod iterators;
pub mod exact_cosine;

pub use generic_spectrum::GenericSpectrum;
pub use iterators::GreedySharedPeaks;
pub use exact_cosine::ExactCosine;