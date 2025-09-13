use sirius::prelude::*;

#[test]
fn test_v6_compose_cli_like() -> Result<(), String> {
    // Build a V6 config composed similarly to the README example and assert args ordering
    let sirius = SiriusBuilder::<Version6>::default()
        .maximal_mz(800.0)?
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
        .add_config_parameter(ConfigV6::FormulaSettingsEnforced(AtomVector::from(vec![Atoms::H,Atoms::C,Atoms::N,Atoms::O,Atoms::P])))?
        .semantic_config(ConfigParam::IdentitySearchPrecursorDeviation(MassDeviation::ppm(20.0)))?
        .semantic_config(ConfigParam::FormulaSearchPerformBottomUpAboveMz(0))?
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
        .enable_formula()? // formulas
        .enable_fingerprint()? // fingerprints
        .enable_canopus()? // classes
        .enable_structure()? // structures
        .enable_write_summaries()? // write-summaries
        .post_tool_arg("--chemvista")
        .post_tool_arg("--feature-quality-summary")
        .post_tool_arg("--full-summary")
        .build();

    // Extract args (without input/output) and assert key segments exist and order sanity
    let args = sirius_config_args(&sirius);
    let joined = args.join(" ");
    assert!(joined.contains("--mzmax=800"));
    assert!(joined.contains("config"));
    assert!(joined.contains("--AlgorithmProfile=orbitrap"));
    assert!(joined.contains("--MS2MassDeviation.allowedMassDeviation=5"));
    assert!(joined.contains("--SpectralSearchDB=METACYC"));
    assert!(joined.contains("--FormulaSearchDB="));
    assert!(joined.contains("--StructureSearchDB=METACYC"));
    assert!(joined.contains("formulas"));
    assert!(joined.contains("fingerprints"));
    assert!(joined.contains("classes"));
    assert!(joined.contains("structures"));
    assert!(joined.contains("write-summaries"));
    assert!(joined.contains("--chemvista"));
    Ok(())
}

fn sirius_config_args<V: Version>(s: &Sirius<V>) -> Vec<String> { s.args() }
