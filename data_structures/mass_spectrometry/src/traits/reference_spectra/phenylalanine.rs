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
    84.329_65, 95.261_45, 104.255_54, 107.151_16, 108.493_41, 120.354_17, 121.332_74, 122.174_09,
    123.061_45, 123.876_31, 125.26976, 127.950_97, 131.030_27, 131.898_82, 132.987_49, 133.966_95,
    135.217_8, 136.683_53, 138.050_45, 148.049_01, 149.007_42, 149.634_23, 150.754_49,
];
/// The intensities for Phenylalanine.
pub const PHENYLANINE_INTENSITIES: [f32; 23] = [
    802.63324,
    527.747_3,
    232.556_43,
    297.472_17,
    113.401_2,
    8457614.0,
    5_223.299,
    1_784.401_9,
    733.603_8,
    514.92334,
    680.641_6,
    341.421_33,
    44541.625,
    596.891_6,
    781.120_4,
    2_167.02,
    130.054_76,
    264.539_2,
    1_620.136,
    55_389.79,
    364_218.6,
    4_676.809_6,
    590.623_3,
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
