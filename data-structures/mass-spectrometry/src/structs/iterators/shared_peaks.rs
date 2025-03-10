//! Iterator over the shared peaks in two spectra, within a given tolerance.

use algebra::prelude::*;

use crate::prelude::Spectrum;

/// Iterator over the shared peaks in two spectra, within a given tolerance.
pub struct SharedPeaks<LeftSpectrum, RightSpectrum>
where
    LeftSpectrum: Spectrum,
    RightSpectrum: Spectrum<Mz = LeftSpectrum::Mz>,
{
    left: LeftSpectrum,
    right: RightSpectrum,
    tolerance: LeftSpectrum::Mz,
    shift: LeftSpectrum::Mz,
}

#[derive(Clone, Debug, Default)]
/// Builder for the [`SharedPeaks`] iterator.
pub struct SharedPeaksBuilder<LeftSpectrum, RightSpectrum>
where
    LeftSpectrum: Spectrum,
    RightSpectrum: Spectrum<Mz = LeftSpectrum::Mz>,
{
    left: Option<LeftSpectrum>,
    right: Option<RightSpectrum>,
    tolerance: Option<LeftSpectrum::Mz>,
    shift: LeftSpectrum::Mz,
}

impl<LeftSpectrum, RightSpectrum> SharedPeaksBuilder<LeftSpectrum, RightSpectrum>
where
    LeftSpectrum: Spectrum,
    RightSpectrum: Spectrum<Mz = LeftSpectrum::Mz>,
{
    /// Sets the left spectrum.
    pub fn left(mut self, left: LeftSpectrum) -> Self {
        self.left = Some(left);
        self
    }

    /// Sets the right spectrum.
    pub fn right(mut self, right: RightSpectrum) -> Self {
        self.right = Some(right);
        self
    }

    /// Sets the tolerance.
    pub fn tolerance(mut self, tolerance: LeftSpectrum::Mz) -> Self {
        self.tolerance = Some(tolerance);
        self
    }

    /// Sets the shift.
    pub fn shift(mut self, shift: LeftSpectrum::Mz) -> Self {
        self.shift = shift;
        self
    }

    /// Builds the [`SharedPeaks`] iterator.
    pub fn build(self) -> SharedPeaks<LeftSpectrum, RightSpectrum> {
        SharedPeaks {
            left: self.left.expect("Left spectrum not set."),
            right: self.right.expect("Right spectrum not set."),
            tolerance: self.tolerance.expect("Tolerance not set."),
            shift: self.shift,
        }
    }
}
