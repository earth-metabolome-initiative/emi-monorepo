//! Iterator over the shared peaks in two spectra, within a given tolerance.

use core::iter::Peekable;

use common_traits::prelude::{Builder, BuilderError};
use num_traits::ConstZero;

use crate::prelude::Spectrum;

#[derive(Clone, Copy, Debug)]
/// Attribute for the [`GreedySharedPeaks`] iterator.
pub enum GreedySharedPeaksAttribute {
    /// The left spectrum.
    Left,
    /// The right spectrum.
    Right,
    /// The tolerance.
    Tolerance,
    /// The right spectra shift.
    RightShift,
}

impl core::fmt::Display for GreedySharedPeaksAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
            Self::Tolerance => write!(f, "tolerance"),
            Self::RightShift => write!(f, "right_shift"),
        }
    }
}

/// Iterator over the shared peaks in two spectra, within a given tolerance.
pub struct GreedySharedPeaks<'a, LeftSpectrum, RightSpectrum>
where
    LeftSpectrum: Spectrum + 'a,
    RightSpectrum: Spectrum<Mz = LeftSpectrum::Mz> + 'a,
{
    left: Peekable<LeftSpectrum::SortedPeaksIter<'a>>,
    right: Peekable<RightSpectrum::SortedPeaksIter<'a>>,
    tolerance: LeftSpectrum::Mz,
    right_shift: LeftSpectrum::Mz,
}

impl<LeftSpectrum, RightSpectrum> Iterator for GreedySharedPeaks<'_, LeftSpectrum, RightSpectrum>
where
    LeftSpectrum: Spectrum,
    RightSpectrum: Spectrum<Mz = LeftSpectrum::Mz>,
{
    type Item = (
        (LeftSpectrum::Mz, LeftSpectrum::Intensity),
        (RightSpectrum::Mz, RightSpectrum::Intensity),
    );

    fn next(&mut self) -> Option<Self::Item> {
        match (self.left.peek(), self.right.peek()) {
            (Some((left_mz, _)), Some((right_mz, _))) => {
                let shifted_right_mz = *right_mz + self.right_shift;
                if shifted_right_mz < *left_mz + self.tolerance
                    && *left_mz < shifted_right_mz + self.tolerance
                {
                    let left = self.left.next().unwrap();
                    let right = self.right.next().unwrap();
                    Some((left, right))
                } else if left_mz < right_mz {
                    self.left.next();
                    self.next()
                } else {
                    self.right.next();
                    self.next()
                }
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
/// Builder for the [`GreedySharedPeaks`] iterator.
pub struct GreedySharedPeaksBuilder<'spectra, LeftSpectrum, RightSpectrum>
where
    LeftSpectrum: Spectrum + 'spectra,
    RightSpectrum: Spectrum<Mz = LeftSpectrum::Mz> + 'spectra,
{
    left: Option<LeftSpectrum::SortedPeaksIter<'spectra>>,
    right: Option<RightSpectrum::SortedPeaksIter<'spectra>>,
    tolerance: Option<LeftSpectrum::Mz>,
    right_shift: LeftSpectrum::Mz,
}

impl<LeftSpectrum, RightSpectrum> Default
    for GreedySharedPeaksBuilder<'_, LeftSpectrum, RightSpectrum>
where
    LeftSpectrum: Spectrum,
    RightSpectrum: Spectrum<Mz = LeftSpectrum::Mz>,
{
    fn default() -> Self {
        Self { left: None, right: None, tolerance: None, right_shift: LeftSpectrum::Mz::ZERO }
    }
}

impl<'spectra, LeftSpectrum, RightSpectrum>
    GreedySharedPeaksBuilder<'spectra, LeftSpectrum, RightSpectrum>
where
    LeftSpectrum: Spectrum,
    RightSpectrum: Spectrum<Mz = LeftSpectrum::Mz>,
{
    /// Sets the left spectrum.
    pub fn left(mut self, left: &'spectra LeftSpectrum) -> Self {
        self.left = Some(left.peaks());
        self
    }

    /// Sets the right spectrum.
    pub fn right(mut self, right: &'spectra RightSpectrum) -> Self {
        self.right = Some(right.peaks());
        self
    }

    /// Sets the tolerance.
    pub fn tolerance(mut self, tolerance: LeftSpectrum::Mz) -> Self {
        self.tolerance = Some(tolerance);
        self
    }

    /// Sets the shift for the right spectrum.
    pub fn right_shift(mut self, right_shift: LeftSpectrum::Mz) -> Self {
        self.right_shift = right_shift;
        self
    }
}

impl<'spectra, LeftSpectrum, RightSpectrum> Builder
    for GreedySharedPeaksBuilder<'spectra, LeftSpectrum, RightSpectrum>
where
    LeftSpectrum: Spectrum,
    RightSpectrum: Spectrum<Mz = LeftSpectrum::Mz>,
{
    type Object = GreedySharedPeaks<'spectra, LeftSpectrum, RightSpectrum>;
    type Error = BuilderError<Self::Attribute>;
    type Attribute = GreedySharedPeaksAttribute;

    fn is_complete(&self) -> bool {
        self.left.is_some() && self.right.is_some() && self.tolerance.is_some()
    }

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            left: self
                .left
                .ok_or(BuilderError::IncompleteBuild(GreedySharedPeaksAttribute::Left))?
                .peekable(),
            right: self
                .right
                .ok_or(BuilderError::IncompleteBuild(GreedySharedPeaksAttribute::Right))?
                .peekable(),
            tolerance: self
                .tolerance
                .ok_or(BuilderError::IncompleteBuild(GreedySharedPeaksAttribute::Tolerance))?,
            right_shift: self.right_shift,
        })
    }
}
