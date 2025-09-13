use sirius::prelude::*;
use std::path::Path;

// This is an end-to-end test that runs the sirius 6.x CLI via the bindings.
// It is ignored by default because it requires a local SIRIUS installation and credentials.
// Run with: cargo test -p sirius --test test_v6_run -- --ignored --nocapture
#[test]
#[ignore]
fn run_v6_end_to_end() -> Result<(), String> {
    let sirius = SiriusBuilder::<Version6>::default()
        // core
        .maximal_mz(800.0)?
        // config
        .add_config_parameter(ConfigV6::AlgorithmProfile(Instruments::Orbitrap))?
        .semantic_config(ConfigParam::Ms2Allowed(MassDeviation::ppm(5.0)))?
        .semantic_config(ConfigParam::SpectralSearchDb(vec![
            SearchDB::Metacyc,
            SearchDB::BloodExposome,
            SearchDB::Chebi,
            SearchDB::Coconut,
            SearchDB::FooDB,
            SearchDB::Gnps,
            SearchDB::Hmdb,
            SearchDB::Hsdb,
            SearchDB::Kegg,
            SearchDB::Knapsack,
            SearchDB::Lotus,
            SearchDB::LipidMaps,
            SearchDB::Maconda,
            SearchDB::Mesh,
            SearchDB::MiMeDB,
            SearchDB::Norman,
            SearchDB::Plantcyc,
            SearchDB::PubchemAnnotationBio,
            SearchDB::PubchemAnnotationDrug,
            SearchDB::PubchemAnnotationFood,
            SearchDB::PubchemAnnotationSafetyAndToxic,
            SearchDB::Supernatural,
            SearchDB::TeroMol,
            SearchDB::Ymdb,
        ]))?
        .add_config_parameter(ConfigV6::AdductSettingsFallback(AdductsVector::from(vec![
            Adducts::MplusHplus,
            Adducts::MplusNaplus,
            Adducts::MplusKplus,
            Adducts::MplusH3NplusHplus,
            Adducts::MplusH2OplusHplus,
        ])))?
        .add_config_parameter(ConfigV6::FormulaSettingsEnforced(AtomVector::from(vec![
            Atoms::H,
            Atoms::C,
            Atoms::N,
            Atoms::O,
            Atoms::P,
        ])))?
        .semantic_config(ConfigParam::IdentitySearchPrecursorDeviation(MassDeviation::ppm(20.0)))?
        .semantic_config(ConfigParam::Raw("--FormulaSearchSettings.performBottomUpAboveMz=0".to_string()))?
        .semantic_config(ConfigParam::FormulaDbList(vec![]))?
        .semantic_config(ConfigParam::StructureDbList(vec![
            SearchDB::Metacyc,
            SearchDB::BloodExposome,
            SearchDB::Chebi,
            SearchDB::Coconut,
            SearchDB::FooDB,
            SearchDB::Gnps,
            SearchDB::Hmdb,
            SearchDB::Hsdb,
            SearchDB::Kegg,
            SearchDB::Knapsack,
            SearchDB::Lotus,
            SearchDB::LipidMaps,
            SearchDB::Maconda,
            SearchDB::Mesh,
            SearchDB::MiMeDB,
            SearchDB::Norman,
            SearchDB::Plantcyc,
            SearchDB::PubchemAnnotationBio,
            SearchDB::PubchemAnnotationDrug,
            SearchDB::PubchemAnnotationFood,
            SearchDB::PubchemAnnotationSafetyAndToxic,
            SearchDB::Supernatural,
            SearchDB::TeroMol,
            SearchDB::Ymdb,
        ]))?
        // tools
        .enable_formula()? // formulas
        .enable_fingerprint()? // fingerprints
        .enable_canopus()? // classes
        .enable_structure()? // structures
        .enable_write_summaries()? // write-summaries
        // post-tool args
        .post_tool_arg("--chemvista")
        .post_tool_arg("--feature-quality-summary")
        .post_tool_arg("--full-summary")
        .build();

    let input_file_path = Path::new("tests/data/input_sirius.mgf");
    let output_file_path = Path::new("tests/data/output_sirius.sirius");
    if output_file_path.exists() {
        let _ = std::fs::remove_dir_all(output_file_path);
    }
    sirius.run(input_file_path, output_file_path)?;
    assert!(output_file_path.exists());
    Ok(())
}

