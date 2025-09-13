use crate::prelude::*;
use crate::traits::{Enablable, IntoDefault, NamedParametersSet};

/// The possible config settings
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigV5 {
    /// If the config is enabled
    Enabled,

    /// The isotope settings filter
    IsotopeSettingsFilter(bool),

    /// The formula search db
    FormulaSearchDB(SearchDB),

    /// The structure search db
    StructureSearchDB(SearchDB),

    /// The timeout seconds per tree
    TimeoutSecondsPerTree(u32),

    /// The number of candidates per ion
    NumberOfCandidates(u32),

    /// The number of candidates per ion
    NumberOfCandidatesPerIon(u32),

    /// The number of structure candidates
    NumberOfStructureCandidates(u32),

    /// Whether to recompute results
    RecomputeResults(bool),

    /// Whether to print citations
    PrintCitations(bool),

    /// The timeout seconds per instance
    TimeoutSecondsPerInstance(u32),

    /// Whether to use formula result threshold
    FormulaResultThreshold(bool),

    /// Candidates matching the lipid class estimated by El Gordo will be tagged.
    InjectElGordoCompounds(bool),

    /// The median noise intensity
    MedianNoiseIntensity(f32),

    /// The MS1 absolute intensity error
    MS1AbsoluteIntensityError(f32),

    /// The MS1 minimal intensity to consider
    MS1MinimalIntensityToConsider(f32),

    /// The MS1 relative intensity error
    MS1RelativeIntensityError(f32),

    /// The noise threshold settings intensity threshold
    NoiseThresholdSettingsIntensityThreshold(f32),

    /// The noise threshold settings maximal number of peaks
    NoiseThresholdSettingsMaximalNumberOfPeaks(u32),

    /// Whether to cluster compounds before running zodiac
    ZodiacClusterCompounds(bool),

    /// The zodiac edge filter thresholds min local candidates
    ZodiacEdgeFilterThresholdsMinLocalCandidates(u32),

    /// The zodiac edge filter thresholds min local connections
    ZodiacEdgeFilterThresholdsMinLocalConnections(u32),

    /// The zodiac edge filter thresholds threshold filter
    ZodiacEdgeFilterThresholdsThresholdFilter(f32),

    /// The zodiac epochs burn in period
    ZodiacEpochsBurnInPeriod(u32),

    /// The zodiac epochs iterations
    ZodiacEpochsIterations(u32),

    /// The number of markov chains for zodiac
    ZodiacEpochsNumberOfMarkovChains(u32),

    /// The zodiac library scoring lambda
    ZodiacLibraryScoringLambda(u32),

    /// The zodiac library scoring min cosine
    ZodiacLibraryScoringMinCosine(f32),

    /// The zodiac number of considered candidates at 300 mz
    ZodiacNumberOfConsideredCandidatesAt300Mz(i32),

    /// The zodiac number of considered candidates at 800 mz
    ZodiacNumberOfConsideredCandidatesAt800Mz(i32),

    /// The zodiac ratio of considered candidates per ionization
    ZodiacRatioOfConsideredCandidatesPerIonization(f32), //can't be negative, higher than 1 or NaN

    /// Whether to run zodiac in two steps
    ZodiacRunInTwoSteps(bool),

    /// The MS1 mass deviation allowed mass deviation
    MS1MassDeviationAllowedMassDeviation(MassDeviation),

    /// The MS1 mass deviation mass difference deviation
    MS1MassDeviationMassDifferenceDeviation(MassDeviation),

    /// The MS1 mass deviation standard mass deviation
    MS1MassDeviationStandardMassDeviation(MassDeviation),

    /// The MS2 mass deviation allowed mass deviation
    MS2MassDeviationAllowedMassDeviation(MassDeviation),

    /// The MS2 mass deviation standard mass deviation
    MS2MassDeviationStandardMassDeviation(MassDeviation),

    /// The formula settings detectable
    FormulaSettingsDetectable(AtomVector),

    /// The formula settings enforced
    FormulaSettingsEnforced(AtomVector),

    /// The formula settings fallback
    FormulaSettingsFallback(AtomVector),

    /// Whether to forbid recalibration
    ForbidRecalibration(ForbidRecalibration),

    /// The use heuristic mz to use heuristic
    UseHeuristicMZToUseHeuristic(u32),

    /// The use heuristic mz to use heuristic only
    UseHeuristicMZToUseHeuristicOnly(u32),

    /// The detectable adducts
    AdductSettingsDetectable(AdductsVector),

    /// The fallback adducts
    AdductSettingsFallback(AdductsVector),

    /// The algorithm profile
    AlgorithmProfile(Instruments),

    /// The compound quality
    CompoundQuality(CompoundQuality),

    /// The enforced adducts
    AdductSettingsEnforced(AdductSettingsEnforced),

    /// The candidate formulas
    CandidateFormulas(CandidateFormulas),

    /// The formula result ranking score
    FormulaResultRankingScore(FormulaResultRankingScore),

    /// The isotope ms2 settings
    IsotopeMS2Settings(IsotopeMS2Settings),

    /// The isotope settings multiplier
    IsotopeSettingsMultiplier(u32),

    /// The noise threshold settings absolute threshold
    NoiseThresholdSettingsAbsoluteThreshold(u32),

    /// The noise threshold settings base peak
    NoiseThresholdSettingsBasePeak(BasePeak),

    /// The structure predictors
    StructurePredictors(StructurePredictors),

    /// The possible adduct switches
    PossibleAdductSwitches(PossibleAdductSwitches),
}

