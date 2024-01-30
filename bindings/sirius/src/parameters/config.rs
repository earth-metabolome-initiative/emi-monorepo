// this file sets all the parameters of the `sirius config` command
// TODO verify if some parameters must be float, int or unsigned integers

use crate::prelude::*;
use crate::traits::{Enablable, IntoDefault, NamedParametersSet};

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigV5 {
    Enabled,
    IsotopeSettingsFilter(bool),
    FormulaSearchDB(FormulaSearchDB),
    TimeoutSecondsPerTree(u32),
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
    NumberOfCandidates(u32),
    ZodiacClusterCompounds(bool),
    ZodiacEdgeFilterThresholdsMinLocalCandidates(u32),
    ZodiacEdgeFilterThresholdsMinLocalConnections(u32),
    ZodiacEdgeFilterThresholdsThresholdFilter(f32),
    ZodiacEpochsBurnInPeriod(u32),
    ZodiacEpochsIterations(u32),
    ZodiacEpochsNumberOfMarkovChains(u32),
    ZodiacLibraryScoringLambda(u32),
    ZodiacLibraryScoringMinCosine(f32),
    ZodiacNumberOfConsideredCandidatesAt300Mz(u32),
    ZodiacNumberOfConsideredCandidatesAt800Mz(u32),
    ZodiacRatioOfConsideredCandidatesPerIonization(f32), //can't be negative, higher than 1 or NaN
    ZodiacRunInTwoSteps(bool),
    MS1MassDeviationAllowedMassDeviation(MassDeviation),
    MS1MassDeviationMassDifferenceDeviation(MassDeviation),
    MS1MassDeviationStandardMassDeviation(MassDeviation),
    MS2MassDeviationAllowedMassDeviation(MassDeviation),
    MS2MassDeviationStandardMassDeviation(MassDeviation),
    FormulaSettingsDetectable(AtomVector),
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
        }
    }
}

impl IntoDefault for ConfigV5 {
    fn into_default(self) -> Self {
        match self {
            ConfigV5::Enabled => ConfigV5::Enabled,
            ConfigV5::IsotopeSettingsFilter(_) => ConfigV5::IsotopeSettingsFilter(true),
            ConfigV5::FormulaSearchDB(_) => ConfigV5::FormulaSearchDB(FormulaSearchDB::Bio),
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
                ConfigV5::MS1MassDeviationAllowedMassDeviation(MassDeviation::Ppm(10.0))
            }
            ConfigV5::MS1MassDeviationMassDifferenceDeviation(_) => {
                ConfigV5::MS1MassDeviationMassDifferenceDeviation(MassDeviation::Ppm(5.0))
            }
            ConfigV5::MS1MassDeviationStandardMassDeviation(_) => {
                ConfigV5::MS1MassDeviationStandardMassDeviation(MassDeviation::Ppm(10.0))
            }
            ConfigV5::MS2MassDeviationAllowedMassDeviation(_) => {
                ConfigV5::MS2MassDeviationAllowedMassDeviation(MassDeviation::Ppm(10.0))
            }
            ConfigV5::MS2MassDeviationStandardMassDeviation(_) => {
                ConfigV5::MS2MassDeviationStandardMassDeviation(MassDeviation::Ppm(10.0))
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
        }
    }
}

impl Enablable for ConfigV5 {
    fn is_enabler(&self) -> bool {
        match self {
            ConfigV5::Enabled => true,
            _ => false,
        }
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
}
