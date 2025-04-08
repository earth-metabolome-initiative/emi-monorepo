use std::fmt;

use crate::{
    prelude::*,
    traits::{Enablable, IntoDefault, NamedParametersSet},
};

/// The possible config settings
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigV5 {
    /// If the config is enabled
    Enabled,

    /// This configurations define how to deal with isotope patterns in MS1.
    /// When filtering is enabled, molecular formulas are excluded if their
    /// theoretical isotope pattern does not match the theoretical one, even
    /// if their MS/MS pattern has high score.
    IsotopeSettingsFilter(bool),

    /// The database from which to search formulas. This can consist of one or
    /// more search databases.
    FormulaSearchDB(DBVector),

    /// The database from which to search structures. This can consist of one or
    /// more search databases.
    StructureSearchDB(DBVector),

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

    /// Specifies if the list of Molecular Formula Identifications is filtered
    /// by a soft threshold (calculateThreshold) before CSI:FingerID
    /// predictions are calculated.
    FormulaResultThreshold(bool),

    /// Candidates matching the lipid class estimated by El Gordo will be
    /// tagged. The lipid class will only be available if El Gordo predicts
    /// that the MS/MS is a lipid spectrum.
    ///
    /// If this parameter is set to 'false' El Gordo will still be executed and
    /// e.g. improve the fragmentation tree, but the matching candidates
    /// will not be tagged as lipid class.
    ///
    /// Default: `true`
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
    ZodiacRatioOfConsideredCandidatesPerIonization(f32), // can't be negative, higher than 1 or
    // NaN
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

    /// Detectable elements are added to the chemical alphabet, if there are
    /// indications for them (e.g. in isotope pattern)
    ///
    /// Default: `S,Br,Cl,B,Se`
    FormulaSettingsDetectable(AtomVector),

    /// These configurations hold the information how to autodetect elements
    /// based on the given formula constraints.
    /// Note: If the compound is already assigned to a specific molecular
    /// formula, this annotation is ignored.
    ///
    /// Enforced elements are always considered.
    ///
    /// Default: `C,H,N,O,P`
    FormulaSettingsEnforced(AtomVector),

    /// Fallback elements are used, if the auto-detection fails (e.g. no isotope
    /// pattern available)
    ///
    /// Default: `S`
    FormulaSettingsFallback(AtomVector),

    /// Enable/Disable the hypothesen driven recalibration of MS/MS spectra.
    /// Must be either `ALLOWED` or `FORBIDDEN`
    ///
    /// Default: `ALLOWED`
    ForbidRecalibration(ForbidRecalibration),

    /// The use heuristic mz to use heuristic
    UseHeuristicMZToUseHeuristic(u32),

    /// The use heuristic mz to use heuristic only
    UseHeuristicMZToUseHeuristicOnly(u32),

    ///  Detectable ion modes which are only considered if there is an
    /// indication in the MS1 scan (e.g. correct mass delta).
    ///
    /// The default is
    /// `[M+H]+,[M+K]+,[M+Na]+,[M+H-H2O]+,[M+H-H4O2]+,[M+NH4]+,[M-H]-,[M+Cl]-,
    /// [M-H2O-H]-,[M+Br]-`.
    AdductSettingsDetectable(AdductsVector),

    /// Fallback ion modes which are considered if the auto
    /// detection did not find any indication for an ion mode.
    /// ATTENTION: Expanding adducts from ionizations (e.g. `[M+H]+` to
    /// `[M+H-H2O]+``) does not respect databases that were selected in the
    /// formulas annotation step.
    AdductSettingsFallback(AdductsVector),

    /// Configuration profile to store instrument specific algorithm properties.
    /// Some of the default profiles are: 'qtof', 'orbitrap', 'fticr'.
    /// Default: `default`
    AlgorithmProfile(Instruments),

    /// Keywords that can be assigned to a input spectrum to judge its quality.
    /// Available keywords are: Good, LowIntensity, NoMS1Peak, FewPeaks,
    /// Chimeric, NotMonoisotopicPeak, PoorlyExplained
    ///
    /// Default: `UNKNOWN`
    CompoundQuality(CompoundQuality),

    /// Describes how to deal with Adducts:
    ///
    /// Pos Examples:
    /// `[M+H]+,[M]+,[M+K]+,[M+Na]+,[M+H-H2O]+,[M+Na2-H]+,[M+2K-H]+,[M+NH4]+,
    /// [M+H3O]+,[M+MeOH+H]+,[M+ACN+H]+,[M+2ACN+H]+,[M+IPA+H]+,[M+ACN+Na]+,
    /// [M+DMSO+H]+`
    ///
    /// Neg Examples:
    /// `[M-H]-,[M]-,[M+K-2H]-,[M+Cl]-,[M-H2O-H]-,[M+Na-2H]-,[M+FA-H]-,[M+Br]-,
    /// [M+HAc-H]-,[M+TFA-H]-,[M+ACN-H]-`
    ///
    /// Default: `,`
    ///
    /// Enforced ion modes that are always considered.
    AdductSettingsEnforced(AdductSettingsEnforced),

    ///  This configuration holds a set of user given formulas to be used as
    /// candidates for SIRIUS. Note: This set might be merged with other
    /// sources like formulas from databases Set of Molecular Formulas to be
    /// used as candidates for molecular formula estimation with SIRIUS
    ///
    /// Currently only the default value is supported.
    ///
    /// Default: `,`
    CandidateFormulas(CandidateFormulas),

    /// Allows the USER to Specify the ScoreType that is used to rank the list
    /// of Molecular Formula Identifications before CSI:FingerID predictions
    /// are calculated. Auto means that this ScoreType is automatically set
    /// depending on the executed workflow.
    ///
    /// Currently only the default value is supported.
    ///
    /// Default: `AUTO`
    FormulaResultRankingScore(FormulaResultRankingScore),

    /// The isotope ms2 settings
    IsotopeMS2Settings(IsotopeMS2Settings),

    ///   multiplier for the isotope score. Set to 0 to disable isotope scoring.
    /// Otherwise, the score from isotope pattern analysis is multiplied
    /// with this coefficient. Set to a value larger than one if
    ///  your isotope pattern data is of much better quality than your MS/MS
    /// data.
    ///
    /// Default: `1`
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

impl fmt::Display for ConfigV5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigV5::Enabled => write!(f, "{}", Self::parameter_set_name()),
            ConfigV5::IsotopeSettingsFilter(isotope_settings_filter) => {
                write!(f, "--IsotopeSettings.filter={isotope_settings_filter}")
            }
            ConfigV5::FormulaSearchDB(formula_search_db) => {
                write!(f, "--FormulaSearchDB={formula_search_db}")
            }
            ConfigV5::StructureSearchDB(structure_search_db) => {
                write!(f, "--StructureSearchDB={structure_search_db}")
            }
            ConfigV5::TimeoutSecondsPerTree(timeout_seconds_per_tree) => {
                write!(f, "--Timeout.secondsPerTree={timeout_seconds_per_tree}")
            }
            ConfigV5::NumberOfCandidatesPerIon(number_of_candidates_per_ion) => {
                write!(f, "--NumberOfCandidatesPerIon={number_of_candidates_per_ion}")
            }
            ConfigV5::NumberOfStructureCandidates(number_of_structure_candidates) => {
                write!(f, "--NumberOfStructureCandidates={number_of_structure_candidates}")
            }
            ConfigV5::RecomputeResults(recompute_results) => {
                write!(f, "--RecomputeResults={recompute_results}")
            }
            ConfigV5::PrintCitations(print_citations) => {
                write!(f, "--PrintCitations={print_citations}")
            }
            ConfigV5::TimeoutSecondsPerInstance(timeout_seconds_per_instance) => {
                write!(f, "--Timeout.secondsPerInstance={timeout_seconds_per_instance}")
            }
            ConfigV5::FormulaResultThreshold(formula_result_threshold) => {
                write!(f, "--FormulaResultThreshold={formula_result_threshold}")
            }
            ConfigV5::InjectElGordoCompounds(inject_el_gordo_compounds) => {
                write!(f, "--InjectElGordoCompounds={inject_el_gordo_compounds}")
            }
            ConfigV5::MedianNoiseIntensity(median_noise_intensity) => {
                write!(f, "--MedianNoiseIntensity={median_noise_intensity}")
            }
            ConfigV5::MS1AbsoluteIntensityError(ms1_absolute_intensity_error) => {
                write!(f, "--ms1.absoluteIntensityError={ms1_absolute_intensity_error}")
            }
            ConfigV5::MS1MinimalIntensityToConsider(ms1_minimal_intensity_to_consider) => {
                write!(f, "--ms1.minimalIntensityToConsider={ms1_minimal_intensity_to_consider}")
            }
            ConfigV5::MS1RelativeIntensityError(ms1_relative_intensity_error) => {
                write!(f, "--ms1.relativeIntensityError={ms1_relative_intensity_error}")
            }
            ConfigV5::NoiseThresholdSettingsIntensityThreshold(
                noise_threshold_settings_intensity_threshold,
            ) => {
                write!(
                    f,
                    "--NoiseThresholdSettings.intensityThreshold={noise_threshold_settings_intensity_threshold}"
                )
            }
            ConfigV5::NoiseThresholdSettingsMaximalNumberOfPeaks(
                noise_threshold_settings_maximal_number_of_peaks,
            ) => {
                write!(
                    f,
                    "--NoiseThresholdSettings.maximalNumberOfPeaks={noise_threshold_settings_maximal_number_of_peaks}"
                )
            }
            ConfigV5::NumberOfCandidates(number_of_candidates) => {
                write!(f, "--NumberOfCandidates={number_of_candidates}")
            }
            ConfigV5::ZodiacClusterCompounds(zodiac_cluster_compounds) => {
                write!(f, "--ZodiacClusterCompounds={zodiac_cluster_compounds}")
            }
            ConfigV5::ZodiacEdgeFilterThresholdsMinLocalCandidates(
                zodiac_edge_filter_thresholds_min_local_candidates,
            ) => {
                write!(
                    f,
                    "--ZodiacEdgeFilterThresholds.minLocalCandidates={zodiac_edge_filter_thresholds_min_local_candidates}"
                )
            }
            ConfigV5::ZodiacEdgeFilterThresholdsMinLocalConnections(
                zodiac_edge_filter_thresholds_min_local_connections,
            ) => {
                write!(
                    f,
                    "--ZodiacEdgeFilterThresholds.minLocalConnections={zodiac_edge_filter_thresholds_min_local_connections}"
                )
            }
            ConfigV5::ZodiacEdgeFilterThresholdsThresholdFilter(
                zodiac_edge_filter_thresholds_threshold_filter,
            ) => {
                write!(
                    f,
                    "--ZodiacEdgeFilterThresholds.thresholdFilter={zodiac_edge_filter_thresholds_threshold_filter}"
                )
            }
            ConfigV5::ZodiacEpochsBurnInPeriod(zodiac_epochs_burn_in_period) => {
                write!(f, "--ZodiacEpochs.burnInPeriod={zodiac_epochs_burn_in_period}")
            }
            ConfigV5::ZodiacEpochsIterations(zodiac_epochs_iterations) => {
                write!(f, "--ZodiacEpochs.iterations={zodiac_epochs_iterations}")
            }
            ConfigV5::ZodiacEpochsNumberOfMarkovChains(zodiac_epochs_number_of_markov_chains) => {
                write!(
                    f,
                    "--ZodiacEpochs.numberOfMarkovChains={zodiac_epochs_number_of_markov_chains}"
                )
            }
            ConfigV5::ZodiacLibraryScoringLambda(zodiac_library_scoring_lambda) => {
                write!(f, "--ZodiacLibraryScoring.lambda={zodiac_library_scoring_lambda}")
            }
            ConfigV5::ZodiacLibraryScoringMinCosine(zodiac_library_scoring_min_cosine) => {
                write!(f, "--ZodiacLibraryScoring.minCosine={zodiac_library_scoring_min_cosine}")
            }
            ConfigV5::ZodiacNumberOfConsideredCandidatesAt300Mz(
                zodiac_number_of_considered_candidates_at_300_mz,
            ) => {
                write!(
                    f,
                    "--ZodiacNumberOfConsideredCandidatesAt300Mz={zodiac_number_of_considered_candidates_at_300_mz}"
                )
            }
            ConfigV5::ZodiacNumberOfConsideredCandidatesAt800Mz(
                zodiac_number_of_considered_candidates_at_800_mz,
            ) => {
                write!(
                    f,
                    "--ZodiacNumberOfConsideredCandidatesAt800Mz={zodiac_number_of_considered_candidates_at_800_mz}"
                )
            }
            ConfigV5::ZodiacRatioOfConsideredCandidatesPerIonization(
                zodiac_ratio_of_considered_candidates_per_ionization,
            ) => {
                write!(
                    f,
                    "--ZodiacRatioOfConsideredCandidatesPerIonization={zodiac_ratio_of_considered_candidates_per_ionization}"
                )
            }
            ConfigV5::ZodiacRunInTwoSteps(zodiac_run_in_two_steps) => {
                write!(f, "--ZodiacRunInTwoSteps={zodiac_run_in_two_steps}")
            }
            ConfigV5::MS1MassDeviationAllowedMassDeviation(
                ms1_mass_deviation_allowed_mass_deviation,
            ) => {
                write!(
                    f,
                    "--MS1MassDeviation.allowedMassDeviation={ms1_mass_deviation_allowed_mass_deviation}"
                )
            }
            ConfigV5::MS1MassDeviationMassDifferenceDeviation(
                ms1_mass_deviation_mass_difference_deviation,
            ) => {
                write!(
                    f,
                    "--MS1MassDeviation.massDifferenceDeviation={ms1_mass_deviation_mass_difference_deviation}"
                )
            }
            ConfigV5::MS1MassDeviationStandardMassDeviation(
                ms1_mass_deviation_standard_mass_deviation,
            ) => {
                write!(
                    f,
                    "--MS1MassDeviation.standardMassDeviation={ms1_mass_deviation_standard_mass_deviation}"
                )
            }
            ConfigV5::MS2MassDeviationAllowedMassDeviation(
                ms2_mass_deviation_allowed_mass_deviation,
            ) => {
                write!(
                    f,
                    "--MS2MassDeviation.allowedMassDeviation={ms2_mass_deviation_allowed_mass_deviation}"
                )
            }
            ConfigV5::MS2MassDeviationStandardMassDeviation(
                ms2_mass_deviation_standard_mass_deviation,
            ) => {
                write!(
                    f,
                    "--MS2MassDeviation.standardMassDeviation={ms2_mass_deviation_standard_mass_deviation}"
                )
            }
            ConfigV5::FormulaSettingsDetectable(formula_settings_detectable) => {
                write!(f, "--FormulaSettings.detectable={formula_settings_detectable}")
            }
            ConfigV5::FormulaSettingsEnforced(formula_settings_enforced) => {
                write!(f, "--FormulaSettings.enforced={formula_settings_enforced}")
            }
            ConfigV5::FormulaSettingsFallback(formula_settings_fallback) => {
                write!(f, "--FormulaSettings.fallback={formula_settings_fallback}")
            }
            ConfigV5::ForbidRecalibration(forbid_recalibration) => {
                write!(f, "--ForbidRecalibration={forbid_recalibration}")
            }
            ConfigV5::UseHeuristicMZToUseHeuristic(use_heuristic_mz_to_use_heuristic) => {
                write!(f, "--UseHeuristic.mzToUseHeuristic={use_heuristic_mz_to_use_heuristic}")
            }
            ConfigV5::UseHeuristicMZToUseHeuristicOnly(use_heuristic_mz_to_use_heuristic_only) => {
                write!(
                    f,
                    "--UseHeuristic.mzToUseHeuristicOnly={use_heuristic_mz_to_use_heuristic_only}"
                )
            }
            ConfigV5::AdductSettingsDetectable(adduct_settings_detectable) => {
                write!(f, "--AdductSettings.detectable={adduct_settings_detectable}")
            }
            ConfigV5::AdductSettingsFallback(adduct_settings_fallback) => {
                write!(f, "--AdductSettings.fallback={adduct_settings_fallback}")
            }
            ConfigV5::AlgorithmProfile(algorithm_profile) => {
                write!(f, "--AlgorithmProfile={algorithm_profile}")
            }
            ConfigV5::CompoundQuality(compound_quality) => {
                write!(f, "--CompoundQuality={compound_quality}")
            }
            ConfigV5::AdductSettingsEnforced(adduct_settings_enforced) => {
                write!(f, "--AdductSettings.enforced={adduct_settings_enforced}")
            }
            ConfigV5::CandidateFormulas(candidate_formulas) => {
                write!(f, "--CandidateFormulas={candidate_formulas}")
            }
            ConfigV5::FormulaResultRankingScore(formula_result_ranking_score) => {
                write!(f, "--FormulaResultRankingScore={formula_result_ranking_score}")
            }
            ConfigV5::IsotopeMS2Settings(isotope_ms2_settings) => {
                write!(f, "--IsotopeMs2Settings={isotope_ms2_settings}")
            }
            ConfigV5::IsotopeSettingsMultiplier(isotope_settings_multiplier) => {
                write!(f, "--IsotopeSettings.multiplier={isotope_settings_multiplier}")
            }
            ConfigV5::NoiseThresholdSettingsAbsoluteThreshold(
                noise_threshold_settings_absolute_threshold,
            ) => {
                write!(
                    f,
                    "--NoiseThresholdSettings.absoluteThreshold={noise_threshold_settings_absolute_threshold}"
                )
            }
            ConfigV5::NoiseThresholdSettingsBasePeak(noise_threshold_settings_base_peak) => {
                write!(f, "--NoiseThresholdSettings.basePeak={noise_threshold_settings_base_peak}")
            }
            ConfigV5::StructurePredictors(structure_predictors) => {
                write!(f, "--StructurePredictors={structure_predictors}")
            }
            ConfigV5::PossibleAdductSwitches(possible_adduct_switches) => {
                write!(f, "--PossibleAdductSwitches={possible_adduct_switches}")
            }
        }
    }
}

