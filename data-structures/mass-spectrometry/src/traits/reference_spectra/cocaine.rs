//! Submodule providing data for the cocaine molecule.

use crate::traits::SpectrumAlloc;

/// Trait for a spectrum of cocaine.
pub trait CocaineSpectrum: SpectrumAlloc {
    /// Create a new spectrum of cocaine.
    fn cocaine() -> Self;
}

/// The precursor mass over charge value for cocaine.
pub const COCAINE_PRECURSOR_MZ: f32 = 304.153137;

/// The mass over charge values for cocaine.
pub const COCAINE_MZ: [f32; 9] = [
    82.064789, 105.033249, 109.213745, 119.04921, 150.0914, 182.117676, 185.804688, 226.579071,
    304.153137,
];
/// The intensities for cocaine.
pub const COCAINE_INTENSITIES: [f32; 9] = [
    13342.493164,
    3264.133545,
    1584.27478,
    2382.930908,
    3257.366211,
    133504.296875,
    1849.140137,
    1391.734497,
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
