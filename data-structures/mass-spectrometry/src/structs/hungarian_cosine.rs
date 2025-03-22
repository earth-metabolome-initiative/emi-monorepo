//! Implementation of the cosine distance for mass spectra.

use ::graph::prelude::{Assignment, HungarianAlgorithm};
use algebra::{
    impls::{ranged::SimpleRanged, GenericImplicitValuedMatrix2D, RangedCSR2D},
    prelude::{Number, Pow, Sqrt, Zero},
};
use functional_properties::prelude::ScalarSimilarity;
use graph::traits::Edges;

use crate::traits::{ScalarSpectralSimilarity, Spectrum};

/// Implementation of the cosine distance for mass spectra.
pub struct HungarianCosine<EXP, MZ> {
    /// The power to which the mass/charge ratio is raised.
    mz_power: EXP,
    /// The power to which the intensity is raised.
    intensity_power: EXP,
    /// The tolerance for the mass-shift of the mass/charge ratio.
    mz_tolerance: MZ,
}

impl<EXP, MZ> HungarianCosine<EXP, MZ> {
    /// Creates a new instance of the Hungarian cosine distance.
    /// 
    /// # Arguments
    /// 
    /// * `mz_power`: The power to which the mass/charge ratio is raised.
    /// * `intensity_power`: The power to which the intensity is raised.
    /// * `mz_tolerance`: The tolerance for the mass-shift of the mass/charge ratio.
    /// 
    /// # Returns
    /// 
    /// A new instance of the Hungarian cosine distance.
    pub fn new(mz_power: EXP, intensity_power: EXP, mz_tolerance: MZ) -> Self {
        Self {
            mz_power,
            intensity_power,
            mz_tolerance,
        }
    }
}

impl<EXP, S1, S2> ScalarSimilarity<S1, S2> for HungarianCosine<EXP, S1::Mz>
where
    EXP: Number,
    S1::Mz: Pow<EXP> + Sqrt + Number,
    S1: Spectrum<Intensity = <S1 as Spectrum>::Mz>,
    S2: Spectrum<Intensity = S1::Mz, Mz = S1::Mz>,
{
    type Similarity = S1::Mz;

    fn similarity(&self, left: &S1, right: &S2) -> Self::Similarity {
        let mut left_peak_products = Vec::with_capacity(left.len());
        let mut left_peak_squared_sums: S1::Mz = S1::Mz::ZERO;
        let mut right_peak_products = Vec::with_capacity(right.len());
        let mut right_peak_squared_sums: S1::Mz = S1::Mz::ZERO;

        for (mz, intensity) in left.peaks() {
            let score = mz.pow(self.mz_power) * intensity.pow(self.intensity_power);
            left_peak_products.push(score);
            left_peak_squared_sums += score * score;
        }
        for (mz, intensity) in right.peaks() {
            let score = mz.pow(self.mz_power) * intensity.pow(self.intensity_power);
            right_peak_products.push(score);
            right_peak_squared_sums += score * score;
        }

        let left_peak_norm: S1::Mz = left_peak_squared_sums.sqrt();
        let right_peak_norm: S1::Mz = right_peak_squared_sums.sqrt();

        let map: GenericImplicitValuedMatrix2D<
            RangedCSR2D<u16, u16, SimpleRanged<u16>>,
            _,
            S1::Mz,
        > = GenericImplicitValuedMatrix2D::new(
            left.matching_peaks(right, self.mz_tolerance),
            |(i, j)| left_peak_products[i as usize] * right_peak_products[j as usize],
        );

        println!("Initial graph {}", map.number_of_edges());

        let Ok(matching): Result<Vec<(u16, u16, S1::Mz)>, _> = map.hungarian() else {
            return S1::Mz::ZERO;
        };

        println!("Matching {:?}", matching);

        matching.cost() / (left_peak_norm * right_peak_norm)
    }
}

impl<S1, S2, EXP> ScalarSpectralSimilarity<S1, S2> for HungarianCosine<EXP, S1::Mz>
where
    EXP: Number,
    S1::Mz: Pow<EXP> + Sqrt,
    S1: Spectrum<Intensity = <S1 as Spectrum>::Mz>,
    S2: Spectrum<Intensity = S1::Mz, Mz = S1::Mz>,
{
}
