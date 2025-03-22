//! Test suite to verify that the shared peaks are correctly identified.

// use mass_spectrometry::{
//     prelude::GenericSpectrum,
//     traits::{
//         CocaineSpectrum, GlucoseSpectrum, HydroxyCholesterolSpectrum, Spectrum,
//         PhenylalanineSpectrum,
//     },
// };

// #[test]
// /// Test the shared peaks between two spectra.
// pub fn test_greedy_shared_peaks_homogeneous() {
//     for spectrum in [
//         GenericSpectrum::cocaine(),
//         GenericSpectrum::glucose(),
//         GenericSpectrum::hydroxy_cholesterol(),
//         GenericSpectrum::phenylalanine(),
//     ] {
//         let greedy_shared_peaks = spectrum.greedy_shared_peaks(&spectrum, 0.1_f32, 0.0_f32);

//         let greedy_shared_peaks: Vec<_> = greedy_shared_peaks.collect();
//         let cocaine_peaks: Vec<(f32, f32)> = spectrum.peaks().collect::<Vec<_>>();

//         assert_eq!(greedy_shared_peaks.len(), cocaine_peaks.len());
//         for ((left_peak, right_peak), cocaine_peak) in
//             greedy_shared_peaks.iter().zip(cocaine_peaks.iter())
//         {
//             assert_eq!(left_peak, cocaine_peak);
//             assert_eq!(right_peak, cocaine_peak);
//         }
//     }
// }

// #[test]
// /// Test the shared peaks between two spectra.
// pub fn test_greedy_shared_peaks_heterogeneous() {
//     let cocaine: GenericSpectrum<f32, f32> = GenericSpectrum::cocaine();
//     let glucose: GenericSpectrum<f32, f32> = GenericSpectrum::glucose();

//     let greedy_shared_peaks =
//         cocaine.greedy_shared_peaks(&glucose, 1.0_f32, 0.0_f32).collect::<Vec<_>>();

//     assert_eq!(greedy_shared_peaks.len(), 3);
//     assert_eq!(
//         greedy_shared_peaks,
//         vec![
//             ((82.06479, 13342.493), (82.95215, 798.8589)),
//             ((105.03325, 3264.1335), (105.27045, 1257.2534)),
//             ((185.80469, 1849.1401), (185.15268, 9220.535))
//         ]
//     );
// }
