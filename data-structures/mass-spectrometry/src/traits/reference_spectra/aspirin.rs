//! Submodule providing data for the aspirin molecule.

use crate::traits::SpectrumAlloc;

/// Trait for a spectrum of aspirin.
pub trait AspirinSpectrum: SpectrumAlloc {
    /// Create a new spectrum of aspirin.
    fn aspirin() -> Self;
}

/// The precursor mass over charge value for aspirin.
pub const ASPIRIN_PRECURSOR_MZ: f32 = 181.0490;

/// The mass over charge values for aspirin.
pub const ASPIRIN_MZ: [f32; 29] = [
    50.0149, 51.0228, 53.0385, 55.0177, 65.0384, 77.0383, 79.054, 80.0254, 81.0334, 91.0541,
    92.0254, 93.0333, 94.0411, 95.049, 98.0361, 105.0333, 105.0445, 107.0489, 111.0439, 120.0203,
    121.0282, 121.0394, 133.0282, 135.0438, 138.0308, 149.0231, 163.0386, 167.0337, 181.0491,
];
/// The intensities for aspirin.
pub const ASPIRIN_INTENSITIES: [f32; 29] = [
    49377.4, 53422.1, 454244.5, 57881.0, 1997532.0, 825848.2, 1153465.5, 96202.8, 58626.8, 44573.8,
    394779.8, 1129287.8, 56357.2, 1654496.5, 72487.3, 707899.4, 1119356.4, 207437.4, 587441.8,
    166384.0, 9695889.0, 2506571.2, 5824675.0, 6124332.0, 78621.8, 34285450.0, 18191732.0, 69049.3,
    120675.9,
];

impl<S: SpectrumAlloc> AspirinSpectrum for S
where
    S::Mz: From<f32>,
    S::Intensity: From<f32>,
{
    fn aspirin() -> Self {
        let mut spectrum = Self::with_capacity(ASPIRIN_PRECURSOR_MZ.into(), ASPIRIN_MZ.len());
        for (&mz, &intensity) in ASPIRIN_MZ.iter().zip(ASPIRIN_INTENSITIES.iter()) {
            spectrum
                .add_peak(mz.into(), intensity.into())
                .expect("Failed to add ASPIRIN peak to spectrum");
        }
        spectrum
    }
}
