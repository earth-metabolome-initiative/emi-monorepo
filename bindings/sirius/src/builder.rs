//! A builder is a type of struct that will collect configurations and once build, prodiuces a complete struct.
//!
use crate::prelude::*;
use crate::sirius_config::SiriusConfig;
use crate::traits::IntoDefault;

#[derive(Default)]
pub struct SiriusBuilder<V: Version> {
    config: SiriusConfig<V>,
}

impl SiriusBuilder<Version5> {
    /// Set the maximal value of m/z ratio on which Sirius calculation will be carried.
    ///
    /// # Arguments
    ///
    /// * `maximal_mz` - The maximal m/z ratio.
    ///
    /// # Example
    ///
    /// ```
    /// use sirius::prelude::*;
    ///
    /// let sirius = SiriusBuilder::default()
    ///    .maximal_mz(1000.0).unwrap()
    ///   .build();
    ///
    /// assert!(SiriusBuilder::default().maximal_mz(-67.0).is_err());
    /// assert!(SiriusBuilder::default().maximal_mz(0.0).is_err());
    /// assert!(SiriusBuilder::default().maximal_mz(std::f64::NAN).is_err());
    /// assert!(SiriusBuilder::default().maximal_mz(std::f64::INFINITY).is_err());
    /// ```
    ///
    pub fn maximal_mz(mut self, maximal_mz: f64) -> Result<Self, String> {
        if maximal_mz < 0.0 {
            return Err(format!(
                concat!("Maximal m/z ratio must be positive. ", "You provided {}."),
                maximal_mz
            ));
        }
        if maximal_mz == 0.0 {
            return Err("Maximal m/z ratio cannot be 0".to_string());
        }
        if maximal_mz.is_nan() {
            return Err("Maximal m/z ratio cannot be NaN".to_string());
        }
        if maximal_mz.is_infinite() {
            return Err("Maximal m/z ratio cannot be infinite".to_string());
        }

        self.config
            .add_core_parameter(CoreV5::MaximalMz(maximal_mz))?;
        Ok(self)
    }

