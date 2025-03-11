//! Structures for mass spectrometry data.

pub mod generic_spectrum;
pub mod iterators;

pub use generic_spectrum::GenericSpectrum;
pub use iterators::GreedySharedPeaks;
