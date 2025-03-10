//! Submodule defining a single Spectrum collection trait.

use algebra::prelude::*;

use crate::prelude::Annotation;

/// Trait for a single Spectrum.
pub trait Spectrum {
    /// The type of the Intensity.
    type Intensity: PositiveNumber;
    /// The type of the mass over charge.
    type Mz: PositiveNumber;
    /// Iterator over the intensities in the Spectrum, sorted by mass over charge.
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
}

/// Trait for [`Spectrum`] with annotations.
pub trait AnnotatedSpectrum: Spectrum {
    /// The type of the annotation.
    type Annotation: Annotation;
}
