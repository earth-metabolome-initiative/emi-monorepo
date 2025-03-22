//! Submodule to test execution of the Hungarian algorithm with cosine
//! similarity.

use mass_spectrometry::{
    prelude::{
        CocaineSpectrum, GenericSpectrum, GlucoseSpectrum, HungarianCosine,
        HydroxyCholesterolSpectrum, PhenylalanineSpectrum, ScalarSimilarity,
    },
    traits::Spectrum,
};

#[test]
/// Test the Hungarian algorithm with cosine similarity.
pub fn test_hungarian_cosine() {
    let left = GenericSpectrum::cocaine();
    let right = GenericSpectrum::glucose();
    let cosine = HungarianCosine::new(1.0, 1.0, 0.5);
    let similarity = cosine.similarity(&left, &right);
    assert_eq!(similarity, 0.0);
	let self_left_similarity = cosine.similarity(&left, &left);
	assert_eq!(self_left_similarity, 1.0);
	let self_right_similarity = cosine.similarity(&right, &right);
	assert_eq!(self_right_similarity, 1.0);
}
