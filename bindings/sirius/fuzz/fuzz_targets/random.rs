#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use sirius::prelude::*;

#[derive(Arbitrary, Debug)]
struct FuzzCase {
    random_state: u64,
    maximal_mz: f64,
    formula_search_db: SearchDB,
    isotope_settings_filter: bool,
    structure_search_db: SearchDB,
    timeout_seconds_per_tree: u32,
    number_of_candidates: u32,
    number_of_candidates_per_ion: u32,
    number_of_structure_candidates: u32,
    recompute_results: bool,
    print_citations: bool,
    timeout_seconds_per_instance: u32,
    formula_result_threshold: bool,
    inject_el_gordo_compounds: bool,
    median_noise_intensity: f32,
    ms1_absolute_intensity_error: f32,
    ms1_minimal_intensity_to_consider: f32,
    ms1_relative_intensity_error: f32,
    noise_threshold_settings_intensity_threshold: f32,
    noise_threshold_settings_maximal_number_of_peaks: u32,
    zodiac_cluster_compounds: bool,
    zodiac_edge_filter_thresholds_min_local_candidates: u32,
    zodiac_edge_filter_thresholds_min_local_connections: u32,
    zodiac_edge_filter_thresholds_threshold_filter: f32,
    zodiac_epochs_burn_in_period: u32,
    zodiac_epochs_iterations: u32,
    zodiac_epochs_number_of_markov_chains: u32,
    zodiac_library_scoring_lambda: u32,
    zodiac_library_scoring_min_cosine: f32,
    zodiac_number_of_considered_candidates_at_300_mz: i32,
    zodiac_number_of_considered_candidates_at_800_mz: i32,
    zodiac_ratio_of_considered_candidates_per_ionization: f32,
    zodiac_run_in_two_steps: bool,
    ms1_mass_deviation_allowed_mass_deviation: MassDeviation,
    ms1_mass_deviation_mass_difference_deviation: MassDeviation,
    ms1_mass_deviation_standard_mass_deviation: MassDeviation,
    ms2_mass_deviation_standard_mass_deviation: MassDeviation,
    ms2_mass_deviation_allowed_mass_deviation: MassDeviation,
    formula_settings_detectable: AtomVector,
    formula_settings_enforced: AtomVector,
    formula_settings_fallback: AtomVector,
    forbid_recalibration: ForbidRecalibration,
    use_heuristic_mz_to_use_heuristic: u32,
    use_heuristic_mz_to_use_heuristic_only: u32,
    adduct_settings_detectable: AdductsVector,
    adduct_settings_fallback: AdductsVector,
    algorithm_profile: Instruments,
    compound_quality: CompoundQuality,
    adduct_settings_enforced: AdductSettingsEnforced,
    candidate_formulas: CandidateFormulas,
    formula_result_ranking_score: FormulaResultRankingScore,
    isotope_ms2_settings: IsotopeMS2Settings,
    isotope_settings_multiplier: u32,
    noise_threshold_settings_absolute_threshold: u32,
    noise_threshold_settings_base_peak: BasePeak,
    structure_predictors: StructurePredictors,
    possible_adduct_switches: PossibleAdductSwitches,
}

fuzz_target!(|params: FuzzCase| {
    let rng = SmallRng::seed_from_u64(params.random_state);
    let builder = SiriusBuilder::default();
    let _ = router(rng, builder, &params);
});

const NUMBER_OF_METHODS: usize = 112;

