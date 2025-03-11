//! Submodule providing data for the glucose molecule.

use crate::traits::SpectrumAlloc;

/// Trait for a spectrum of glucose.
pub trait GlucoseSpectrum: SpectrumAlloc {
    /// Create a new spectrum of glucose.
    fn glucose() -> Self;
}

/// The precursor mass over charge value for glucose.
pub const GLUCOSE_PRECURSOR_MZ: f32 = 203.05;

/// The mass over charge values for glucose.
pub const GLUCOSE_MZ: [f32; 14] = [
    82.952148, 105.270447, 112.789398, 121.208, 129.116699, 131.104095, 131.990692, 135.007935,
    142.50119, 143.102905, 158.092255, 160.235291, 173.100464, 185.152679,
];
/// The intensities for glucose.
pub const GLUCOSE_INTENSITIES: [f32; 14] = [
    798.858887,
    1257.253418,
    3923.249023,
    1952.965454,
    169.587341,
    412.055176,
    309.939514,
    520.569153,
    555.742432,
    13786.814453,
    1758.816406,
    408.889771,
    892.346924,
    9220.535156,
];

impl<S: SpectrumAlloc> GlucoseSpectrum for S
where
    S::Mz: From<f32>,
    S::Intensity: From<f32>,
{
    fn glucose() -> Self {
        let mut spectrum = Self::with_capacity(GLUCOSE_PRECURSOR_MZ.into(), GLUCOSE_MZ.len());
        for (&mz, &intensity) in GLUCOSE_MZ.iter().zip(GLUCOSE_INTENSITIES.iter()) {
            spectrum
                .add_peak(mz.into(), intensity.into())
                .expect("Failed to add glucose peak to spectrum");
        }
        spectrum
    }
}
