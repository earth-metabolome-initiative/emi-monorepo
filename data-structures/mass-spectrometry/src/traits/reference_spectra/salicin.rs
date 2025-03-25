//! Submodule providing data for the salicin molecule.

use crate::traits::SpectrumAlloc;

/// Trait for a spectrum of salicin.
pub trait SalicinSpectrum: SpectrumAlloc {
    /// Create a new spectrum of salicin.
    fn salicin() -> Self;
}

/// The precursor mass over charge value for salicin.
pub const SALICIN_PRECURSOR_MZ: f32 = 321.0750;

/// The mass over charge values for salicin.
pub const SALICIN_MZ: [f32; 21] = [
    52.27001, 55.74109, 57.76688, 60.32213, 64.12009, 82.39278, 87.02966, 91.02013, 92.37563,
    93.13435, 112.07272, 116.92837, 123.04468, 138.61075, 140.95815, 146.96143, 174.95708,
    184.95059, 238.55797, 305.44812, 321.07443,
];
/// The intensities for salicin.
pub const SALICIN_INTENSITIES: [f32; 21] = [
    2309.0, 1977.0, 2003.0, 2102.0, 2177.0, 2127.0, 2376.0, 2380.0, 2703.0, 2200.0, 2232.0, 2923.0,
    2173.0, 2257.0, 2367.0, 4363.0, 31526.0, 5119.0, 2252.0, 2233.0, 22755.0,
];

impl<S: SpectrumAlloc> SalicinSpectrum for S
where
    S::Mz: From<f32>,
    S::Intensity: From<f32>,
{
    fn salicin() -> Self {
        let mut spectrum = Self::with_capacity(SALICIN_PRECURSOR_MZ.into(), SALICIN_MZ.len());
        for (&mz, &intensity) in SALICIN_MZ.iter().zip(SALICIN_INTENSITIES.iter()) {
            spectrum
                .add_peak(mz.into(), intensity.into())
                .expect("Failed to add salicin peak to spectrum");
        }
        spectrum
    }
}