impl ToString for ConfigV5 {
    fn to_string(&self) -> String {
        match self {
            ConfigV5::Enabled => Self::parameter_set_name().to_string(),
            ConfigV5::IsotopeSettingsFilter(isotope_settings_filter) => {
                format!("--IsotopeSettings.filter={}", isotope_settings_filter)
            }
            ConfigV5::FormulaSearchDB(formula_search_db) => {
                format!("--FormulaSearchDB={}", formula_search_db)
            }
            ConfigV5::StructureSearchDB(structure_search_db) => {
                format!("--StructureSearchDB={}", structure_search_db)
            }
            ConfigV5::TimeoutSecondsPerTree(timeout_seconds_per_tree) => {
                format!("--Timeout.secondsPerTree={}", timeout_seconds_per_tree)
            }
            ConfigV5::NumberOfCandidatesPerIon(number_of_candidates_per_ion) => {
                format!(
                    "--NumberOfCandidatesPerIon={}",
                    number_of_candidates_per_ion
                )
            }
            ConfigV5::NumberOfStructureCandidates(number_of_structure_candidates) => {
                format!(
                    "--NumberOfStructureCandidates={}",
                    number_of_structure_candidates
                )
            }
            ConfigV5::RecomputeResults(recompute_results) => {
                format!("--RecomputeResults={}", recompute_results)
            }
            ConfigV5::PrintCitations(print_citations) => {
                format!("--PrintCitations={}", print_citations)
            }
            ConfigV5::TimeoutSecondsPerInstance(timeout_seconds_per_instance) => {
                format!(
                    "--Timeout.secondsPerInstance={}",
                    timeout_seconds_per_instance
                )
            }
            ConfigV5::FormulaResultThreshold(formula_result_threshold) => {
                format!("--FormulaResultThreshold={}", formula_result_threshold)
            }
            ConfigV5::InjectElGordoCompounds(inject_el_gordo_compounds) => {
                format!("--InjectElGordoCompounds={}", inject_el_gordo_compounds)
            }
            ConfigV5::MedianNoiseIntensity(median_noise_intensity) => {
                format!("--MedianNoiseIntensity={}", median_noise_intensity)
            }
            ConfigV5::MS1AbsoluteIntensityError(ms1_absolute_intensity_error) => {
                format!(
                    "--ms1.absoluteIntensityError={}",
                    ms1_absolute_intensity_error
                )
            }
            ConfigV5::MS1MinimalIntensityToConsider(ms1_minimal_intensity_to_consider) => {
                format!(
                    "--ms1.minimalIntensityToConsider={}",
                    ms1_minimal_intensity_to_consider
                )
            }
            ConfigV5::MS1RelativeIntensityError(ms1_relative_intensity_error) => {
                format!(
                    "--ms1.relativeIntensityError={}",
                    ms1_relative_intensity_error
                )
            }
            ConfigV5::NoiseThresholdSettingsIntensityThreshold(
                noise_threshold_settings_intensity_threshold,
            ) => format!(
                "--NoiseThresholdSettings.intensityThreshold={}",
                noise_threshold_settings_intensity_threshold
            ),
            ConfigV5::NoiseThresholdSettingsMaximalNumberOfPeaks(
                noise_threshold_settings_maximal_number_of_peaks,
            ) => format!(
                "--NoiseThresholdSettings.maximalNumberOfPeaks={}",
                noise_threshold_settings_maximal_number_of_peaks
            ),
            ConfigV5::NumberOfCandidates(number_of_candidates) => {
                format!("--NumberOfCandidates={}", number_of_candidates)
            }
            ConfigV5::ZodiacClusterCompounds(zodiac_cluster_compounds) => {
                format!("--ZodiacClusterCompounds={}", zodiac_cluster_compounds)
            }
            ConfigV5::ZodiacEdgeFilterThresholdsMinLocalCandidates(
                zodiac_edge_filter_thresholds_min_local_candidates,
            ) => format!(
                "--ZodiacEdgeFilterThresholds.minLocalCandidates={}",
                zodiac_edge_filter_thresholds_min_local_candidates
            ),
            ConfigV5::ZodiacEdgeFilterThresholdsMinLocalConnections(
                zodiac_edge_filter_thresholds_min_local_connections,
            ) => format!(
                "--ZodiacEdgeFilterThresholds.minLocalConnections={}",
                zodiac_edge_filter_thresholds_min_local_connections
            ),
            ConfigV5::ZodiacEdgeFilterThresholdsThresholdFilter(
                zodiac_edge_filter_thresholds_threshold_filter,
            ) => format!(
                "--ZodiacEdgeFilterThresholds.thresholdFilter={}",
                zodiac_edge_filter_thresholds_threshold_filter
            ),
            ConfigV5::ZodiacEpochsBurnInPeriod(zodiac_epochs_burn_in_period) => format!(
                "--ZodiacEpochs.burnInPeriod={}",
                zodiac_epochs_burn_in_period
            ),
            ConfigV5::ZodiacEpochsIterations(zodiac_epochs_iterations) => {
                format!("--ZodiacEpochs.iterations={}", zodiac_epochs_iterations)
            }
            ConfigV5::ZodiacEpochsNumberOfMarkovChains(zodiac_epochs_number_of_markov_chains) => {
                format!(
                    "--ZodiacEpochs.numberOfMarkovChains={}",
                    zodiac_epochs_number_of_markov_chains
                )
            }
            ConfigV5::ZodiacLibraryScoringLambda(zodiac_library_scoring_lambda) => format!(
                "--ZodiacLibraryScoring.lambda={}",
                zodiac_library_scoring_lambda
            ),
            ConfigV5::ZodiacLibraryScoringMinCosine(zodiac_library_scoring_min_cosine) => format!(
                "--ZodiacLibraryScoring.minCosine={}",
                zodiac_library_scoring_min_cosine
            ),
            ConfigV5::ZodiacNumberOfConsideredCandidatesAt300Mz(
                zodiac_number_of_considered_candidates_at_300_mz,
            ) => format!(
                "--ZodiacNumberOfConsideredCandidatesAt300Mz={}",
                zodiac_number_of_considered_candidates_at_300_mz
            ),
            ConfigV5::ZodiacNumberOfConsideredCandidatesAt800Mz(
                zodiac_number_of_considered_candidates_at_800_mz,
            ) => format!(
                "--ZodiacNumberOfConsideredCandidatesAt800Mz={}",
                zodiac_number_of_considered_candidates_at_800_mz
            ),
            ConfigV5::ZodiacRatioOfConsideredCandidatesPerIonization(
                zodiac_ratio_of_considered_candidates_per_ionization,
            ) => format!(
                "--ZodiacRatioOfConsideredCandidatesPerIonization={}",
                zodiac_ratio_of_considered_candidates_per_ionization
            ),
            ConfigV5::ZodiacRunInTwoSteps(zodiac_run_in_two_steps) => {
                format!("--ZodiacRunInTwoSteps={}", zodiac_run_in_two_steps)
            }
            ConfigV5::MS1MassDeviationAllowedMassDeviation(
                ms1_mass_deviation_allowed_mass_deviation,
            ) => format!(
                "--MS1MassDeviation.allowedMassDeviation={}",
                ms1_mass_deviation_allowed_mass_deviation
            ),
            ConfigV5::MS1MassDeviationMassDifferenceDeviation(
                ms1_mass_deviation_mass_difference_deviation,
            ) => format!(
                "--MS1MassDeviation.massDifferenceDeviation={}",
                ms1_mass_deviation_mass_difference_deviation
            ),
            ConfigV5::MS1MassDeviationStandardMassDeviation(
                ms1_mass_deviation_standard_mass_deviation,
            ) => format!(
                "--MS1MassDeviation.standardMassDeviation={}",
                ms1_mass_deviation_standard_mass_deviation
            ),
            ConfigV5::MS2MassDeviationAllowedMassDeviation(
                ms2_mass_deviation_allowed_mass_deviation,
            ) => format!(
                "--MS2MassDeviation.allowedMassDeviation={}",
                ms2_mass_deviation_allowed_mass_deviation
            ),
            ConfigV5::MS2MassDeviationStandardMassDeviation(
                ms2_mass_deviation_standard_mass_deviation,
            ) => format!(
                "--MS2MassDeviation.standardMassDeviation={}",
                ms2_mass_deviation_standard_mass_deviation
            ),
            ConfigV5::FormulaSettingsDetectable(formula_settings_detectable) => format!(
                "--FormulaSettings.detectable={}",
                formula_settings_detectable
            ),
            ConfigV5::FormulaSettingsEnforced(formula_settings_enforced) => {
                format!("--FormulaSettings.enforced={}", formula_settings_enforced)
            }
            ConfigV5::FormulaSettingsFallback(formula_settings_fallback) => {
                format!("--FormulaSettings.fallback={}", formula_settings_fallback)
            }
            ConfigV5::ForbidRecalibration(forbid_recalibration) => {
                format!("--ForbidRecalibration={}", forbid_recalibration)
            }
            ConfigV5::UseHeuristicMZToUseHeuristic(use_heuristic_mz_to_use_heuristic) => {
                format!(
                    "--UseHeuristic.mzToUseHeuristic={}",
                    use_heuristic_mz_to_use_heuristic
                )
            }
            ConfigV5::UseHeuristicMZToUseHeuristicOnly(use_heuristic_mz_to_use_heuristic_only) => {
                format!(
                    "--UseHeuristic.mzToUseHeuristicOnly={}",
                    use_heuristic_mz_to_use_heuristic_only
                )
            }
            ConfigV5::AdductSettingsDetectable(adduct_settings_detectable) => {
                format!("--AdductSettings.detectable={}", adduct_settings_detectable)
            }
            ConfigV5::AdductSettingsFallback(adduct_settings_fallback) => {
                format!("--AdductSettings.fallback={}", adduct_settings_fallback)
            }
            ConfigV5::AlgorithmProfile(algorithm_profile) => {
                format!("--AlgorithmProfile={}", algorithm_profile)
            }
            ConfigV5::CompoundQuality(compound_quality) => {
                format!("--CompoundQuality={}", compound_quality)
            }
            ConfigV5::AdductSettingsEnforced(adduct_settings_enforced) => {
                format!("--AdductSettings.enforced={}", adduct_settings_enforced)
            }
            ConfigV5::CandidateFormulas(candidate_formulas) => {
                format!("--CandidateFormulas={}", candidate_formulas)
            }
            ConfigV5::FormulaResultRankingScore(formula_result_ranking_score) => {
                format!(
                    "--FormulaResultRankingScore={}",
                    formula_result_ranking_score
                )
            }
            ConfigV5::IsotopeMS2Settings(isotope_ms2_settings) => {
                format!("--IsotopeMs2Settings={}", isotope_ms2_settings)
            }
            ConfigV5::IsotopeSettingsMultiplier(isotope_settings_multiplier) => {
                format!(
                    "--IsotopeSettings.multiplier={}",
                    isotope_settings_multiplier
                )
            }
            ConfigV5::NoiseThresholdSettingsAbsoluteThreshold(
                noise_threshold_settings_absolute_threshold,
            ) => format!(
                "--NoiseThresholdSettings.absoluteThreshold={}",
                noise_threshold_settings_absolute_threshold
            ),
            ConfigV5::NoiseThresholdSettingsBasePeak(noise_threshold_settings_base_peak) => {
                format!(
                    "--NoiseThresholdSettings.basePeak={}",
                    noise_threshold_settings_base_peak
                )
            }
            ConfigV5::StructurePredictors(structure_predictors) => {
                format!("--StructurePredictors={}", structure_predictors)
            }
            ConfigV5::PossibleAdductSwitches(possible_adduct_switches) => {
                format!("--PossibleAdductSwitches={}", possible_adduct_switches)
            }
        }
    }
}

