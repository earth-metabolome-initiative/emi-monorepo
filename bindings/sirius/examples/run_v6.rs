use sirius::prelude::*;
use std::path::PathBuf;

// Run a SIRIUS 6.3.0 job similar to the README example.
// Requires a local SIRIUS installation and credentials set via env/`.env`.
fn main() -> Result<(), String> {
    // Resolve input/output relative to the sirius crate dir
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let input_file_path: PathBuf = [crate_dir, "tests", "data", "input_sirius.mgf"].iter().collect();
    let output_file_path: PathBuf = [crate_dir, "tests", "data", "output_sirius.sirius"].iter().collect();

    if output_file_path.exists() {
        let _ = std::fs::remove_dir_all(&output_file_path);
    }

    let sirius = SiriusBuilder::<Version6>::default()
        // core
        .maximal_mz(800.0)?
        // config
        .add_config_parameter(ConfigV6::AlgorithmProfile(Instruments::Orbitrap))?
        .semantic_config(ConfigParam::Ms2Allowed(MassDeviation::ppm(5.0)))?
        // spectral dbs
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
            Atoms::H, Atoms::C, Atoms::N, Atoms::O, Atoms::P,
        ])))?
        .semantic_config(ConfigParam::IdentitySearchPrecursorDeviation(MassDeviation::ppm(20.0)))?
        // If you want this typed as well, add a dedicated semantic later
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

    let input = input_file_path.as_path();
    let output = output_file_path.as_path();
    sirius.run(input, output)?;
    println!("Wrote SIRIUS project: {}", output.display());
    Ok(())
}

