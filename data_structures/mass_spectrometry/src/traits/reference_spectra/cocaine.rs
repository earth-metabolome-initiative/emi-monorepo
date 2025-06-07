//! Submodule providing data for the cocaine molecule.

use crate::traits::SpectrumAlloc;

/// Trait for a spectrum of cocaine.
pub trait CocaineSpectrum: SpectrumAlloc {
    /// Create a new spectrum of cocaine.
    fn cocaine() -> Self;
}

/// The precursor mass over charge value for cocaine.
pub const COCAINE_PRECURSOR_MZ: f32 = 304.153_14;

/// The mass over charge values for cocaine.
pub const COCAINE_MZ: [f32; 9] = [
    82.064_79, 105.033_25, 109.213745, 119.04921, 150.0914, 182.117_68, 185.804_69, 226.579_07,
    304.153_14,
];
/// The intensities for cocaine.
pub const COCAINE_INTENSITIES: [f32; 9] = [
    13_342.493,
    3_264.133_5,
    1_584.274_8,
    2_382.931,
    3_257.366_2,
    133_504.3,
    1_849.140_1,
    1_391.734_5,
    86052.375,
];

impl<S: SpectrumAlloc> CocaineSpectrum for S
where
    S::Mz: From<f32>,
    S::Intensity: From<f32>,
{
    fn cocaine() -> Self {
        let mut spectrum = Self::with_capacity(COCAINE_PRECURSOR_MZ.into(), COCAINE_MZ.len());
        for (&mz, &intensity) in COCAINE_MZ.iter().zip(COCAINE_INTENSITIES.iter()) {
            spectrum
                .add_peak(mz.into(), intensity.into())
                .expect("Failed to add cocaine peak to spectrum");
        }
        spectrum
    }
}