impl IntoDefault for ConfigV5 {
    fn into_default(self) -> Self {
        match self {
            ConfigV5::Enabled => ConfigV5::Enabled,
            ConfigV5::IsotopeSettingsFilter(_) => ConfigV5::IsotopeSettingsFilter(true),
            ConfigV5::FormulaSearchDB(_) => {
                ConfigV5::FormulaSearchDB(DBVector::from(vec![SearchDB::None]))
            }
            ConfigV5::StructureSearchDB(_) => {
                ConfigV5::StructureSearchDB(DBVector::from(vec![SearchDB::Bio]))
            }
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
            ConfigV5::FormulaSearchDB(DBVector::from(vec![SearchDB::None])),
            ConfigV5::FormulaSearchDB(DBVector::from(vec![SearchDB::Gnps])).into_default()
        );
        assert_eq!(
            ConfigV5::FormulaSearchDB(DBVector::from(vec![SearchDB::default()])),
            ConfigV5::FormulaSearchDB(DBVector::from(vec![SearchDB::None]))
        );
        assert_ne!(
            ConfigV5::FormulaSearchDB(DBVector::from(vec![SearchDB::Gnps])),
            ConfigV5::FormulaSearchDB(DBVector::from(vec![SearchDB::Gnps])).into_default()
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
}
