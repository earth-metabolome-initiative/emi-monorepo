//! A naively implemented generic spectrum struct.

use algebra::prelude::Number;
use sorted_vec::prelude::SortedVec;

use crate::traits::{Spectrum, SpectrumAlloc, SpectrumMut};

/// A generic spectrum struct.
pub struct GenericSpectrum<Mz, Intensity> {
    mz: SortedVec<Mz>,
    intensity: Vec<Intensity>,
    precursor_mz: Mz,
}

impl<Mz, Intensity> Spectrum for GenericSpectrum<Mz, Intensity>
where
    Mz: Number,
    Intensity: Number,
{
    type Intensity = Intensity;
    type Mz = Mz;
    type SortedIntensitiesIter<'a>
        = core::iter::Copied<core::slice::Iter<'a, Intensity>>
    where
        Self: 'a;
    type SortedMzIter<'a>
        = core::iter::Copied<core::slice::Iter<'a, Mz>>
    where
        Self: 'a;

    type SortedPeaksIter<'a>
        = core::iter::Zip<Self::SortedMzIter<'a>, Self::SortedIntensitiesIter<'a>>
    where
        Self: 'a;

    fn len(&self) -> usize {
        self.mz.len()
    }

    fn intensities(&self) -> Self::SortedIntensitiesIter<'_> {
        self.intensity.iter().copied()
    }

    fn mz(&self) -> Self::SortedMzIter<'_> {
        self.mz.iter().copied()
    }

    fn peaks(&self) -> Self::SortedPeaksIter<'_> {
        self.mz().zip(self.intensities())
    }

    fn precursor_mz(&self) -> Self::Mz {
        self.precursor_mz
    }
}

impl<Mz, Intensity> SpectrumMut for GenericSpectrum<Mz, Intensity>
where
    Mz: Number,
    Intensity: Number,
{
    type MutationError = sorted_vec::error::Error<Mz>;

    fn add_peak(
        &mut self,
        mz: Self::Mz,
        intensity: Self::Intensity,
    ) -> Result<(), Self::MutationError> {
        self.mz.push(mz)?;
        self.intensity.push(intensity);
        Ok(())
    }
}

impl<Mz, Intensity> SpectrumAlloc for GenericSpectrum<Mz, Intensity>
where
    Mz: Number,
    Intensity: Number,
{
    fn with_capacity(precursor_mz: Self::Mz, capacity: usize) -> Self {
        Self {
            mz: SortedVec::with_capacity(capacity),
            intensity: Vec::with_capacity(capacity),
            precursor_mz,
        }
    }
}
