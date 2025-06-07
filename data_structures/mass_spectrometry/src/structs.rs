//! Structures for mass spectrometry data.

pub mod exact_cosine;
pub mod generic_spectrum;
pub mod iterators;

pub use exact_cosine::ExactCosine;
pub use generic_spectrum::GenericSpectrum;
pub use iterators::GreedySharedPeaks;
