//! Submodule to test execution of the exact algorithm with cosine
//! similarity.

use mass_spectrometry::prelude::{
    CocaineSpectrum, ExactCosine, GenericSpectrum, GlucoseSpectrum, ScalarSimilarity, Spectrum,
};

#[test]
/// Test the exact algorithm with cosine similarity.
pub fn test_exact_cosine() {
    let cocaine = GenericSpectrum::cocaine();
    let glucose = GenericSpectrum::glucose();
    let cosine = ExactCosine::new(1.0, 1.0, 0.1);
    let (similarity, number_of_shared_peaks) = cosine.similarity(&cocaine, &glucose);
    assert!(similarity <= 0.01, "Similarity: {}", similarity);
    assert_eq!(number_of_shared_peaks, 0);

    let (self_cocaine_similarity, number_of_shared_peaks) = cosine.similarity(&cocaine, &cocaine);
    assert_eq!(self_cocaine_similarity, 1.0);
    assert_eq!(number_of_shared_peaks, cocaine.len() as u16);
    let (self_glucose_similarity, number_of_shared_peaks) = cosine.similarity(&glucose, &glucose);
    assert_eq!(self_glucose_similarity, 1.0);
    assert_eq!(number_of_shared_peaks, glucose.len() as u16);
}
