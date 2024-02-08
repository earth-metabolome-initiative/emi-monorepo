use sirius::prelude::*;
use std::path::Path;

#[test]
#[should_panic]
fn test_failing_sirius() {
    let sirius = SiriusBuilder::default()
        .maximal_mz(802.2)
        .unwrap()
        .formula_search_db(SearchDB::Bio)
        .unwrap()
        .isotope_settings_filter(false)
        .unwrap()
        .structure_search_db(SearchDB::Coconut)
        .unwrap()
        .timeout_seconds_per_tree_default()
        .unwrap()
        .number_of_candidates_per_ion(5)
        .unwrap()
        .adduct_settings_detectable_default()
        .unwrap()
        .timeout_seconds_per_instance_default()
        .unwrap()
        .formula_result_threshold_default()
        .unwrap()
        .inject_el_gordo_compounds_default()
        .unwrap()
        .median_noise_intensity(0.013)
        .unwrap()
        .ms1_absolute_intensity_error(-0.5) // should panic here !
        .unwrap()
        .ms1_minimal_intensity_to_consider(0.4)
        .unwrap()
        .ms1_relative_intensity_error_default()
        .unwrap()
        .noise_threshold_settings_absolute_threshold_default()
        .unwrap()
        .noise_threshold_settings_intensity_threshold(0.2)
        .unwrap()
        .noise_threshold_settings_maximal_number_of_peaks(42)
        .unwrap()
        .number_of_structure_candidates(2)
        .unwrap()
        .enable_formula()
        .unwrap()
        .enable_zodiac()
        .unwrap()
        .enable_fingerprint()
        .unwrap()
        .enable_structure()
        .unwrap()
        .enable_canopus()
        .unwrap()
        .build();

    let input_file_path = Path::new("tests/data/input_sirius.mgf");
    let output_file_path = Path::new("tests/data/output_sirius");

    // Check if the path exists before attempting to remove it
    if output_file_path.exists() {
        let _ = std::fs::remove_dir_all(output_file_path);
    }

    sirius.run(input_file_path, output_file_path).unwrap();
}

#[test]
fn test_run_sirius_default() -> Result<(), String> {
    let sirius = SiriusBuilder::<Version5>::default()
        .maximal_mz_default()?
        .enable_formula()?
        .enable_zodiac()?
        .enable_fingerprint()?
        .enable_structure()?
        .enable_canopus()?
        .enable_write_summaries()?
        .build();
    let input_file_path = Path::new("tests/data/input_sirius.mgf");
    let output_file_path = Path::new("tests/data/output_sirius_default");

    // Check if the path exists before attempting to remove it
    if output_file_path.exists() {
        let _ = std::fs::remove_dir_all(output_file_path);
    }
    sirius.run(input_file_path, output_file_path).unwrap();

    Ok(())
}

#[test]
fn test_run_sirius_with_enpkg_params() -> Result<(), String> {
    let sirius = SiriusBuilder::default()
        .maximal_mz(800.0)?
        .isotope_settings_filter(true)?
        .formula_search_db(SearchDB::Bio)?
        .timeout_seconds_per_tree(0)?
        .formula_settings_enforced(AtomVector::from(vec![
            Atoms::H,
            Atoms::C,
            Atoms::N,
            Atoms::O,
            Atoms::P,
        ]))?
        .timeout_seconds_per_instance(0)?
        .adduct_settings_detectable(AdductsVector::from(vec![
            Adducts::MplusHplus,
            Adducts::MplusHminusTwoH2Oplus,
            Adducts::MplusNaplus,
            Adducts::MplusKplus,
            Adducts::MplusH3NplusHplus,
            Adducts::MplusHminusH2Oplus,
        ]))?
        .use_heuristic_mz_to_use_heuristic_only(650)?
        .algorithm_profile(Instruments::Orbitrap)?
        .isotope_ms2_settings(IsotopeMS2Settings::Ignore)?
        .ms2_mass_deviation_allowed_mass_deviation(MassDeviation::Ppm(5.0))?
        .number_of_candidates_per_ion(1)?
        .use_heuristic_mz_to_use_heuristic(300)?
        .formula_settings_detectable(AtomVector::from(vec![
            Atoms::B,
            Atoms::Cl,
            Atoms::Se,
            Atoms::S,
        ]))?
        .number_of_candidates(10)?
        .zodiac_number_of_considered_candidates_at_300_mz(10)?
        .zodiac_run_in_two_steps(true)?
        .zodiac_edge_filter_thresholds_min_local_connections(10)?
        .zodiac_edge_filter_thresholds_threshold_filter(0.95)?
        .zodiac_epochs_burn_in_period(2000)?
        .zodiac_epochs_number_of_markov_chains(10)?
        .zodiac_number_of_considered_candidates_at_800_mz(50)?
        .zodiac_epochs_iterations(20000)?
        .adduct_settings_enforced_default()?
        .adduct_settings_fallback(AdductsVector::from(vec![
            Adducts::MplusHplus,
            Adducts::MplusNaplus,
            Adducts::MplusKplus,
        ]))?
        .formula_result_threshold(true)?
        .inject_el_gordo_compounds(true)?
        .structure_search_db(SearchDB::Bio)?
        .recompute_results(false)?
        .enable_formula()?
        .enable_zodiac()?
        .enable_fingerprint()?
        .enable_structure()?
        .enable_canopus()?
        .enable_write_summaries()?
        .build();

    let input_file_path = Path::new("tests/data/input_sirius.mgf");
    let output_file_path = Path::new("tests/data/output_sirius");

    // Check if the path exists before attempting to remove it
    if output_file_path.exists() {
        let _ = std::fs::remove_dir_all(output_file_path);
    }
    // Start running sirius
    sirius.run(input_file_path, output_file_path).unwrap();

    Ok(())
}