impl IntoDefault for ConfigV5 {
    fn into_default(self) -> Self {
        match self {
            ConfigV5::Enabled => ConfigV5::Enabled,
            ConfigV5::IsotopeSettingsFilter(_) => ConfigV5::IsotopeSettingsFilter(true),
            ConfigV5::FormulaSearchDB(_) => ConfigV5::FormulaSearchDB(SearchDB::None),
            ConfigV5::StructureSearchDB(_) => ConfigV5::StructureSearchDB(SearchDB::Bio),
            ConfigV5::TimeoutSecondsPerTree(_) => ConfigV5::TimeoutSecondsPerTree(0),
            ConfigV5::NumberOfCandidatesPerIon(_) => ConfigV5::NumberOfCandidatesPerIon(1),
            ConfigV5::NumberOfStructureCandidates(_) => {
                ConfigV5::NumberOfStructureCandidates(10000)
            }
            ConfigV5::RecomputeResults(_) => ConfigV5::RecomputeResults(false),
            ConfigV5::PrintCitations(_) => ConfigV5::PrintCitations(true),
            ConfigV5::TimeoutSecondsPerInstance(_) => ConfigV5::TimeoutSecondsPerInstance(0),
            ConfigV5::FormulaResultThreshold(_) => ConfigV5::FormulaResultThreshold(true),
            ConfigV5::InjectElGordoCompounds(_) => ConfigV5::InjectElGordoCompounds(true),
            ConfigV5::MedianNoiseIntensity(_) => ConfigV5::MedianNoiseIntensity(0.015),
            ConfigV5::MS1AbsoluteIntensityError(_) => ConfigV5::MS1AbsoluteIntensityError(0.02),
            ConfigV5::MS1MinimalIntensityToConsider(_) => {
                ConfigV5::MS1MinimalIntensityToConsider(0.01)
            }
            ConfigV5::MS1RelativeIntensityError(_) => ConfigV5::MS1RelativeIntensityError(0.08),
            ConfigV5::NoiseThresholdSettingsIntensityThreshold(_) => {
                ConfigV5::NoiseThresholdSettingsIntensityThreshold(0.005)
            }
            ConfigV5::NoiseThresholdSettingsMaximalNumberOfPeaks(_) => {
                ConfigV5::NoiseThresholdSettingsMaximalNumberOfPeaks(60)
            }
            ConfigV5::NumberOfCandidates(_) => ConfigV5::NumberOfCandidates(10),
            ConfigV5::ZodiacClusterCompounds(_) => ConfigV5::ZodiacClusterCompounds(false),
            ConfigV5::ZodiacEdgeFilterThresholdsMinLocalCandidates(_) => {
                ConfigV5::ZodiacEdgeFilterThresholdsMinLocalCandidates(1)
            }
            ConfigV5::ZodiacEdgeFilterThresholdsMinLocalConnections(_) => {
                ConfigV5::ZodiacEdgeFilterThresholdsMinLocalConnections(10)
            }
            ConfigV5::ZodiacEdgeFilterThresholdsThresholdFilter(_) => {
                ConfigV5::ZodiacEdgeFilterThresholdsThresholdFilter(0.95)
            }
            ConfigV5::ZodiacEpochsBurnInPeriod(_) => ConfigV5::ZodiacEpochsBurnInPeriod(2000),
            ConfigV5::ZodiacEpochsIterations(_) => ConfigV5::ZodiacEpochsIterations(20000),
            ConfigV5::ZodiacEpochsNumberOfMarkovChains(_) => {
                ConfigV5::ZodiacEpochsNumberOfMarkovChains(10)
            }
            ConfigV5::ZodiacLibraryScoringLambda(_) => ConfigV5::ZodiacLibraryScoringLambda(1000),
            ConfigV5::ZodiacLibraryScoringMinCosine(_) => {
                ConfigV5::ZodiacLibraryScoringMinCosine(0.5)
            }
            ConfigV5::ZodiacNumberOfConsideredCandidatesAt300Mz(_) => {
                ConfigV5::ZodiacNumberOfConsideredCandidatesAt300Mz(10)
            }
            ConfigV5::ZodiacNumberOfConsideredCandidatesAt800Mz(_) => {
                ConfigV5::ZodiacNumberOfConsideredCandidatesAt800Mz(50)
            }
            ConfigV5::ZodiacRatioOfConsideredCandidatesPerIonization(_) => {
                ConfigV5::ZodiacRatioOfConsideredCandidatesPerIonization(0.2)
            }
            ConfigV5::ZodiacRunInTwoSteps(_) => ConfigV5::ZodiacRunInTwoSteps(true),
            ConfigV5::MS1MassDeviationAllowedMassDeviation(_) => {
                ConfigV5::MS1MassDeviationAllowedMassDeviation(MassDeviation::ppm(10.0))
            }
            ConfigV5::MS1MassDeviationMassDifferenceDeviation(_) => {
                ConfigV5::MS1MassDeviationMassDifferenceDeviation(MassDeviation::ppm(5.0))
            }
            ConfigV5::MS1MassDeviationStandardMassDeviation(_) => {
                ConfigV5::MS1MassDeviationStandardMassDeviation(MassDeviation::ppm(10.0))
            }
            ConfigV5::MS2MassDeviationAllowedMassDeviation(_) => {
                ConfigV5::MS2MassDeviationAllowedMassDeviation(MassDeviation::ppm(10.0))
            }
            ConfigV5::MS2MassDeviationStandardMassDeviation(_) => {
                ConfigV5::MS2MassDeviationStandardMassDeviation(MassDeviation::ppm(10.0))
            }
            ConfigV5::FormulaSettingsDetectable(_) => {
                ConfigV5::FormulaSettingsDetectable(AtomVector::from(vec![
                    Atoms::S,
                    Atoms::Br,
                    Atoms::Cl,
                    Atoms::B,
                    Atoms::Se,
                ]))
            }
            ConfigV5::FormulaSettingsEnforced(_) => {
                ConfigV5::FormulaSettingsEnforced(AtomVector::from(vec![
                    Atoms::C,
                    Atoms::H,
                    Atoms::N,
                    Atoms::O,
                    Atoms::P,
                ]))
            }
            ConfigV5::FormulaSettingsFallback(_) => {
                ConfigV5::FormulaSettingsFallback(AtomVector::from(vec![Atoms::S]))
            }
            ConfigV5::ForbidRecalibration(_) => {
                ConfigV5::ForbidRecalibration(ForbidRecalibration::Allowed)
            }
            ConfigV5::UseHeuristicMZToUseHeuristic(_) => {
                ConfigV5::UseHeuristicMZToUseHeuristic(300)
            }
            ConfigV5::UseHeuristicMZToUseHeuristicOnly(_) => {
                ConfigV5::UseHeuristicMZToUseHeuristicOnly(650)
            }
            ConfigV5::AdductSettingsDetectable(_) => {
                ConfigV5::AdductSettingsDetectable(AdductsVector::from(vec![
                    Adducts::MplusHplus,
                    Adducts::MplusKplus,
                    Adducts::MplusNaplus,
                    Adducts::MplusHminusH2Oplus,
                    Adducts::MplusHminusTwoH2Oplus,
                    Adducts::MplusNH4plus,
                    Adducts::MminusHminus,
                    Adducts::MplusClminus,
                    Adducts::MminusH20minusHminus,
                    Adducts::MplusBromideminus,
                ]))
            }
            ConfigV5::AdductSettingsFallback(_) => {
                ConfigV5::AdductSettingsFallback(AdductsVector::from(vec![
                    Adducts::MplusHplus,
                    Adducts::MminusHminus,
                    Adducts::MplusKplus,
                    Adducts::MplusNaplus,
                ]))
            }
            ConfigV5::AlgorithmProfile(_) => ConfigV5::AlgorithmProfile(Instruments::Default),
            ConfigV5::CompoundQuality(_) => ConfigV5::CompoundQuality(CompoundQuality::Unknown),
            ConfigV5::AdductSettingsEnforced(_) => {
                ConfigV5::AdductSettingsEnforced(AdductSettingsEnforced::Comma)
            }
            ConfigV5::CandidateFormulas(_) => ConfigV5::CandidateFormulas(CandidateFormulas::Comma),
            ConfigV5::FormulaResultRankingScore(_) => {
                ConfigV5::FormulaResultRankingScore(FormulaResultRankingScore::Auto)
            }
            ConfigV5::IsotopeMS2Settings(_) => {
                ConfigV5::IsotopeMS2Settings(IsotopeMS2Settings::Ignore)
            }
            ConfigV5::IsotopeSettingsMultiplier(_) => ConfigV5::IsotopeSettingsMultiplier(1),
            ConfigV5::NoiseThresholdSettingsAbsoluteThreshold(_) => {
                ConfigV5::NoiseThresholdSettingsAbsoluteThreshold(0)
            }
            ConfigV5::NoiseThresholdSettingsBasePeak(_) => {
                ConfigV5::NoiseThresholdSettingsBasePeak(BasePeak::NotPrecursor)
            }
            ConfigV5::StructurePredictors(_) => {
                ConfigV5::StructurePredictors(StructurePredictors::CsiFingerId)
            }
            ConfigV5::PossibleAdductSwitches(_) => {
                ConfigV5::PossibleAdductSwitches(PossibleAdductSwitches::DefaultAdductsSwitches)
            }
        }
    }
}

