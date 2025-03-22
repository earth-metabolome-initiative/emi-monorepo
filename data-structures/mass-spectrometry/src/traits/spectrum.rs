//! Submodule defining a single Spectrum collection trait.

use algebra::{impls::ranged::SimpleRanged, prelude::*};

use crate::prelude::Annotation;

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

    /// Returns an iterator over the intensities in the Spectrum, SORTED by mass
    /// over charge.
    fn intensities(&self) -> Self::SortedIntensitiesIter<'_>;

    /// Returns the nth-intensity value.
    ///
    /// # Arguments
    ///
    /// * `n`: The index of the intensity value.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn intensity_nth(&self, n: usize) -> Self::Intensity;

    /// Returns an iterator over the SORTED mass over charge values in the
    /// Spectrum.
    fn mz(&self) -> Self::SortedMzIter<'_>;

    /// Returns an iterator over the SORTED mass over charge values in the
    /// Spectrum, starting from the requested index.
    fn mz_from(&self, index: usize) -> Self::SortedMzIter<'_>;

    /// Returns the nth-mz value.
    ///
    /// # Arguments
    ///
    /// * `n`: The index of the mz value.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn mz_nth(&self, n: usize) -> Self::Mz;

    /// Returns an iterator over the peaks in the Spectrum, SORTED by mass over
    /// charge.
    fn peaks(&self) -> Self::SortedPeaksIter<'_>;

    /// Returns the nth-peak.
    ///
    /// # Arguments
    ///
    /// * `n`: The index of the peak.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn peak_nth(&self, n: usize) -> (Self::Mz, Self::Intensity);

    /// Returns the precursor mass over charge.
    fn precursor_mz(&self) -> Self::Mz;

    /// Returns the matching peaks graph for the provided tolerance.
    ///
    /// # Arguments
    ///
    /// * `other`: The other Spectrum.
    /// * `mz_tolerance`: The mass over charge
    fn matching_peaks<S: Spectrum<Mz = Self::Mz>>(
        &self,
        other: &S,
        mz_tolerance: Self::Mz,
    ) -> RangedCSR2D<u16, u16, SimpleRanged<u16>> {
        let mut matching_peaks = RangedCSR2D::default();
        let mut lowest_other_index = 0;
        for (i, mz) in self.mz().enumerate() {
            let mut lowest_other_index_tmp = lowest_other_index;
            for (j, other_mz) in other
                .mz_from(lowest_other_index)
                .enumerate()
                .map(|(j, mz)| (j + lowest_other_index, mz))
            {
                if other_mz > mz + mz_tolerance {
                    // The mz values are sorted, so we can break here as we have
                    // reached the end of the mz values that are within the mz
                    // tolerance for the current mz value.
                    break;
                }
                if other_mz < mz - mz_tolerance {
                    continue;
                }
                matching_peaks
                    .add((i as u16, j as u16))
                    .expect("The peak matching graph should not contain duplicate edges.");
                lowest_other_index_tmp = j;
            }
            lowest_other_index = lowest_other_index_tmp;
        }

        matching_peaks
    }
}

/// Trait for [`Spectrum`] with annotations.
pub trait AnnotatedSpectrum: Spectrum {
    /// The type of the annotation.
    type Annotation: Annotation;
}
