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
    82.952_15, 105.270_45, 112.789_4, 121.208, 129.116_7, 131.104_1, 131.990_69, 135.007_93,
    142.50119, 143.102_9, 158.092_25, 160.235_29, 173.100_46, 185.152_68,
];
/// The intensities for glucose.
pub const GLUCOSE_INTENSITIES: [f32; 14] = [
    798.858_9,
    1_257.253_4,
    3_923.249,
    1_952.965_5,
    169.587_34,
    412.055_18,
    309.939_5,
    520.569_15,
    555.742_43,
    13_786.814,
    1_758.816_4,
    408.889_77,
    892.346_9,
    9_220.535,
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