impl Enablable for ConfigV5 {
    fn is_enabler(&self) -> bool {
        matches!(self, ConfigV5::Enabled)
    }

    fn enabler() -> Self {
        ConfigV5::Enabled
    }
}

impl NamedParametersSet for ConfigV5 {
    fn parameter_set_name() -> &'static str {
        "config"
    }
}

/// The possible config settings for SIRIUS 6.x
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigV6 {
    /// If the config is enabled
    Enabled,

    /// The isotope settings filter
    IsotopeSettingsFilter(bool),

    /// The formula search db
    FormulaSearchDB(SearchDB),

    /// The structure search db
    StructureSearchDB(SearchDB),

    /// The timeout seconds per tree
    TimeoutSecondsPerTree(u32),

    /// The number of candidates per ionization
    NumberOfCandidatesPerIonization(u32),

    /// The number of structure candidates
    NumberOfStructureCandidates(u32),

    /// Whether to recompute results
    RecomputeResults(bool),

    /// Whether to print citations
    PrintCitations(bool),

    /// The timeout seconds per instance
    TimeoutSecondsPerInstance(u32),

    /// Whether to use formula result threshold
    FormulaResultThreshold(bool),

    /// Fix the single molecular formula determined by El Gordo if a lipid class is detected.
    EnforceElGordoFormula(bool),

    /// The median noise intensity
    MedianNoiseIntensity(f32),

    /// Isotopic intensity absolute error (MS1)
    IsotopicIntensitySettingsAbsoluteIntensityError(f32),

    /// Isotopic intensity minimal intensity to consider (MS1)
    IsotopicIntensitySettingsMinimalIntensityToConsider(f32),

    /// Isotopic intensity relative error (MS1)
    IsotopicIntensitySettingsRelativeIntensityError(f32),

    /// The noise threshold settings intensity threshold
    NoiseThresholdSettingsIntensityThreshold(f32),

    /// The noise threshold settings maximal number of peaks
    NoiseThresholdSettingsMaximalNumberOfPeaks(u32),

    /// Whether to cluster compounds before running zodiac
    ZodiacClusterCompounds(bool),

    /// The zodiac edge filter thresholds min local candidates
    ZodiacEdgeFilterThresholdsMinLocalCandidates(u32),

    /// The zodiac edge filter thresholds min local connections
    ZodiacEdgeFilterThresholdsMinLocalConnections(u32),

    /// The zodiac edge filter thresholds threshold filter
    ZodiacEdgeFilterThresholdsThresholdFilter(f32),

    /// The zodiac epochs burn in period
    ZodiacEpochsBurnInPeriod(u32),

    /// The zodiac epochs iterations
    ZodiacEpochsIterations(u32),

    /// The number of markov chains for zodiac
    ZodiacEpochsNumberOfMarkovChains(u32),

    /// The zodiac library scoring lambda
    ZodiacLibraryScoringLambda(u32),

    /// The zodiac library scoring min similarity
    ZodiacLibraryScoringMinSimilarity(f32),

    /// The zodiac number of considered candidates at 300 mz
    ZodiacNumberOfConsideredCandidatesAt300Mz(i32),

    /// The zodiac number of considered candidates at 800 mz
    ZodiacNumberOfConsideredCandidatesAt800Mz(i32),

    /// The zodiac ratio of considered candidates per ionization
    ZodiacRatioOfConsideredCandidatesPerIonization(f32),

    /// Whether to run zodiac in two steps
    ZodiacRunInTwoSteps(bool),

    /// The MS1 mass deviation allowed mass deviation
    MS1MassDeviationAllowedMassDeviation(MassDeviation),

    /// The MS1 mass deviation mass difference deviation
    MS1MassDeviationMassDifferenceDeviation(MassDeviation),

    /// The MS1 mass deviation standard mass deviation
    MS1MassDeviationStandardMassDeviation(MassDeviation),

    /// The MS2 mass deviation allowed mass deviation
    MS2MassDeviationAllowedMassDeviation(MassDeviation),

    /// The MS2 mass deviation standard mass deviation
    MS2MassDeviationStandardMassDeviation(MassDeviation),

    /// The formula settings detectable
    FormulaSettingsDetectable(AtomVector),

    /// The formula settings enforced
    FormulaSettingsEnforced(AtomVector),

    /// The formula settings fallback
    FormulaSettingsFallback(AtomVector),

    /// Whether to forbid recalibration
    ForbidRecalibration(ForbidRecalibration),

    /// Minimum m/z to enable heuristic preprocessing
    UseHeuristicUseHeuristicAboveMz(u32),

    /// Minimum m/z to use only heuristic preprocessing
    UseHeuristicUseOnlyHeuristicAboveMz(u32),

    /// The detectable adducts
    AdductSettingsDetectable(AdductsVector),

    /// The fallback adducts
    AdductSettingsFallback(AdductsVector),

    /// The algorithm profile
    AlgorithmProfile(Instruments),

    /// The compound quality
    CompoundQuality(CompoundQuality),

    /// The enforced adducts
    AdductSettingsEnforced(AdductSettingsEnforced),

    /// The candidate formulas
    CandidateFormulas(CandidateFormulas),

    /// The formula result ranking score
    FormulaResultRankingScore(FormulaResultRankingScore),

    /// The isotope ms2 settings
    IsotopeMS2Settings(IsotopeMS2Settings),

    /// The isotope settings multiplier
    IsotopeSettingsMultiplier(u32),

    /// The noise threshold settings absolute threshold
    NoiseThresholdSettingsAbsoluteThreshold(u32),

    /// The noise threshold settings base peak
    NoiseThresholdSettingsBasePeak(BasePeak),

    /// The structure predictors
    StructurePredictors(StructurePredictors),

    /// The possible adduct switches
    PossibleAdductSwitches(PossibleAdductSwitches),
}

