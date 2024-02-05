use crate::prelude::*;
use crate::traits::{Enablable, IntoDefault, NamedParametersSet};

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigV5 {
    Enabled,
    IsotopeSettingsFilter(bool),
    FormulaSearchDB(FormulaSearchDB),
    StructureSearchDB(FormulaSearchDB),
    TimeoutSecondsPerTree(u32),
    NumberOfCandidates(u32),
    NumberOfCandidatesPerIon(u32), // can this be equal to zero ?
    NumberOfStructureCandidates(u32),
    RecomputeResults(bool),
    PrintCitations(bool),
    TimeoutSecondsPerInstance(u32),
    FormulaResultThreshold(bool),
    InjectElGordoCompounds(bool),
    MedianNoiseIntensity(f32),
    MS1AbsoluteIntensityError(f32),
    MS1MinimalIntensityToConsider(f32),
    MS1RelativeIntensityError(f32),
    NoiseThresholdSettingsIntensityThreshold(f32),
    NoiseThresholdSettingsMaximalNumberOfPeaks(u32),
    ZodiacClusterCompounds(bool),
    ZodiacEdgeFilterThresholdsMinLocalCandidates(u32),
    ZodiacEdgeFilterThresholdsMinLocalConnections(u32),
    ZodiacEdgeFilterThresholdsThresholdFilter(f32),
    ZodiacEpochsBurnInPeriod(u32),
    ZodiacEpochsIterations(u32),
    ZodiacEpochsNumberOfMarkovChains(u32),
    ZodiacLibraryScoringLambda(u32),
    ZodiacLibraryScoringMinCosine(f32),
    ZodiacNumberOfConsideredCandidatesAt300Mz(i32),
    ZodiacNumberOfConsideredCandidatesAt800Mz(i32),
    ZodiacRatioOfConsideredCandidatesPerIonization(f32), //can't be negative, higher than 1 or NaN
    ZodiacRunInTwoSteps(bool),
    MS1MassDeviationAllowedMassDeviation(MassDeviation),
    MS1MassDeviationMassDifferenceDeviation(MassDeviation),
    MS1MassDeviationStandardMassDeviation(MassDeviation),
    MS2MassDeviationAllowedMassDeviation(MassDeviation),
    MS2MassDeviationStandardMassDeviation(MassDeviation),
    FormulaSettingsDetectable(AtomVector),
    FormulaSettingsEnforced(AtomVector),
    FormulaSettingsFallback(AtomVector),
    ForbidRecalibration(ForbidRecalibration),
    UseHeuristicMZToUseHeuristic(u32),
    UseHeuristicMZToUseHeuristicOnly(u32),
    AdductSettingsDetectable(AdductsVector),
    AdductSettingsFallback(AdductsVector),
    AlgorithmProfile(Instruments),
    CompoundQuality(CompoundQuality),
    AdductSettingsEnforced(AdductSettingsEnforced),
    CandidateFormulas(CandidateFormulas),
    FormulaResultRankingScore(FormulaResultRankingScore),
    IsotopeMS2Settings(IsotopeMS2Settings),
    IsotopeSettingsMultiplier(u32),
    NoiseThresholdSettingsAbsoluteThreshold(u32),
    NoiseThresholdSettingsBasePeak(BasePeak),
    StructurePredictors(StructurePredictors),
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
            ConfigV5::FormulaSearchDB(_) => ConfigV5::FormulaSearchDB(FormulaSearchDB::default()),
            ConfigV5::StructureSearchDB(_) => ConfigV5::StructureSearchDB(FormulaSearchDB::Bio),
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
                ConfigV5::FormulaSettingsDetectable(AtomVector::new(vec![
                    Atoms::S,
                    Atoms::Br,
                    Atoms::Cl,
                    Atoms::B,
                    Atoms::Se,
                ]))
            }
            ConfigV5::FormulaSettingsEnforced(_) => {
                ConfigV5::FormulaSettingsEnforced(AtomVector::new(vec![
                    Atoms::C,
                    Atoms::H,
                    Atoms::N,
                    Atoms::O,
                    Atoms::P,
                ]))
            }
            ConfigV5::FormulaSettingsFallback(_) => {
                ConfigV5::FormulaSettingsFallback(AtomVector::new(vec![Atoms::S]))
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
                ConfigV5::AdductSettingsDetectable(AdductsVector::new(vec![
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
                ConfigV5::AdductSettingsFallback(AdductsVector::new(vec![
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
            ConfigV5::FormulaSearchDB(FormulaSearchDB::Bio),
            ConfigV5::FormulaSearchDB(FormulaSearchDB::Gnps).into_default()
        );
        assert_eq!(
            ConfigV5::FormulaSearchDB(FormulaSearchDB::default()),
            ConfigV5::FormulaSearchDB(FormulaSearchDB::Bio)
        );
        assert_ne!(
            ConfigV5::FormulaSearchDB(FormulaSearchDB::Gnps),
            ConfigV5::FormulaSearchDB(FormulaSearchDB::Gnps).into_default()
        );
    }

    #[test]
    fn test_formula_settings_detectable_to_string() {
        assert_eq!(
            "--FormulaSettings.detectable=S,Br,Cl,B,Se",
            ConfigV5::FormulaSettingsDetectable(AtomVector::new(vec![
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
            ConfigV5::FormulaSettingsDetectable(AtomVector::new(vec![
                Atoms::S,
                Atoms::Br,
                Atoms::Cl,
                Atoms::B,
                Atoms::Se,
            ])),
            ConfigV5::FormulaSettingsDetectable(AtomVector::new(vec![
                Atoms::S,
                Atoms::Br,
                Atoms::Cl,
                Atoms::B,
                Atoms::Fe,
            ]))
            .into_default()
        );
        assert_ne!(
            ConfigV5::FormulaSettingsDetectable(AtomVector::new(vec![
                Atoms::S,
                Atoms::Br,
                Atoms::Cl,
                Atoms::B,
                Atoms::Se,
            ])),
            ConfigV5::FormulaSettingsDetectable(AtomVector::new(vec![
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
            ConfigV5::AdductSettingsDetectable(AdductsVector::new(vec![
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
            ConfigV5::AdductSettingsDetectable(AdductsVector::new(vec![
                Adducts::MplusHplus,
                Adducts::MplusKplus,
                Adducts::MplusNaplus,
            ]))
            .to_string()
        );
        assert_eq!(
            "--AdductSettings.detectable=[M+H]+,[M+K]+,[M+Na]+,[M+H-H2O]+,[M+H-H4O2]+,[M+NH4]+,[M-H]-,[M+Cl]-,[M-H2O-H]-,[M+Br]-",
            ConfigV5::AdductSettingsDetectable(AdductsVector::new(vec![Adducts::MplusHplus]))
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