    /// Activate the use of the isotope settings filter.
    /// # Arguments
    /// * `isotope_settings_filter` - Whether to enable the isotope settings filter.
    ///
    /// # Example
    /// ```
    /// use sirius::prelude::*;
    /// let sirius = SiriusBuilder::default()
    ///   .isotope_settings_filter(true).unwrap()
    ///   .build();
    /// ```
    pub fn isotope_settings_filter(
        mut self,
        isotope_settings_filter: bool,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::IsotopeSettingsFilter(isotope_settings_filter))?;
        Ok(self)
    }

    /// Set the database to be used for formula search.
    /// # Arguments
    /// * `formula_search_db` - The database to be used for formula search.
    /// # Example
    /// ```
    /// use sirius::prelude::*;
    /// let sirius = SiriusBuilder::default()
    ///  .formula_search_db(FormulaSearchDB::Hmdb).unwrap()
    /// .build();
    ///
    /// assert!(SiriusBuilder::default().formula_search_db(FormulaSearchDB::Hmdb).is_ok());
    /// ```
    pub fn formula_search_db(
        mut self,
        formula_search_db: crate::sirius_types::FormulaSearchDB,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::FormulaSearchDB(formula_search_db))?;
        Ok(self)
    }

    /// Set the database to be used for the structure search.
    /// # Arguments
    /// * `structure_search_db` - The database to be used for the structure search.
    /// # Example
    /// ```
    /// use sirius::prelude::*;
    /// let sirius = SiriusBuilder::default()
    /// .structure_search_db(FormulaSearchDB::Zincbio).unwrap()
    /// .build();
    /// ```
    pub fn structure_search_db(
        mut self,
        structure_search_db: FormulaSearchDB,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::StructureSearchDB(structure_search_db))?;
        Ok(self)
    }

    /// Set the timeout seconds for each tree.
    /// # Arguments
    /// * `timeout_seconds_per_tree` - The timeout seconds for each tree.
    /// # Example
    /// ```
    /// use sirius::prelude::*;
    /// let sirius = SiriusBuilder::default()
    /// .timeout_seconds_per_tree(100).unwrap()
    /// .build();
    /// ```
    pub fn timeout_seconds_per_tree(
        mut self,
        timeout_seconds_per_tree: u32,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::TimeoutSecondsPerTree(timeout_seconds_per_tree))?;
        Ok(self)
    }

    pub fn number_of_candidates(mut self, number_of_candidates: u32) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::NumberOfCandidates(number_of_candidates))?;
        Ok(self)
    }

    pub fn number_of_candidates_per_ion(
        mut self,
        number_of_candidates_per_ion: u32,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::NumberOfCandidatesPerIon(
                number_of_candidates_per_ion,
            ))?;
        Ok(self)
    }

    pub fn number_of_structure_candidates(
        mut self,
        number_of_structure_candidates: u32,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::NumberOfStructureCandidates(
                number_of_structure_candidates,
            ))?;
        Ok(self)
    }

    pub fn recompute_results(mut self, recompute_results: bool) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::RecomputeResults(recompute_results))?;
        Ok(self)
    }

    pub fn print_citations(mut self, print_citations: bool) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::PrintCitations(print_citations))?;
        Ok(self)
    }

    pub fn timeout_seconds_per_instance(
        mut self,
        timeout_seconds_per_instance: u32,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::TimeoutSecondsPerInstance(
                timeout_seconds_per_instance,
            ))?;
        Ok(self)
    }

    pub fn formula_result_threshold(
        mut self,
        formula_result_threshold: bool,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::FormulaResultThreshold(formula_result_threshold))?;
        Ok(self)
    }

    pub fn inject_el_gordo_compounds(
        mut self,
        inject_el_gordo_compounds: bool,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::InjectElGordoCompounds(inject_el_gordo_compounds))?;
        Ok(self)
    }

    pub fn median_noise_intensity(mut self, median_noise_intensity: f32) -> Result<Self, String> {
        if median_noise_intensity < 0.0 {
            return Err(format!(
                concat!(
                    "Median noise intensity must be positive. ",
                    "You provided {}."
                ),
                median_noise_intensity
            ));
        }
        self.config
            .add_config_parameter(ConfigV5::MedianNoiseIntensity(median_noise_intensity))?;
        Ok(self)
    }

    pub fn ms1_absolute_intensity_error(
        mut self,
        ms1_absolute_intensity_error: f32,
    ) -> Result<Self, String> {
        if ms1_absolute_intensity_error < 0.0 {
            return Err(format!(
                concat!(
                    "MS1 absolute intensity error must be positive. ",
                    "You provided {}."
                ),
                ms1_absolute_intensity_error
            ));
        }
        self.config
            .add_config_parameter(ConfigV5::MS1AbsoluteIntensityError(
                ms1_absolute_intensity_error,
            ))?;
        Ok(self)
    }

    pub fn ms1_minimal_intensity_to_consider(
        mut self,
        ms1_minimal_intensity_to_consider: f32,
    ) -> Result<Self, String> {
        if ms1_minimal_intensity_to_consider < 0.0 {
            return Err(format!(
                concat!(
                    "MS1 minimal intensity to consider must be positive. ",
                    "You provided {}."
                ),
                ms1_minimal_intensity_to_consider
            ));
        }
        self.config
            .add_config_parameter(ConfigV5::MS1MinimalIntensityToConsider(
                ms1_minimal_intensity_to_consider,
            ))?;
        Ok(self)
    }

    pub fn ms1_relative_intensity_error(
        mut self,
        ms1_relative_intensity_error: f32,
    ) -> Result<Self, String> {
        if ms1_relative_intensity_error < 0.0 {
            return Err(format!(
                concat!(
                    "MS1 relative intensity error must be positive. ",
                    "You provided {}."
                ),
                ms1_relative_intensity_error
            ));
        }
        self.config
            .add_config_parameter(ConfigV5::MS1RelativeIntensityError(
                ms1_relative_intensity_error,
            ))?;
        Ok(self)
    }

    pub fn noise_threshold_settings_intensity_threshold(
        mut self,
        noise_threshold_settings_intensity_threshold: f32,
    ) -> Result<Self, String> {
        if noise_threshold_settings_intensity_threshold < 0.0 {
            return Err(format!(
                concat!(
                    "Noise threshold settings intensity threshold must be positive. ",
                    "You provided {}."
                ),
                noise_threshold_settings_intensity_threshold
            ));
        }
        self.config
            .add_config_parameter(ConfigV5::NoiseThresholdSettingsIntensityThreshold(
                noise_threshold_settings_intensity_threshold,
            ))?;
        Ok(self)
    }

    pub fn noise_threshold_settings_maximal_number_of_peaks(
        mut self,
        noise_threshold_settings_maximal_number_of_peaks: u32,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::NoiseThresholdSettingsMaximalNumberOfPeaks(
                noise_threshold_settings_maximal_number_of_peaks,
            ))?;
        Ok(self)
    }

    pub fn zodiac_cluster_compounds(
        mut self,
        zodiac_cluster_compounds: bool,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::ZodiacClusterCompounds(zodiac_cluster_compounds))?;
        Ok(self)
    }

    pub fn zodiac_edge_filter_thresholds_min_local_candidates(
        mut self,
        zodiac_edge_filter_thresholds_min_local_candidates: u32,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacEdgeFilterThresholdsMinLocalCandidates(
                zodiac_edge_filter_thresholds_min_local_candidates,
            ),
        )?;
        Ok(self)
    }

    pub fn zodiac_edge_filter_thresholds_min_local_connections(
        mut self,
        zodiac_edge_filter_thresholds_min_local_connections: u32,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacEdgeFilterThresholdsMinLocalConnections(
                zodiac_edge_filter_thresholds_min_local_connections,
            ),
        )?;
        Ok(self)
    }

    pub fn zodiac_edge_filter_thresholds_threshold_filter(
        mut self,
        zodiac_edge_filter_thresholds_threshold_filter: f32,
    ) -> Result<Self, String> {
        if zodiac_edge_filter_thresholds_threshold_filter < 0.0 {
            return Err(format!(
                concat!(
                    "Zodiac edge filter thresholds threshold filter must be positive. ",
                    "You provided {}."
                ),
                zodiac_edge_filter_thresholds_threshold_filter
            ));
        }
        self.config
            .add_config_parameter(ConfigV5::ZodiacEdgeFilterThresholdsThresholdFilter(
                zodiac_edge_filter_thresholds_threshold_filter,
            ))?;
        Ok(self)
    }

    pub fn zodiac_epochs_burn_in_period(
        mut self,
        zodiac_epochs_burn_in_period: u32,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::ZodiacEpochsBurnInPeriod(
                zodiac_epochs_burn_in_period,
            ))?;
        Ok(self)
    }

    pub fn zodiac_epochs_iterations(
        mut self,
        zodiac_epochs_number_of_epochs: u32,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::ZodiacEpochsIterations(
                zodiac_epochs_number_of_epochs,
            ))?;
        Ok(self)
    }

    pub fn zodiac_epochs_number_of_markov_chains(
        mut self,
        zodiac_epochs_number_of_markov_chains: u32,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::ZodiacEpochsNumberOfMarkovChains(
                zodiac_epochs_number_of_markov_chains,
            ))?;
        Ok(self)
    }

    pub fn zodiac_library_scoring_lambda(
        mut self,
        zodiac_library_scoring_lambda: u32,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::ZodiacLibraryScoringLambda(
                zodiac_library_scoring_lambda,
            ))?;
        Ok(self)
    }

    /// Set the minimal cosine value
    /// # Arguments
    /// * `zodiac_library_scoring_min_cosine` - The minimal cosine value.
    /// # Example
    /// ```
    /// use sirius::prelude::*;
    /// let sirius = SiriusBuilder::default()
    /// .zodiac_library_scoring_min_cosine(0.5).unwrap()
    /// .build();
    /// ```
    /// # Errors
    /// If the value is not in the range [0,1].
    /// # Example
    /// ```
    /// use sirius::prelude::*;
    /// assert!(SiriusBuilder::default().zodiac_library_scoring_min_cosine(1.1).is_err());
    /// assert!(SiriusBuilder::default().zodiac_library_scoring_min_cosine(-0.1).is_err());
    /// ```
    pub fn zodiac_library_scoring_min_cosine(
        mut self,
        zodiac_library_scoring_min_cosine: f32,
    ) -> Result<Self, String> {
        // Value must be in [0,1].
        if !(0.0..=1.0).contains(&zodiac_library_scoring_min_cosine) {
            // fast and easy way to check interval of values in Rust. Then add the "!" to negate the condition.
            return Err(format!(
                concat!(
                    "Zodiac library scoring min cosine must be in [0,1]. ",
                    "You provided {}."
                ),
                zodiac_library_scoring_min_cosine
            ));
        }
        self.config
            .add_config_parameter(ConfigV5::ZodiacLibraryScoringMinCosine(
                zodiac_library_scoring_min_cosine,
            ))?;
        Ok(self)
    }

    pub fn zodiac_number_of_considered_candidates_at_300_mz(
        mut self,
        zodiac_number_of_considered_candidates_at_300_mz: i32,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::ZodiacNumberOfConsideredCandidatesAt300Mz(
                zodiac_number_of_considered_candidates_at_300_mz,
            ))?;
        Ok(self)
    }

    pub fn zodiac_number_of_considered_candidates_at_800_mz(
        mut self,
        zodiac_number_of_considered_candidates_at_800_mz: i32,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::ZodiacNumberOfConsideredCandidatesAt800Mz(
                zodiac_number_of_considered_candidates_at_800_mz,
            ))?;
        Ok(self)
    }

    pub fn zodiac_ratio_of_considered_candidates_per_ionization(
        mut self,
        zodiac_ratio_of_considered_candidates_per_ionization: f32,
    ) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&zodiac_ratio_of_considered_candidates_per_ionization) {
            return Err(format!(
                concat!(
                    "Zodiac ratio of considered candidates per ionization must be in [0,1]. ",
                    "You provided {}."
                ),
                zodiac_ratio_of_considered_candidates_per_ionization
            ));
        }
        self.config.add_config_parameter(
            ConfigV5::ZodiacRatioOfConsideredCandidatesPerIonization(
                zodiac_ratio_of_considered_candidates_per_ionization,
            ),
        )?;
        Ok(self)
    }

    pub fn zodiac_run_in_two_steps(
        mut self,
        zodiac_run_in_two_steps: bool,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::ZodiacRunInTwoSteps(zodiac_run_in_two_steps))?;
        Ok(self)
    }

    /// This function return the allowed mass deviation for MS1.
    /// It should get a float and then we should also know the unit of the mass deviation.
    /// Mass deviation enum is defined as :
    /// ```rust
    /// pub enum MassDeviation {
    /// Ppm(f32),
    /// Da(f32),
    /// }
    /// ```
    ///
    pub fn ms1_mass_deviation_allowed_mass_deviation(
        mut self,
        ms1_mass_deviation_allowed_mass_deviation: MassDeviation,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::MS1MassDeviationAllowedMassDeviation(
                ms1_mass_deviation_allowed_mass_deviation.must_be_positive()?,
            ))?;
        Ok(self)
    }

    pub fn ms1_mass_deviation_mass_difference_deviation(
        mut self,
        ms1_mass_deviation_mass_difference_deviation: MassDeviation,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::MS1MassDeviationMassDifferenceDeviation(
                ms1_mass_deviation_mass_difference_deviation.must_be_positive()?,
            ))?;
        Ok(self)
    }

    pub fn ms1_mass_deviation_standard_mass_deviation(
        mut self,
        ms1_mass_deviation_standard_mass_deviation: MassDeviation,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::MS1MassDeviationStandardMassDeviation(
                ms1_mass_deviation_standard_mass_deviation.must_be_positive()?,
            ))?;
        Ok(self)
    }

    pub fn ms2_mass_deviation_standard_mass_deviation(
        mut self,
        ms2_mass_deviation_standard_mass_deviation: MassDeviation,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::MS2MassDeviationStandardMassDeviation(
                ms2_mass_deviation_standard_mass_deviation.must_be_positive()?,
            ))?;
        Ok(self)
    }

    pub fn ms2_mass_deviation_allowed_mass_deviation(
        mut self,
        ms2_mass_deviation_allowed_mass_deviation: MassDeviation,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::MS2MassDeviationAllowedMassDeviation(
                ms2_mass_deviation_allowed_mass_deviation.must_be_positive()?,
            ))?;
        Ok(self)
    }

    pub fn formula_settings_detectable(
        mut self,
        formula_settings_detectable: AtomVector,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::FormulaSettingsDetectable(
                formula_settings_detectable,
            ))?;
        Ok(self)
    }

    pub fn formula_settings_enforced(
        mut self,
        formula_settings_enforced: AtomVector,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::FormulaSettingsEnforced(formula_settings_enforced))?;
        Ok(self)
    }

    pub fn formula_settings_fallback(
        mut self,
        formula_settings_fallback: AtomVector,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::FormulaSettingsFallback(formula_settings_fallback))?;
        Ok(self)
    }

    pub fn forbid_recalibration(
        mut self,
        forbid_recalibration: ForbidRecalibration,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::ForbidRecalibration(forbid_recalibration))?;
        Ok(self)
    }

    pub fn use_heuristic_mz_to_use_heuristic(
        mut self,
        use_heuristic_mz_to_use_heuristic: u32,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::UseHeuristicMZToUseHeuristic(
                use_heuristic_mz_to_use_heuristic,
            ))?;
        Ok(self)
    }

    pub fn use_heuristic_mz_to_use_heuristic_only(
        mut self,
        use_heuristic_mz_to_use_heuristic: u32,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::UseHeuristicMZToUseHeuristicOnly(
                use_heuristic_mz_to_use_heuristic,
            ))?;
        Ok(self)
    }

    pub fn adduct_settings_detectable(
        mut self,
        adduct_settings_detectable: AdductsVector,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::AdductSettingsDetectable(
                adduct_settings_detectable,
            ))?;
        Ok(self)
    }

    pub fn adduct_settings_fallback(
        mut self,
        adduct_settings_fallback: AdductsVector,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::AdductSettingsFallback(adduct_settings_fallback))?;
        Ok(self)
    }

    pub fn algorithm_profile(mut self, algorithm_profile: Instruments) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::AlgorithmProfile(algorithm_profile))?;
        Ok(self)
    }

    pub fn compound_quality(mut self, compound_quality: CompoundQuality) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::CompoundQuality(compound_quality))?;
        Ok(self)
    }

    pub fn adduct_settings_enforced(
        mut self,
        adduct_settings_enforced: AdductSettingsEnforced,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::AdductSettingsEnforced(adduct_settings_enforced))?;
        Ok(self)
    }

    pub fn candidate_formulas(
        mut self,
        candidate_formulas: CandidateFormulas,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::CandidateFormulas(candidate_formulas))?;
        Ok(self)
    }

    pub fn formula_result_ranking_score(
        mut self,
        formula_result_ranking_score: FormulaResultRankingScore,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::FormulaResultRankingScore(
                formula_result_ranking_score,
            ))?;
        Ok(self)
    }

    pub fn isotope_ms2_settings(
        mut self,
        isotope_ms2_settings: IsotopeMS2Settings,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::IsotopeMS2Settings(isotope_ms2_settings))?;
        Ok(self)
    }

    pub fn isotope_settings_multiplier(
        mut self,
        isotope_settings_multiplier: u32,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::IsotopeSettingsMultiplier(
                isotope_settings_multiplier,
            ))?;
        Ok(self)
    }

    pub fn noise_threshold_settings_absolute_threshold(
        mut self,
        noise_threshold_settings_absolute_threshold: u32,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::NoiseThresholdSettingsAbsoluteThreshold(
                noise_threshold_settings_absolute_threshold,
            ))?;
        Ok(self)
    }

    pub fn noise_threshold_settings_base_peak(
        mut self,
        noise_threshold_settings_base_peak: BasePeak,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::NoiseThresholdSettingsBasePeak(
                noise_threshold_settings_base_peak,
            ))?;
        Ok(self)
    }

    pub fn structure_predictors(
        mut self,
        structure_predictors: StructurePredictors,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::StructurePredictors(structure_predictors))?;
        Ok(self)
    }

    pub fn possible_adduct_switches(
        mut self,
        possible_adduct_switches: PossibleAdductSwitches,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::PossibleAdductSwitches(possible_adduct_switches))?;
        Ok(self)
    }

    /// End `sirius config` command parameters.

    ///
    ///
    ///
    ///
    /// Wether to enable the Formula module.
    pub fn enable_formula(mut self) -> Result<Self, String> {
        self.config.add_formula_parameter(FormulaV5::Enabled)?;
        Ok(self)
    }

    /// Set whether to display the help of Formula.
    pub fn formula_help(mut self) -> Result<Self, String> {
        self.config.add_formula_parameter(FormulaV5::Help)?;
        Ok(self)
    }

    /// Set whether to display the version of Formula.
    pub fn formula_version(mut self) -> Result<Self, String> {
        self.config.add_formula_parameter(FormulaV5::Version)?;
        Ok(self)
    }

    /// Wether to enable the Zodiac module.
    pub fn enable_zodiac(mut self) -> Result<Self, String> {
        self.config.add_zodiac_parameter(ZodiacV5::Enabled)?;
        Ok(self)
    }

    /// Set whether to display the help of Zodiac.
    pub fn zodiac_help(mut self) -> Result<Self, String> {
        self.config.add_zodiac_parameter(ZodiacV5::Help)?;
        Ok(self)
    }

    /// Set whether to display the version of Zodiac.
    pub fn zodiac_version(mut self) -> Result<Self, String> {
        self.config.add_zodiac_parameter(ZodiacV5::Version)?;
        Ok(self)
    }

    /// Wether to enable the Fingerprint module.
    pub fn enable_fingerprint(mut self) -> Result<Self, String> {
        self.config
            .add_fingerprint_parameter(FingerprintV5::Enabled)?;
        Ok(self)
    }

    /// Set whether to display the help of Fingerprint.
    pub fn fingerprint_help(mut self) -> Result<Self, String> {
        self.config.add_fingerprint_parameter(FingerprintV5::Help)?;
        Ok(self)
    }

    /// Set whether to display the version of Fingerprint.
    pub fn fingerprint_version(mut self) -> Result<Self, String> {
        self.config
            .add_fingerprint_parameter(FingerprintV5::Version)?;
        Ok(self)
    }

    /// Wether to enable the Structure module.
    pub fn enable_structure(mut self) -> Result<Self, String> {
        self.config.add_structure_parameter(StructureV5::Enabled)?;
        Ok(self)
    }

    /// Set whether to display the help of Structure.
    pub fn structure_help(mut self) -> Result<Self, String> {
        self.config.add_structure_parameter(StructureV5::Help)?;
        Ok(self)
    }

    /// Set whether to display the version of Structure.
    pub fn structure_version(mut self) -> Result<Self, String> {
        self.config.add_structure_parameter(StructureV5::Version)?;
        Ok(self)
    }

    /// Whether to enable the Canopus module.
    pub fn enable_canopus(mut self) -> Result<Self, String> {
        self.config.add_canopus_parameter(CanopusV5::Enabled)?;
        Ok(self)
    }

    /// Set whether to display the help of Canopus.
    pub fn canopus_help(mut self) -> Result<Self, String> {
        self.config.add_canopus_parameter(CanopusV5::Help)?;
        Ok(self)
    }

    /// Set whether to display the version of Canopus.
    pub fn canopus_version(mut self) -> Result<Self, String> {
        self.config.add_canopus_parameter(CanopusV5::Version)?;
        Ok(self)
    }
}
impl<V: Version> SiriusBuilder<V> {
    pub fn build(self) -> Sirius<V> {
        Sirius::from(self.config)
    }
}