impl ToString for ConfigV6 {
    fn to_string(&self) -> String {
        match self {
            ConfigV6::Enabled => Self::parameter_set_name().to_string(),
            ConfigV6::IsotopeSettingsFilter(isotope_settings_filter) => {
                format!("--IsotopeSettings.filter={}", isotope_settings_filter)
            }
            ConfigV6::FormulaSearchDB(formula_search_db) => {
                format!("--FormulaSearchDB={}", formula_search_db)
            }
            ConfigV6::StructureSearchDB(structure_search_db) => {
                format!("--StructureSearchDB={}", structure_search_db)
            }
            ConfigV6::TimeoutSecondsPerTree(timeout_seconds_per_tree) => {
                format!("--Timeout.secondsPerTree={}", timeout_seconds_per_tree)
            }
            ConfigV6::NumberOfCandidatesPerIonization(number_of_candidates_per_ion) => {
                format!(
                    "--NumberOfCandidatesPerIonization={}",
                    number_of_candidates_per_ion
                )
            }
            ConfigV6::NumberOfStructureCandidates(number_of_structure_candidates) => {
                format!(
                    "--NumberOfStructureCandidates={}",
                    number_of_structure_candidates
                )
            }
            ConfigV6::RecomputeResults(recompute_results) => {
                format!("--RecomputeResults={}", recompute_results)
            }
            ConfigV6::PrintCitations(print_citations) => {
                format!("--PrintCitations={}", print_citations)
            }
            ConfigV6::TimeoutSecondsPerInstance(timeout_seconds_per_instance) => {
                format!(
                    "--Timeout.secondsPerInstance={}",
                    timeout_seconds_per_instance
                )
            }
            ConfigV6::FormulaResultThreshold(formula_result_threshold) => {
                format!("--FormulaResultThreshold={}", formula_result_threshold)
            }
            ConfigV6::EnforceElGordoFormula(enforce_el_gordo_formula) => {
                format!("--EnforceElGordoFormula={}", enforce_el_gordo_formula)
            }
            ConfigV6::MedianNoiseIntensity(median_noise_intensity) => {
                format!("--MedianNoiseIntensity={}", median_noise_intensity)
            }
            ConfigV6::IsotopicIntensitySettingsAbsoluteIntensityError(v) => {
                format!("--IsotopicIntensitySettings.absoluteIntensityError={}", v)
            }
            ConfigV6::IsotopicIntensitySettingsMinimalIntensityToConsider(v) => {
                format!("--IsotopicIntensitySettings.minimalIntensityToConsider={}", v)
            }
            ConfigV6::IsotopicIntensitySettingsRelativeIntensityError(v) => {
                format!("--IsotopicIntensitySettings.relativeIntensityError={}", v)
            }
            ConfigV6::NoiseThresholdSettingsIntensityThreshold(v) => format!(
                "--NoiseThresholdSettings.intensityThreshold={}",
                v
            ),
            ConfigV6::NoiseThresholdSettingsMaximalNumberOfPeaks(v) => format!(
                "--NoiseThresholdSettings.maximalNumberOfPeaks={}",
                v
            ),
            ConfigV6::ZodiacClusterCompounds(v) => {
                format!("--ZodiacClusterCompounds={}", v)
            }
            ConfigV6::ZodiacEdgeFilterThresholdsMinLocalCandidates(v) => format!(
                "--ZodiacEdgeFilterThresholds.minLocalCandidates={}",
                v
            ),
            ConfigV6::ZodiacEdgeFilterThresholdsMinLocalConnections(v) => format!(
                "--ZodiacEdgeFilterThresholds.minLocalConnections={}",
                v
            ),
            ConfigV6::ZodiacEdgeFilterThresholdsThresholdFilter(v) => format!(
                "--ZodiacEdgeFilterThresholds.thresholdFilter={}",
                v
            ),
            ConfigV6::ZodiacEpochsBurnInPeriod(v) => format!(
                "--ZodiacEpochs.burnInPeriod={}",
                v
            ),
            ConfigV6::ZodiacEpochsIterations(v) => {
                format!("--ZodiacEpochs.iterations={}", v)
            }
            ConfigV6::ZodiacEpochsNumberOfMarkovChains(v) => {
                format!("--ZodiacEpochs.numberOfMarkovChains={}", v)
            }
            ConfigV6::ZodiacLibraryScoringLambda(v) => format!(
                "--ZodiacLibraryScoring.lambda={}",
                v
            ),
            ConfigV6::ZodiacLibraryScoringMinSimilarity(v) => format!(
                "--ZodiacLibraryScoring.minSimilarity={}",
                v
            ),
            ConfigV6::ZodiacNumberOfConsideredCandidatesAt300Mz(v) => format!(
                "--ZodiacNumberOfConsideredCandidatesAt300Mz={}",
                v
            ),
            ConfigV6::ZodiacNumberOfConsideredCandidatesAt800Mz(v) => format!(
                "--ZodiacNumberOfConsideredCandidatesAt800Mz={}",
                v
            ),
            ConfigV6::ZodiacRatioOfConsideredCandidatesPerIonization(v) => format!(
                "--ZodiacRatioOfConsideredCandidatesPerIonization={}",
                v
            ),
            ConfigV6::ZodiacRunInTwoSteps(v) => {
                format!("--ZodiacRunInTwoSteps={}", v)
            }
            ConfigV6::MS1MassDeviationAllowedMassDeviation(v) => format!(
                "--MS1MassDeviation.allowedMassDeviation={}",
                v
            ),
            ConfigV6::MS1MassDeviationMassDifferenceDeviation(v) => format!(
                "--MS1MassDeviation.massDifferenceDeviation={}",
                v
            ),
            ConfigV6::MS1MassDeviationStandardMassDeviation(v) => format!(
                "--MS1MassDeviation.standardMassDeviation={}",
                v
            ),
            ConfigV6::MS2MassDeviationAllowedMassDeviation(v) => format!(
                "--MS2MassDeviation.allowedMassDeviation={}",
                v
            ),
            ConfigV6::MS2MassDeviationStandardMassDeviation(v) => format!(
                "--MS2MassDeviation.standardMassDeviation={}",
                v
            ),
            ConfigV6::FormulaSettingsDetectable(v) => format!(
                "--FormulaSettings.detectable={}",
                v
            ),
            ConfigV6::FormulaSettingsEnforced(v) => {
                format!("--FormulaSettings.enforced={}", v)
            }
            ConfigV6::FormulaSettingsFallback(v) => {
                format!("--FormulaSettings.fallback={}", v)
            }
            ConfigV6::ForbidRecalibration(v) => {
                format!("--ForbidRecalibration={}", v)
            }
            ConfigV6::UseHeuristicUseHeuristicAboveMz(v) => {
                format!("--UseHeuristic.useHeuristicAboveMz={}", v)
            }
            ConfigV6::UseHeuristicUseOnlyHeuristicAboveMz(v) => {
                format!("--UseHeuristic.useOnlyHeuristicAboveMz={}", v)
            }
            ConfigV6::AdductSettingsDetectable(v) => {
                format!("--AdductSettings.detectable={}", v)
            }
            ConfigV6::AdductSettingsFallback(v) => {
                format!("--AdductSettings.fallback={}", v)
            }
            ConfigV6::AlgorithmProfile(v) => {
                format!("--AlgorithmProfile={}", v)
            }
            ConfigV6::CompoundQuality(v) => {
                format!("--CompoundQuality={}", v)
            }
            ConfigV6::AdductSettingsEnforced(v) => {
                format!("--AdductSettings.enforced={}", v)
            }
            ConfigV6::CandidateFormulas(v) => {
                format!("--CandidateFormulas={}", v)
            }
            ConfigV6::FormulaResultRankingScore(v) => {
                format!("--FormulaResultRankingScore={}", v)
            }
            ConfigV6::IsotopeMS2Settings(v) => {
                format!("--IsotopeMs2Settings={}", v)
            }
            ConfigV6::IsotopeSettingsMultiplier(v) => {
                format!("--IsotopeSettings.multiplier={}", v)
            }
            ConfigV6::NoiseThresholdSettingsAbsoluteThreshold(v) => format!(
                "--NoiseThresholdSettings.absoluteThreshold={}",
                v
            ),
            ConfigV6::NoiseThresholdSettingsBasePeak(v) => {
                format!("--NoiseThresholdSettings.basePeak={}", v)
            }
            ConfigV6::StructurePredictors(v) => {
                format!("--StructurePredictors={}", v)
            }
            ConfigV6::PossibleAdductSwitches(v) => {
                format!("--PossibleAdductSwitches={}", v)
            }
        }
    }
}

