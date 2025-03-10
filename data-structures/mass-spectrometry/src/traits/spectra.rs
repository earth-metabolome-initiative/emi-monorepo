//! Submodule defining a Spectra collection trait.

use super::spectrum::Spectrum;

/// Trait for a collection of Spectra.
pub trait Spectra {
    /// The type of the Spectrum.
    type Spectrum: Spectrum;
	/// The type of the Spectrum iterator.
	type SpectraIter<'a>: Iterator<Item = Self::Spectrum> where Self: 'a;

	/// Returns an iterator over the Spectra.
	fn spectra(&self) -> Self::SpectraIter<'_>;

	/// Returns the number of Spectra.
	fn len(&self) -> usize;

	/// Returns true if the Spectra is empty.
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}