impl SiriusBuilder<Version5> {
    /// Set the default maximal value of m/z ratio on which Sirius calculation will be carried.
    ///
    /// # Example
    ///
    /// ```
    /// use sirius::prelude::*;
    ///
    /// let sirius = SiriusBuilder::default()
    ///    .maximal_mz_default().unwrap()
    ///  .build();
    ///
    /// assert!(SiriusBuilder::default().maximal_mz_default().is_ok());
    ///
    /// assert!(SiriusBuilder::default().maximal_mz_default().unwrap().maximal_mz_default().is_err());
    ///
    ///
    /// ```
    pub fn maximal_mz_default(mut self) -> Result<Self, String> {
        self.config
            .add_core_parameter(CoreV5::MaximalMz(f64::default()).into_default())?;
        Ok(self)
    }

    pub fn isotope_settings_filter_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::IsotopeSettingsFilter(bool::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn formula_search_db_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::FormulaSearchDB(crate::sirius_types::FormulaSearchDB::default())
                .into_default(),
        )?;
        Ok(self)
    }
    pub fn structure_search_db_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::StructureSearchDB(crate::sirius_types::FormulaSearchDB::default())
                .into_default(),
        )?;
        Ok(self)
    }

    pub fn timeout_seconds_per_tree_default(mut self) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::TimeoutSecondsPerTree(u32::default()).into_default())?;
        Ok(self)
    }

    pub fn number_of_candidates_default(mut self) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::NumberOfCandidates(u32::default()).into_default())?;
        Ok(self)
    }

    pub fn number_of_candidates_per_ion_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::NumberOfCandidatesPerIon(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn number_of_structure_candidates_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::NumberOfStructureCandidates(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn recompute_results_default(mut self) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::RecomputeResults(bool::default()).into_default())?;
        Ok(self)
    }

    pub fn print_citations_default(mut self) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::PrintCitations(bool::default()).into_default())?;
        Ok(self)
    }

    pub fn timeout_seconds_per_instance_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::TimeoutSecondsPerInstance(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn formula_result_threshold_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::FormulaResultThreshold(bool::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn inject_el_gordo_compounds_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::InjectElGordoCompounds(bool::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn median_noise_intensity_default(mut self) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::MedianNoiseIntensity(f32::default()).into_default())?;
        Ok(self)
    }

    pub fn ms1_absolute_intensity_error_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS1AbsoluteIntensityError(f32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn ms1_minimal_intensity_to_consider_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS1MinimalIntensityToConsider(f32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn ms1_relative_intensity_error_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS1RelativeIntensityError(f32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn noise_threshold_settings_intensity_threshold_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::NoiseThresholdSettingsIntensityThreshold(f32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn noise_threshold_settings_maximal_number_of_peaks_default(
        mut self,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::NoiseThresholdSettingsMaximalNumberOfPeaks(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn zodiac_cluster_compounds_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacClusterCompounds(bool::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn zodiac_edge_filter_thresholds_min_local_candidates_default(
        mut self,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacEdgeFilterThresholdsMinLocalCandidates(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn zodiac_edge_filter_thresholds_min_local_connections_default(
        mut self,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacEdgeFilterThresholdsMinLocalConnections(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn zodiac_edge_filter_thresholds_threshold_filter_default(
        mut self,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacEdgeFilterThresholdsThresholdFilter(f32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn zodiac_epochs_burn_in_period_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacEpochsBurnInPeriod(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn zodiac_epochs_iterations_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacEpochsIterations(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn zodiac_epochs_number_of_markov_chains_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacEpochsNumberOfMarkovChains(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn zodiac_library_scoring_lambda_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacLibraryScoringLambda(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn zodiac_library_scoring_min_cosine_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacLibraryScoringMinCosine(f32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn zodiac_number_of_considered_candidates_at_300_mz_default(
        mut self,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacNumberOfConsideredCandidatesAt300Mz(i32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn zodiac_number_of_considered_candidates_at_800_mz_default(
        mut self,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacNumberOfConsideredCandidatesAt800Mz(i32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn zodiac_ratio_of_considered_candidates_per_ionization_default(
        mut self,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacRatioOfConsideredCandidatesPerIonization(f32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn zodiac_run_in_two_steps_default(mut self) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::ZodiacRunInTwoSteps(bool::default()).into_default())?;
        Ok(self)
    }

    pub fn ms1_mass_deviation_allowed_mass_deviation_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS1MassDeviationAllowedMassDeviation(
                MassDeviation::Ppm(f32::default()).must_be_positive()?,
            )
            .into_default(),
        )?;
        Ok(self)
    }

    pub fn ms1_mass_deviation_mass_difference_deviation_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS1MassDeviationMassDifferenceDeviation(
                MassDeviation::Ppm(f32::default()).must_be_positive()?,
            )
            .into_default(),
        )?;
        Ok(self)
    }

    pub fn ms1_mass_deviation_standard_mass_deviation_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS1MassDeviationStandardMassDeviation(
                MassDeviation::Ppm(f32::default()).must_be_positive()?,
            )
            .into_default(),
        )?;
        Ok(self)
    }

    pub fn ms2_mass_deviation_standard_mass_deviation_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS2MassDeviationStandardMassDeviation(
                MassDeviation::Ppm(f32::default()).must_be_positive()?,
            )
            .into_default(),
        )?;
        Ok(self)
    }

    pub fn ms2_mass_deviation_allowed_mass_deviation_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS2MassDeviationAllowedMassDeviation(
                MassDeviation::Ppm(f32::default()).must_be_positive()?,
            )
            .into_default(),
        )?;
        Ok(self)
    }

    pub fn formula_settings_detectable_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::FormulaSettingsDetectable(AtomVector::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn formula_settings_enforced_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::FormulaSettingsEnforced(AtomVector::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn formula_settings_fallback_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::FormulaSettingsFallback(AtomVector::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn forbid_recalibration_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ForbidRecalibration(ForbidRecalibration::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn use_heuristic_mz_to_use_heuristic_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::UseHeuristicMZToUseHeuristic(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn use_heuristic_mz_to_use_heuristic_only_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::UseHeuristicMZToUseHeuristicOnly(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn adduct_settings_detectable_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::AdductSettingsDetectable(AdductsVector::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn adduct_settings_fallback_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::AdductSettingsFallback(AdductsVector::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn algorithm_profile_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::AlgorithmProfile(Instruments::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn compound_quality_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::CompoundQuality(CompoundQuality::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn adduct_settings_enforced_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::AdductSettingsEnforced(AdductSettingsEnforced::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn candidate_formulas_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::CandidateFormulas(CandidateFormulas::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn formula_result_ranking_score_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::FormulaResultRankingScore(FormulaResultRankingScore::default())
                .into_default(),
        )?;
        Ok(self)
    }

    pub fn isotope_ms2_settings_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::IsotopeMS2Settings(IsotopeMS2Settings::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn isotope_settings_multiplier_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::IsotopeSettingsMultiplier(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn noise_threshold_settings_absolute_threshold_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::NoiseThresholdSettingsAbsoluteThreshold(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn noise_threshold_settings_base_peak_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::NoiseThresholdSettingsBasePeak(BasePeak::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn structure_predictors_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::StructurePredictors(StructurePredictors::default()).into_default(),
        )?;
        Ok(self)
    }

    pub fn possible_adduct_switches_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::PossibleAdductSwitches(PossibleAdductSwitches::default()).into_default(),
        )?;
        Ok(self)
    }
}