impl IntoDefault for ConfigV6 {
    fn into_default(self) -> Self {
        match self {
            ConfigV6::Enabled => ConfigV6::Enabled,
            ConfigV6::IsotopeSettingsFilter(_) => ConfigV6::IsotopeSettingsFilter(true),
            ConfigV6::FormulaSearchDB(_) => ConfigV6::FormulaSearchDB(SearchDB::None),
            ConfigV6::StructureSearchDB(_) => ConfigV6::StructureSearchDB(SearchDB::Bio),
            ConfigV6::TimeoutSecondsPerTree(_) => ConfigV6::TimeoutSecondsPerTree(0),
            ConfigV6::NumberOfCandidatesPerIonization(_) => ConfigV6::NumberOfCandidatesPerIonization(1),
            ConfigV6::NumberOfStructureCandidates(_) => {
                ConfigV6::NumberOfStructureCandidates(10000)
            }
            ConfigV6::RecomputeResults(_) => ConfigV6::RecomputeResults(false),
            ConfigV6::PrintCitations(_) => ConfigV6::PrintCitations(true),
            ConfigV6::TimeoutSecondsPerInstance(_) => ConfigV6::TimeoutSecondsPerInstance(0),
            ConfigV6::FormulaResultThreshold(_) => ConfigV6::FormulaResultThreshold(true),
            ConfigV6::EnforceElGordoFormula(_) => ConfigV6::EnforceElGordoFormula(true),
            ConfigV6::MedianNoiseIntensity(_) => ConfigV6::MedianNoiseIntensity(0.015),
            ConfigV6::IsotopicIntensitySettingsAbsoluteIntensityError(_) => ConfigV6::IsotopicIntensitySettingsAbsoluteIntensityError(0.02),
            ConfigV6::IsotopicIntensitySettingsMinimalIntensityToConsider(_) => {
                ConfigV6::IsotopicIntensitySettingsMinimalIntensityToConsider(0.01)
            }
            ConfigV6::IsotopicIntensitySettingsRelativeIntensityError(_) => ConfigV6::IsotopicIntensitySettingsRelativeIntensityError(0.08),
            ConfigV6::NoiseThresholdSettingsIntensityThreshold(_) => {
                ConfigV6::NoiseThresholdSettingsIntensityThreshold(0.005)
            }
            ConfigV6::NoiseThresholdSettingsMaximalNumberOfPeaks(_) => {
                ConfigV6::NoiseThresholdSettingsMaximalNumberOfPeaks(60)
            }
            ConfigV6::ZodiacClusterCompounds(_) => ConfigV6::ZodiacClusterCompounds(false),
            ConfigV6::ZodiacEdgeFilterThresholdsMinLocalCandidates(_) => {
                ConfigV6::ZodiacEdgeFilterThresholdsMinLocalCandidates(1)
            }
            ConfigV6::ZodiacEdgeFilterThresholdsMinLocalConnections(_) => {
                ConfigV6::ZodiacEdgeFilterThresholdsMinLocalConnections(10)
            }
            ConfigV6::ZodiacEdgeFilterThresholdsThresholdFilter(_) => {
                ConfigV6::ZodiacEdgeFilterThresholdsThresholdFilter(0.95)
            }
            ConfigV6::ZodiacEpochsBurnInPeriod(_) => ConfigV6::ZodiacEpochsBurnInPeriod(2000),
            ConfigV6::ZodiacEpochsIterations(_) => ConfigV6::ZodiacEpochsIterations(20000),
            ConfigV6::ZodiacEpochsNumberOfMarkovChains(_) => {
                ConfigV6::ZodiacEpochsNumberOfMarkovChains(10)
            }
            ConfigV6::ZodiacLibraryScoringLambda(_) => ConfigV6::ZodiacLibraryScoringLambda(1000),
            ConfigV6::ZodiacLibraryScoringMinSimilarity(_) => {
                ConfigV6::ZodiacLibraryScoringMinSimilarity(0.5)
            }
            ConfigV6::ZodiacNumberOfConsideredCandidatesAt300Mz(_) => {
                ConfigV6::ZodiacNumberOfConsideredCandidatesAt300Mz(10)
            }
            ConfigV6::ZodiacNumberOfConsideredCandidatesAt800Mz(_) => {
                ConfigV6::ZodiacNumberOfConsideredCandidatesAt800Mz(50)
            }
            ConfigV6::ZodiacRatioOfConsideredCandidatesPerIonization(_) => {
                ConfigV6::ZodiacRatioOfConsideredCandidatesPerIonization(0.2)
            }
            ConfigV6::ZodiacRunInTwoSteps(_) => ConfigV6::ZodiacRunInTwoSteps(true),
            ConfigV6::MS1MassDeviationAllowedMassDeviation(_) => {
                ConfigV6::MS1MassDeviationAllowedMassDeviation(MassDeviation::ppm(10.0))
            }
            ConfigV6::MS1MassDeviationMassDifferenceDeviation(_) => {
                ConfigV6::MS1MassDeviationMassDifferenceDeviation(MassDeviation::ppm(5.0))
            }
            ConfigV6::MS1MassDeviationStandardMassDeviation(_) => {
                ConfigV6::MS1MassDeviationStandardMassDeviation(MassDeviation::ppm(10.0))
            }
            ConfigV6::MS2MassDeviationAllowedMassDeviation(_) => {
                ConfigV6::MS2MassDeviationAllowedMassDeviation(MassDeviation::ppm(10.0))
            }
            ConfigV6::MS2MassDeviationStandardMassDeviation(_) => {
                ConfigV6::MS2MassDeviationStandardMassDeviation(MassDeviation::ppm(10.0))
            }
            ConfigV6::FormulaSettingsDetectable(_) => {
                ConfigV6::FormulaSettingsDetectable(AtomVector::from(vec![
                    Atoms::S,
                    Atoms::Br,
                    Atoms::Cl,
                    Atoms::B,
                    Atoms::Se,
                ]))
            }
            ConfigV6::FormulaSettingsEnforced(_) => {
                ConfigV6::FormulaSettingsEnforced(AtomVector::from(vec![
                    Atoms::C,
                    Atoms::H,
                    Atoms::N,
                    Atoms::O,
                    Atoms::P,
                ]))
            }
            ConfigV6::FormulaSettingsFallback(_) => {
                ConfigV6::FormulaSettingsFallback(AtomVector::from(vec![Atoms::S]))
            }
            ConfigV6::ForbidRecalibration(_) => {
                ConfigV6::ForbidRecalibration(ForbidRecalibration::Allowed)
            }
            ConfigV6::UseHeuristicUseHeuristicAboveMz(_) => {
                ConfigV6::UseHeuristicUseHeuristicAboveMz(300)
            }
            ConfigV6::UseHeuristicUseOnlyHeuristicAboveMz(_) => {
                ConfigV6::UseHeuristicUseOnlyHeuristicAboveMz(650)
            }
            ConfigV6::AdductSettingsDetectable(_) => {
                ConfigV6::AdductSettingsDetectable(AdductsVector::from(vec![
                    Adducts::MplusHplus,
                    Adducts::MplusKplus,
                    Adducts::MplusNaplus,
                    Adducts::MplusHminusH2Oplus,
                    Adducts::MplusHminusTwoH2Oplus,
                    Adducts::MplusNH4plus,
                    Adducts::MminusHminus,
                    Adducts::MplusClminus,
                    Adducts::MminusH20minusHminus,
                    Adducts::MplusBromideminus,
                ]))
            }
            ConfigV6::AdductSettingsFallback(_) => {
                ConfigV6::AdductSettingsFallback(AdductsVector::from(vec![
                    Adducts::MplusHplus,
                    Adducts::MminusHminus,
                    Adducts::MplusKplus,
                    Adducts::MplusNaplus,
                ]))
            }
            ConfigV6::AlgorithmProfile(_) => ConfigV6::AlgorithmProfile(Instruments::Default),
            ConfigV6::CompoundQuality(_) => ConfigV6::CompoundQuality(CompoundQuality::Unknown),
            ConfigV6::AdductSettingsEnforced(_) => {
                ConfigV6::AdductSettingsEnforced(AdductSettingsEnforced::Comma)
            }
            ConfigV6::CandidateFormulas(_) => ConfigV6::CandidateFormulas(CandidateFormulas::Comma),
            ConfigV6::FormulaResultRankingScore(_) => {
                ConfigV6::FormulaResultRankingScore(FormulaResultRankingScore::Auto)
            }
            ConfigV6::IsotopeMS2Settings(_) => {
                ConfigV6::IsotopeMS2Settings(IsotopeMS2Settings::Ignore)
            }
            ConfigV6::IsotopeSettingsMultiplier(_) => ConfigV6::IsotopeSettingsMultiplier(1),
            ConfigV6::NoiseThresholdSettingsAbsoluteThreshold(_) => {
                ConfigV6::NoiseThresholdSettingsAbsoluteThreshold(0)
            }
            ConfigV6::NoiseThresholdSettingsBasePeak(_) => {
                ConfigV6::NoiseThresholdSettingsBasePeak(BasePeak::NotPrecursor)
            }
            ConfigV6::StructurePredictors(_) => {
                ConfigV6::StructurePredictors(StructurePredictors::CsiFingerId)
            }
            ConfigV6::PossibleAdductSwitches(_) => {
                ConfigV6::PossibleAdductSwitches(PossibleAdductSwitches::DefaultAdductsSwitches)
            }
        }
    }
}

