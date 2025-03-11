//! Submodule providing data for the phenylalanine molecule.

use crate::traits::SpectrumAlloc;

/// Trait for a spectrum of phenylalanine.
pub trait PhenylalanineSpectrum: SpectrumAlloc {
    /// Create a new spectrum of phenylalanine.
    fn phenylalanine() -> Self;
}

/// The precursor mass over charge value for Phenylalanine.
pub const PHENYLANINE_PRECURSOR_MZ: f32 = 166.086;

/// The mass over charge values for Phenylalanine.
pub const PHENYLANINE_MZ: [f32; 23] = [
    84.329651, 95.261452, 104.255539, 107.151161, 108.493408, 120.354172, 121.332741, 122.174088,
    123.061447, 123.876312, 125.26976, 127.950974, 131.030273, 131.898819, 132.987488, 133.966949,
    135.217804, 136.683533, 138.050446, 148.049011, 149.007416, 149.634232, 150.754486,
];
/// The intensities for Phenylalanine.
pub const PHENYLANINE_INTENSITIES: [f32; 23] = [
    802.63324,
    527.747314,
    232.556427,
    297.472168,
    113.401199,
    8457614.0,
    5223.298828,
    1784.401855,
    733.603821,
    514.92334,
    680.641602,
    341.421326,
    44541.625,
    596.891602,
    781.120422,
    2167.02002,
    130.054764,
    264.539215,
    1620.135986,
    55389.789062,
    364218.59375,
    4676.80957,
    590.623291,
];

impl<S: SpectrumAlloc> PhenylalanineSpectrum for S
where
    S::Mz: From<f32>,
    S::Intensity: From<f32>,
{
    fn phenylalanine() -> Self {
        let mut spectrum =
            Self::with_capacity(PHENYLANINE_PRECURSOR_MZ.into(), PHENYLANINE_MZ.len());
        for (&mz, &intensity) in PHENYLANINE_MZ.iter().zip(PHENYLANINE_INTENSITIES.iter()) {
            spectrum
                .add_peak(mz.into(), intensity.into())
                .expect("Failed to add Phenylalanine peak to spectrum");
        }
        spectrum
    }
}
