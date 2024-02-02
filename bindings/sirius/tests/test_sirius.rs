use sirius::prelude::*;
use std::fs;
use std::path::Path;

#[test]
#[should_panic]
fn test_failing_sirius() {
    let sirius = SiriusBuilder::default()
        .maximal_mz(802.2)
        .unwrap()
        .formula_search_db(FormulaSearchDB::Bio)
        .unwrap()
        .isotope_settings_filter(false)
        .unwrap()
        .structure_search_db(FormulaSearchDB::Coconut)
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
        .ms1_absolute_intensity_error(-0.5)
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

    //Check if the file exists before attempting to remove it
    if output_file_path.exists() {
        fs::remove_file(output_file_path)
            .map_err(|e| format!("Failed to remove output file: {}", e))
            .unwrap();
    }

    sirius.run(input_file_path, output_file_path).unwrap();
}

#[test]
fn test_run_sirius_default() -> Result<(), String> {
    let sirius = SiriusBuilder::<Version5>::default()
        .maximal_mz_default()?
        .build();

    let input_file_path = Path::new("tests/data/input_sirius.mgf");
    let output_file_path = Path::new("tests/data/output_sirius");

    //Check if the file exists before attempting to remove it
    if output_file_path.exists() {
        fs::remove_file(output_file_path)
            .map_err(|e| format!("Failed to remove output file: {}", e))?;
    }

    sirius.run(input_file_path, output_file_path).unwrap();

    Ok(())
}

#[test]
fn test_run_sirius() -> Result<(), String> {
    let sirius = SiriusBuilder::default()
        .maximal_mz(802.2)?
        .formula_search_db(FormulaSearchDB::Bio)?
        .isotope_settings_filter(false)?
        .structure_search_db(FormulaSearchDB::Coconut)?
        .timeout_seconds_per_tree_default()?
        .number_of_candidates_per_ion(5)?
        .adduct_settings_detectable_default()?
        .timeout_seconds_per_instance_default()?
        .formula_result_threshold_default()?
        .inject_el_gordo_compounds_default()?
        .median_noise_intensity(0.013)?
        .ms1_absolute_intensity_error(0.5)?
        .ms1_minimal_intensity_to_consider(0.4)?
        .ms1_relative_intensity_error_default()?
        .noise_threshold_settings_absolute_threshold_default()?
        .noise_threshold_settings_intensity_threshold(0.2)?
        .noise_threshold_settings_maximal_number_of_peaks(42)?
        .number_of_structure_candidates(2)?
        .number_of_candidates(3)?
        .zodiac_cluster_compounds(true)?
        .zodiac_edge_filter_thresholds_min_local_candidates_default()?
        .zodiac_edge_filter_thresholds_min_local_connections_default()?
        .enable_formula()?
        .enable_zodiac()?
        .enable_fingerprint()?
        .enable_structure()?
        .enable_canopus()?
        .build();

    let input_file_path = Path::new("tests/data/input_sirius.mgf");
    let output_file_path = Path::new("tests/data/output_sirius");

    //Check if the file exists before attempting to remove it
    // if output_file_path.exists() {
    // fs::remove_file(output_file_path)
    // .map_err(|e| format!("Failed to remove output file: {}", e))?;
    // }

    sirius.run(input_file_path, output_file_path).unwrap();

    Ok(())
}