impl Enablable for ConfigV6 {
    fn is_enabler(&self) -> bool {
        matches!(self, ConfigV6::Enabled)
    }

    fn enabler() -> Self {
        ConfigV6::Enabled
    }
}

impl NamedParametersSet for ConfigV6 {
    fn parameter_set_name() -> &'static str {
        "config"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_isotope_settings_filter() {
        assert_eq!(
            ConfigV5::IsotopeSettingsFilter(true),
            ConfigV5::IsotopeSettingsFilter(false).into_default()
        );
        assert_ne!(
            ConfigV5::IsotopeSettingsFilter(false),
            ConfigV5::IsotopeSettingsFilter(false).into_default()
        );
    }

    #[test]
    fn test_default_formula_search_db() {
        assert_eq!(
            ConfigV5::FormulaSearchDB(SearchDB::None),
            ConfigV5::FormulaSearchDB(SearchDB::Gnps).into_default()
        );
        assert_eq!(
            ConfigV5::FormulaSearchDB(SearchDB::default()),
            ConfigV5::FormulaSearchDB(SearchDB::None)
        );
        assert_ne!(
            ConfigV5::FormulaSearchDB(SearchDB::Gnps),
            ConfigV5::FormulaSearchDB(SearchDB::Gnps).into_default()
        );
    }