fn router(
    mut random_state: SmallRng,
    builder: SiriusBuilder<Version5>,
    params: &FuzzCase,
) -> Result<(), String> {
    // we sample a random integer to decide which method to call
    let state = random_state.gen_range(0..NUMBER_OF_METHODS + 1);
    match state {
        0 => {
            router(random_state, builder.maximal_mz_default()?, params)?;
        }
        1 => {
            router(random_state, builder.maximal_mz(params.maximal_mz)?, params)?;
        }
        2 => {
            router(
                random_state,
                builder.formula_search_db(params.formula_search_db)?,
                params,
            )?;
        }
        3 => {
            router(random_state, builder.formula_search_db_default()?, params)?;
        }
        4 => {
            router(
                random_state,
                builder.isotope_settings_filter(params.isotope_settings_filter)?,
                params,
            )?;
        }
        5 => {
            router(
                random_state,
                builder.isotope_settings_filter_default()?,
                params,
            )?;
        }
        6 => {
            router(
                random_state,
                builder.structure_search_db(params.structure_search_db)?,
                params,
            )?;
        }
        7 => {
            router(random_state, builder.structure_search_db_default()?, params)?;
        }
        8 => {
            router(
                random_state,
                builder.timeout_seconds_per_tree(params.timeout_seconds_per_tree)?,
                params,
            )?;
        }
        9 => {
            router(
                random_state,
                builder.timeout_seconds_per_tree_default()?,
                params,
            )?;
        }
        10 => {
            router(
                random_state,
                builder.number_of_candidates(params.number_of_candidates)?,
                params,
            )?;
        }
        11 => {
            router(
                random_state,
                builder.number_of_candidates_default()?,
                params,
            )?;
        }
        12 => {
            router(
                random_state,
                builder.number_of_candidates_per_ion(params.number_of_candidates_per_ion)?,
                params,
            )?;
        }
        13 => {
            router(
                random_state,
                builder.number_of_candidates_per_ion_default()?,
                params,
            )?;
        }
        14 => {
            router(
                random_state,
                builder.number_of_structure_candidates(params.number_of_structure_candidates)?,
                params,
            )?;
        }
        15 => {
            router(
                random_state,
                builder.number_of_structure_candidates_default()?,
                params,
            )?;
        }
        16 => {
            router(
                random_state,
                builder.recompute_results(params.recompute_results)?,
                params,
            )?;
        }
        17 => {
            router(random_state, builder.recompute_results_default()?, params)?;
        }
        18 => {
            router(
                random_state,
                builder.print_citations(params.print_citations)?,
                params,
            )?;
        }
        19 => {
            router(random_state, builder.print_citations_default()?, params)?;
        }
        20 => {
            router(
                random_state,
                builder.timeout_seconds_per_instance(params.timeout_seconds_per_instance)?,
                params,
            )?;
        }
        21 => {
            router(
                random_state,
                builder.timeout_seconds_per_instance_default()?,
                params,
            )?;
        }
        22 => {
            router(
                random_state,
                builder.formula_result_threshold(params.formula_result_threshold)?,
                params,
            )?;
        }
        23 => {
            router(
                random_state,
                builder.formula_result_threshold_default()?,
                params,
            )?;
        }
        24 => {
            router(
                random_state,
                builder.inject_el_gordo_compounds(params.inject_el_gordo_compounds)?,
                params,
            )?;
        }
        25 => {
            router(
                random_state,
                builder.inject_el_gordo_compounds_default()?,
                params,
            )?;
        }
        26 => {
            router(
                random_state,
                builder.median_noise_intensity(params.median_noise_intensity)?,
                params,
            )?;
        }
        27 => {
            router(
                random_state,
                builder.median_noise_intensity_default()?,
                params,
            )?;
        }
        28 => {
            router(
                random_state,
                builder.ms1_absolute_intensity_error(params.ms1_absolute_intensity_error)?,
                params,
            )?;
        }
        29 => {
            router(
                random_state,
                builder.ms1_absolute_intensity_error_default()?,
                params,
            )?;
        }
        30 => {
            router(
                random_state,
                builder
                    .ms1_minimal_intensity_to_consider(params.ms1_minimal_intensity_to_consider)?,
                params,
            )?;
        }
        31 => {
            router(
                random_state,
                builder.ms1_minimal_intensity_to_consider_default()?,
                params,
            )?;
        }
        32 => {
            router(
                random_state,
                builder.ms1_relative_intensity_error(params.ms1_relative_intensity_error)?,
                params,
            )?;
        }
        33 => {
            router(
                random_state,
                builder.ms1_relative_intensity_error_default()?,
                params,
            )?;
        }
        34 => {
            router(
                random_state,
                builder.noise_threshold_settings_intensity_threshold(
                    params.noise_threshold_settings_intensity_threshold,
                )?,
                params,
            )?;
        }
        35 => {
            router(
                random_state,
                builder.noise_threshold_settings_intensity_threshold_default()?,
                params,
            )?;
        }
        36 => {
            router(
                random_state,
                builder.noise_threshold_settings_maximal_number_of_peaks(
                    params.noise_threshold_settings_maximal_number_of_peaks,
                )?,
                params,
            )?;
        }
        37 => {
            router(
                random_state,
                builder.noise_threshold_settings_maximal_number_of_peaks_default()?,
                params,
            )?;
        }
        38 => {
            router(
                random_state,
                builder.zodiac_cluster_compounds(params.zodiac_cluster_compounds)?,
                params,
            )?;
        }
        39 => {
            router(
                random_state,
                builder.zodiac_cluster_compounds_default()?,
                params,
            )?;
        }
        40 => {
            router(
                random_state,
                builder.zodiac_edge_filter_thresholds_min_local_candidates(
                    params.zodiac_edge_filter_thresholds_min_local_candidates,
                )?,
                params,
            )?;
        }
        41 => {
            router(
                random_state,
                builder.zodiac_edge_filter_thresholds_min_local_candidates_default()?,
                params,
            )?;
        }
        42 => {
            router(
                random_state,
                builder.zodiac_edge_filter_thresholds_min_local_connections(
                    params.zodiac_edge_filter_thresholds_min_local_connections,
                )?,
                params,
            )?;
        }
        43 => {
            router(
                random_state,
                builder.zodiac_edge_filter_thresholds_min_local_connections_default()?,
                params,
            )?;
        }
        44 => {
            router(
                random_state,
                builder.zodiac_edge_filter_thresholds_threshold_filter(
                    params.zodiac_edge_filter_thresholds_threshold_filter,
                )?,
                params,
            )?;
        }
        45 => {
            router(
                random_state,
                builder.zodiac_edge_filter_thresholds_threshold_filter_default()?,
                params,
            )?;
        }
        46 => {
            router(
                random_state,
                builder.zodiac_epochs_burn_in_period(params.zodiac_epochs_burn_in_period)?,
                params,
            )?;
        }
        47 => {
            router(
                random_state,
                builder.zodiac_epochs_burn_in_period_default()?,
                params,
            )?;
        }
        48 => {
            router(
                random_state,
                builder.zodiac_epochs_iterations(params.zodiac_epochs_iterations)?,
                params,
            )?;
        }
        49 => {
            router(
                random_state,
                builder.zodiac_epochs_iterations_default()?,
                params,
            )?;
        }
        50 => {
            router(
                random_state,
                builder.zodiac_epochs_number_of_markov_chains(
                    params.zodiac_epochs_number_of_markov_chains,
                )?,
                params,
            )?;
        }
        51 => {
            router(
                random_state,
                builder.zodiac_epochs_number_of_markov_chains_default()?,
                params,
            )?;
        }
        52 => {
            router(
                random_state,
                builder.zodiac_library_scoring_lambda(params.zodiac_library_scoring_lambda)?,
                params,
            )?;
        }
        53 => {
            router(
                random_state,
                builder.zodiac_library_scoring_lambda_default()?,
                params,
            )?;
        }
        54 => {
            router(
                random_state,
                builder
                    .zodiac_library_scoring_min_cosine(params.zodiac_library_scoring_min_cosine)?,
                params,
            )?;
        }
        55 => {
            router(
                random_state,
                builder.zodiac_library_scoring_min_cosine_default()?,
                params,
            )?;
        }
        56 => {
            router(
                random_state,
                builder.zodiac_number_of_considered_candidates_at_300_mz(
                    params.zodiac_number_of_considered_candidates_at_300_mz,
                )?,
                params,
            )?;
        }
        57 => {
            router(
                random_state,
                builder.zodiac_number_of_considered_candidates_at_300_mz_default()?,
                params,
            )?;
        }
        58 => {
            router(
                random_state,
                builder.zodiac_number_of_considered_candidates_at_800_mz(
                    params.zodiac_number_of_considered_candidates_at_800_mz,
                )?,
                params,
            )?;
        }
        59 => {
            router(
                random_state,
                builder.zodiac_number_of_considered_candidates_at_800_mz_default()?,
                params,
            )?;
        }
        60 => {
            router(
                random_state,
                builder.zodiac_ratio_of_considered_candidates_per_ionization(
                    params.zodiac_ratio_of_considered_candidates_per_ionization,
                )?,
                params,
            )?;
        }
        61 => {
            router(
                random_state,
                builder.zodiac_ratio_of_considered_candidates_per_ionization_default()?,
                params,
            )?;
        }
        62 => {
            router(
                random_state,
                builder.zodiac_run_in_two_steps(params.zodiac_run_in_two_steps)?,
                params,
            )?;
        }
        63 => {
            router(
                random_state,
                builder.zodiac_run_in_two_steps_default()?,
                params,
            )?;
        }
        64 => {
            router(
                random_state,
                builder.ms1_mass_deviation_allowed_mass_deviation(
                    params.ms1_mass_deviation_allowed_mass_deviation,
                )?,
                params,
            )?;
        }
        65 => {
            router(
                random_state,
                builder.ms1_mass_deviation_allowed_mass_deviation_default()?,
                params,
            )?;
        }
        66 => {
            router(
                random_state,
                builder.ms1_mass_deviation_mass_difference_deviation(
                    params.ms1_mass_deviation_mass_difference_deviation,
                )?,
                params,
            )?;
        }
        67 => {
            router(
                random_state,
                builder.ms1_mass_deviation_mass_difference_deviation_default()?,
                params,
            )?;
        }
        68 => {
            router(
                random_state,
                builder.ms1_mass_deviation_standard_mass_deviation(
                    params.ms1_mass_deviation_standard_mass_deviation,
                )?,
                params,
            )?;
        }
        69 => {
            router(
                random_state,
                builder.ms1_mass_deviation_standard_mass_deviation_default()?,
                params,
            )?;
        }
        70 => {
            router(
                random_state,
                builder.ms2_mass_deviation_standard_mass_deviation(
                    params.ms2_mass_deviation_standard_mass_deviation,
                )?,
                params,
            )?;
        }
        71 => {
            router(
                random_state,
                builder.ms2_mass_deviation_standard_mass_deviation_default()?,
                params,
            )?;
        }
        72 => {
            router(
                random_state,
                builder.ms2_mass_deviation_allowed_mass_deviation(
                    params.ms2_mass_deviation_allowed_mass_deviation,
                )?,
                params,
            )?;
        }
        73 => {
            router(
                random_state,
                builder.ms2_mass_deviation_allowed_mass_deviation_default()?,
                params,
            )?;
        }
        74 => {
            router(
                random_state,
                builder.formula_settings_detectable(params.formula_settings_detectable.clone())?,
                params,
            )?;
        }
        75 => {
            router(
                random_state,
                builder.formula_settings_detectable_default()?,
                params,
            )?;
        }
        76 => {
            router(
                random_state,
                builder.formula_settings_enforced(params.formula_settings_enforced.clone())?,
                params,
            )?;
        }
        77 => {
            router(
                random_state,
                builder.formula_settings_enforced_default()?,
                params,
            )?;
        }
        78 => {
            router(
                random_state,
                builder.formula_settings_fallback(params.formula_settings_fallback.clone())?,
                params,
            )?;
        }
        79 => {
            router(
                random_state,
                builder.formula_settings_fallback_default()?,
                params,
            )?;
        }
        80 => {
            router(
                random_state,
                builder.forbid_recalibration(params.forbid_recalibration)?,
                params,
            )?;
        }
        81 => {
            router(
                random_state,
                builder.forbid_recalibration_default()?,
                params,
            )?;
        }
        82 => {
            router(
                random_state,
                builder
                    .use_heuristic_mz_to_use_heuristic(params.use_heuristic_mz_to_use_heuristic)?,
                params,
            )?;
        }
        83 => {
            router(
                random_state,
                builder.use_heuristic_mz_to_use_heuristic_default()?,
                params,
            )?;
        }
        84 => {
            router(
                random_state,
                builder.use_heuristic_mz_to_use_heuristic_only(
                    params.use_heuristic_mz_to_use_heuristic_only,
                )?,
                params,
            )?;
        }
        85 => {
            router(
                random_state,
                builder.use_heuristic_mz_to_use_heuristic_only_default()?,
                params,
            )?;
        }
        86 => {
            router(
                random_state,
                builder.adduct_settings_detectable(params.adduct_settings_detectable.clone())?,
                params,
            )?;
        }
        87 => {
            router(
                random_state,
                builder.adduct_settings_detectable_default()?,
                params,
            )?;
        }
        88 => {
            router(
                random_state,
                builder.adduct_settings_fallback(params.adduct_settings_fallback.clone())?,
                params,
            )?;
        }
        89 => {
            router(
                random_state,
                builder.adduct_settings_fallback_default()?,
                params,
            )?;
        }
        90 => {
            router(
                random_state,
                builder.algorithm_profile(params.algorithm_profile)?,
                params,
            )?;
        }
        91 => {
            router(random_state, builder.algorithm_profile_default()?, params)?;
        }
        92 => {
            router(
                random_state,
                builder.compound_quality(params.compound_quality)?,
                params,
            )?;
        }
        93 => {
            router(random_state, builder.compound_quality_default()?, params)?;
        }
        94 => {
            router(
                random_state,
                builder.adduct_settings_enforced(params.adduct_settings_enforced)?,
                params,
            )?;
        }
        95 => {
            router(
                random_state,
                builder.adduct_settings_enforced_default()?,
                params,
            )?;
        }
        96 => {
            router(
                random_state,
                builder.candidate_formulas(params.candidate_formulas)?,
                params,
            )?;
        }
        97 => {
            router(random_state, builder.candidate_formulas_default()?, params)?;
        }
        98 => {
            router(
                random_state,
                builder.formula_result_ranking_score(params.formula_result_ranking_score)?,
                params,
            )?;
        }
        99 => {
            router(
                random_state,
                builder.formula_result_ranking_score_default()?,
                params,
            )?;
        }
        100 => {
            router(
                random_state,
                builder.isotope_ms2_settings(params.isotope_ms2_settings)?,
                params,
            )?;
        }
        101 => {
            router(
                random_state,
                builder.isotope_ms2_settings_default()?,
                params,
            )?;
        }
        102 => {
            router(
                random_state,
                builder.isotope_settings_multiplier(params.isotope_settings_multiplier)?,
                params,
            )?;
        }
        103 => {
            router(
                random_state,
                builder.isotope_settings_multiplier_default()?,
                params,
            )?;
        }
        104 => {
            router(
                random_state,
                builder.noise_threshold_settings_absolute_threshold(
                    params.noise_threshold_settings_absolute_threshold,
                )?,
                params,
            )?;
        }
        105 => {
            router(
                random_state,
                builder.noise_threshold_settings_absolute_threshold_default()?,
                params,
            )?;
        }
        106 => {
            router(
                random_state,
                builder.noise_threshold_settings_base_peak(
                    params.noise_threshold_settings_base_peak,
                )?,
                params,
            )?;
        }
        107 => {
            router(
                random_state,
                builder.noise_threshold_settings_base_peak_default()?,
                params,
            )?;
        }
        108 => {
            router(
                random_state,
                builder.structure_predictors(params.structure_predictors)?,
                params,
            )?;
        }
        109 => {
            router(
                random_state,
                builder.structure_predictors_default()?,
                params,
            )?;
        }
        110 => {
            router(
                random_state,
                builder.possible_adduct_switches(params.possible_adduct_switches)?,
                params,
            )?;
        }
        111 => {
            router(
                random_state,
                builder.possible_adduct_switches_default()?,
                params,
            )?;
        }
        _ => {
            builder.build();
        }
    }
    Ok(())
}
