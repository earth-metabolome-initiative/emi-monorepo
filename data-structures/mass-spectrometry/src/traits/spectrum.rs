//! Submodule defining a single Spectrum collection trait.

use algebra::prelude::*;
use common_traits::builder::Builder;

use crate::{
    prelude::Annotation,
    structs::{GreedySharedPeaks, iterators::shared_peaks::GreedySharedPeaksBuilder},
};

/// Trait for a single Spectrum.
pub trait Spectrum {
    /// The type of the Intensity.
    type Intensity: Number;
    /// The type of the mass over charge.
    type Mz: Number;
    /// Iterator over the intensities in the Spectrum, sorted by mass over
    /// charge.
    type SortedIntensitiesIter<'a>: Iterator<Item = Self::Intensity>
    where
        Self: 'a;
    /// Iterator over the sorted mass over charge values in the Spectrum.
    type SortedMzIter<'a>: Iterator<Item = Self::Mz>
    where
        Self: 'a;
    /// Iterator over the peaks in the Spectrum, sorted by mass over charge
    type SortedPeaksIter<'a>: Iterator<Item = (Self::Mz, Self::Intensity)>
    where
        Self: 'a;

    /// Returns the number of peaks in the Spectrum.
    fn len(&self) -> usize;

    /// Returns an iterator over the intensities in the Spectrum.
    fn intensities(&self) -> Self::SortedIntensitiesIter<'_>;

    /// Returns an iterator over the mass over charge values in the Spectrum.
    fn mz(&self) -> Self::SortedMzIter<'_>;

    /// Returns an iterator over the peaks in the Spectrum.
    fn peaks(&self) -> Self::SortedPeaksIter<'_>;

    /// Returns the precursor mass over charge.
    fn precursor_mz(&self) -> Self::Mz;

    /// Returns an iterator over the shared peaks between two Spectra, within a
    /// given tolerance and right-shift.
    fn greedy_shared_peaks<'spectra, Other>(
        &'spectra self,
        other: &'spectra Other,
        tolerance: Self::Mz,
        right_shift: Self::Mz,
    ) -> GreedySharedPeaks<'spectra, Self, Other>
    where
        Self: Sized,
        Other: Sized + Spectrum<Mz = Self::Mz>,
    {
        GreedySharedPeaksBuilder::default()
            .left(self)
            .right(other)
            .tolerance(tolerance)
            .right_shift(right_shift)
            .build()
            .unwrap()
    }
}

/// Trait for [`Spectrum`] with annotations.
pub trait AnnotatedSpectrum: Spectrum {
    /// The type of the annotation.
    type Annotation: Annotation;
}