    #[test]
    fn test_formula_settings_detectable_to_string() {
        assert_eq!(
            "--FormulaSettings.detectable=S,Br,Cl,B,Se",
            ConfigV5::FormulaSettingsDetectable(AtomVector::from(vec![
                Atoms::S,
                Atoms::Br,
                Atoms::Cl,
                Atoms::B,
                Atoms::Se,
            ]))
            .to_string()
        );
    }

    #[test]
    fn test_default_settings_detectable() {
        assert_eq!(
            ConfigV5::FormulaSettingsDetectable(AtomVector::from(vec![
                Atoms::S,
                Atoms::Br,
                Atoms::Cl,
                Atoms::B,
                Atoms::Se,
            ])),
            ConfigV5::FormulaSettingsDetectable(AtomVector::from(vec![
                Atoms::S,
                Atoms::Br,
                Atoms::Cl,
                Atoms::B,
                Atoms::Fe,
            ]))
            .into_default()
        );
        assert_ne!(
            ConfigV5::FormulaSettingsDetectable(AtomVector::from(vec![
                Atoms::S,
                Atoms::Br,
                Atoms::Cl,
                Atoms::B,
                Atoms::Se,
            ])),
            ConfigV5::FormulaSettingsDetectable(AtomVector::from(vec![
                Atoms::S,
                Atoms::Br,
                Atoms::Cl,
                Atoms::B,
                Atoms::Fe,
            ]))
        );
    }
    #[test]
    fn test_adducts_settings_detectable_to_string() {
        assert_ne!(
            "--AdductSettings.detectable=H,K,Na,M-H2O+H,M+H-2H2O,NH4,M-H,Cl,M-H2O-H,M+Br",
            ConfigV5::AdductSettingsDetectable(AdductsVector::from(vec![
                Adducts::MplusH2OplusHplus,
                Adducts::MplusHplus,
                Adducts::MplusClminus,
                Adducts::MplusHminusH2Oplus,
                Adducts::MplusHminusTwoH2Oplus,
                Adducts::MplusNH4plus,
                Adducts::MminusHminus,
                Adducts::MplusClminus,
                Adducts::MminusH20minusHminus,
                Adducts::MplusBromideminus,
            ]))
            .to_string()
        );
        assert_eq!(
            "--AdductSettings.detectable=[M+H]+,[M+K]+,[M+Na]+",
            ConfigV5::AdductSettingsDetectable(AdductsVector::from(vec![
                Adducts::MplusHplus,
                Adducts::MplusKplus,
                Adducts::MplusNaplus,
            ]))
            .to_string()
        );
        assert_eq!(
            "--AdductSettings.detectable=[M+H]+,[M+K]+,[M+Na]+,[M+H-H2O]+,[M+H-H4O2]+,[M+NH4]+,[M-H]-,[M+Cl]-,[M-H2O-H]-,[M+Br]-",
            ConfigV5::AdductSettingsDetectable(AdductsVector::from(vec![Adducts::MplusHplus]))
                .into_default()
                .to_string()
        )
    }
    #[test]
    fn check_negative_mass_deviation_panics() {
        assert!(std::panic::catch_unwind(|| MassDeviation::ppm(-1.0)).is_err());
        assert!(std::panic::catch_unwind(|| MassDeviation::da(-1.0)).is_err());
    }

    #[test]
    fn test_v6_isotopic_intensity_to_string() {
        assert_eq!(
            "--IsotopicIntensitySettings.absoluteIntensityError=0.02",
            ConfigV6::IsotopicIntensitySettingsAbsoluteIntensityError(0.02).to_string()
        );
        assert_eq!(
            "--IsotopicIntensitySettings.minimalIntensityToConsider=0.01",
            ConfigV6::IsotopicIntensitySettingsMinimalIntensityToConsider(0.01).to_string()
        );
        assert_eq!(
            "--IsotopicIntensitySettings.relativeIntensityError=0.08",
            ConfigV6::IsotopicIntensitySettingsRelativeIntensityError(0.08).to_string()
        );
    }

    #[test]
    fn test_v6_candidates_per_ionization_to_string() {
        assert_eq!(
            "--NumberOfCandidatesPerIonization=3",
            ConfigV6::NumberOfCandidatesPerIonization(3).to_string()
        );
    }

    #[test]
    fn test_v6_heuristic_flags_to_string() {
        assert_eq!(
            "--UseHeuristic.useHeuristicAboveMz=300",
            ConfigV6::UseHeuristicUseHeuristicAboveMz(300).to_string()
        );
        assert_eq!(
            "--UseHeuristic.useOnlyHeuristicAboveMz=650",
            ConfigV6::UseHeuristicUseOnlyHeuristicAboveMz(650).to_string()
        );
    }
}
