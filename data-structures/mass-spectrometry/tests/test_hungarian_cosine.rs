//! Submodule to test execution of the Hungarian algorithm with cosine
//! similarity.

use algebra::prelude::SparseMatrix2D;
use mass_spectrometry::prelude::{
    AspirinSpectrum, CocaineSpectrum, GenericSpectrum, GlucoseSpectrum, HungarianCosine,
    HydroxyCholesterolSpectrum, PhenylalanineSpectrum, SalicinSpectrum, ScalarSimilarity, Spectrum,
};

#[test]
/// Test the Hungarian algorithm with cosine similarity.
pub fn test_hungarian_cosine() {
    let cocaine = GenericSpectrum::cocaine();
    let glucose = GenericSpectrum::glucose();
    let cosine = HungarianCosine::new(1.0, 1.0, 0.1);
    let (similarity, number_of_shared_peaks) = cosine.similarity(&cocaine, &glucose);
    assert!(similarity <= 0.01, "Similarity: {}", similarity);
    assert_eq!(number_of_shared_peaks, 0);

    let cosine2 = HungarianCosine::new(0.0, 1.0, 5.0);
    let (similarity, number_of_shared_peaks) = cosine2.similarity(&cocaine, &glucose);
    assert!(
        similarity > 0.453 && similarity < 0.454,
        "Similarity: {similarity}, number of peaks: {number_of_shared_peaks}"
    );
    assert_eq!(number_of_shared_peaks, 5);

    let (self_cocaine_similarity, number_of_shared_peaks) = cosine.similarity(&cocaine, &cocaine);
    assert_eq!(self_cocaine_similarity, 1.0);
    assert_eq!(number_of_shared_peaks, cocaine.len() as u16);
    let (self_glucose_similarity, number_of_shared_peaks) = cosine.similarity(&glucose, &glucose);
    assert_eq!(self_glucose_similarity, 1.0);
    assert_eq!(number_of_shared_peaks, glucose.len() as u16);

    let phenilanine = GenericSpectrum::phenylalanine();
    let hydroxy_cholesterol = GenericSpectrum::hydroxy_cholesterol();

    let (similarity, number_of_shared_peaks) =
        cosine.similarity(&phenilanine, &hydroxy_cholesterol);
    assert!(similarity <= 0.01);
    assert_eq!(number_of_shared_peaks, 3);

    let (similarity, number_of_shared_peaks) =
        cosine2.similarity(&phenilanine, &hydroxy_cholesterol);
    assert!(similarity <= 0.01);
    assert_eq!(
        number_of_shared_peaks,
        23,
        "Incorrect number of shared peaks from spectra with sizes {} and {}. Starts with {} non-empty rows.",
        phenilanine.len(),
        hydroxy_cholesterol.len(),
        phenilanine
            .matching_peaks(&hydroxy_cholesterol, cosine2.mz_tolerance())
            .number_of_non_empty_rows()
    );

    let aspirin = GenericSpectrum::aspirin();
    let salicin = GenericSpectrum::salicin();

    let (similarity, number_of_shared_peaks) = cosine.similarity(&aspirin, &salicin);
    assert!(similarity <= 0.01);
    assert_eq!(number_of_shared_peaks, 0);
}
