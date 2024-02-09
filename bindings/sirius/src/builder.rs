//! A builder is a type of struct that will collect configurations and once build, prodiuces a complete struct.
//!
use crate::prelude::*;
use crate::sirius_config::SiriusConfig;
use crate::traits::IntoDefault;

/// The SiriusBuilder is used to set the parameters of the SiriusConfig.
#[derive(Default)]
pub struct SiriusBuilder<V: Version> {
    config: SiriusConfig<V>,
}

/// The functions in this block are used to set the parameters of the SiriusBuilder.
/// Most of the functions come from the `sirius config` command. The comments in the functions are usually a copy-paste from the `sirius config --help` command.
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
    ///  .formula_search_db(DBVector::from(vec![SearchDB::Hmdb])).unwrap()
    /// .build();
    ///
    /// assert!(SiriusBuilder::default().formula_search_db(DBVector::from(vec![SearchDB::Hmdb])).is_ok());
    /// ```
    pub fn formula_search_db(
        mut self,
        formula_search_db: crate::sirius_types::DBVector,
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
    /// .structure_search_db(DBVector::from(vec![SearchDB::Zincbio])).unwrap()
    /// .build();
    /// ```
    pub fn structure_search_db(
        mut self,
        structure_search_db: crate::sirius_types::DBVector,
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

    /// Set the number of candidates.
    /// # Arguments
    /// * `number_of_candidates` - The number of candidates.
    pub fn number_of_candidates(mut self, number_of_candidates: u32) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::NumberOfCandidates(number_of_candidates))?;
        Ok(self)
    }

    /// Set the number of candidates per ion.
    /// # Arguments
    /// * `number_of_candidates_per_ion` - The number of candidates per ion.
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

    /// Sets the number of structure candidates.
    /// # Arguments
    /// * `number_of_structure_candidates` - The number of structure candidates.
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

    /// Specify if the results should be recomputed.
    /// # Arguments
    /// * `recompute_results` - Whether to recompute the results.
    pub fn recompute_results(mut self, recompute_results: bool) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::RecomputeResults(recompute_results))?;
        Ok(self)
    }

    /// Whether to print citations we Sirius has finished running.
    /// # Arguments
    /// * `print_citations` - Whether to print citations.
    pub fn print_citations(mut self, print_citations: bool) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::PrintCitations(print_citations))?;
        Ok(self)
    }

    /// This configurations define a timeout for the tree computation.
    /// As the underlying problem is NP-hard, it might take forever to compute trees for very challenging (e.g. large mass) compounds.
    /// Setting a time constraint allow the program to continue with other instances and just skip the challenging ones.
    /// Note that, due to multithreading, this time constraints are not absolutely accurate.
    /// Set the maximum number of seconds for computing a single compound. Set to 0 to disable the time constraint.
    /// # Arguments
    /// * `timeout_seconds_per_instance` - The maximum number of seconds for computing a single compound.
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

    /// Specifies if the list of Molecular Formula Identifications is filtered by a soft threshold (calculateThreshold)
    /// before CSI:FingerID predictions are calculated.
    /// # Arguments
    /// * `formula_settings_filter` - Whether to filter the list of Molecular Formula Identifications.
    pub fn formula_result_threshold(
        mut self,
        formula_result_threshold: bool,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::FormulaResultThreshold(formula_result_threshold))?;
        Ok(self)
    }

    /// Candidates matching the lipid class estimated by El Gordo will be tagged.
    /// The lipid class will only be available if El Gordo predicts that the MS/MS is a lipid spectrum.
    /// If this parameter is set to 'false' El Gordo will still be executed and e.g. improve the fragmentation tree,
    /// but the matching candidates will not be tagged as lipid class.
    /// # Arguments
    /// * `inject_el_gordo_compounds` - Whether to inject El Gordo compounds.
    pub fn inject_el_gordo_compounds(
        mut self,
        inject_el_gordo_compounds: bool,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::InjectElGordoCompounds(inject_el_gordo_compounds))?;
        Ok(self)
    }

    /// Sets the median noise intensity.
    /// # Arguments
    /// * `median_noise_intensity` - The median noise intensity.
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

    /// The average absolute deviation between theoretical and measured intensity of isotope peaks.
    ///
    /// Do not change this parameter without a good reason!    
    ///
    /// Ideally use the `ms1_absolute_intensity_error_default()` function.
    /// # Arguments
    /// * `ms1_absolute_intensity_error` - The average absolute deviation between theoretical and measured intensity of isotope peaks.
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

    /// Ignore isotope peaks below this intensity.
    /// This value should reflect the smallest relative intensive which is still above noise level.
    /// Obviously, this is hard to judge without having absolute values.
    /// Keeping this value around 1 percent is fine for most settings. Set it to smaller values if you trust your small intensities.
    /// # Arguments
    /// * `ms1_minimal_intensity_to_consider` - The minimal intensity to consider.
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

    /// The average relative deviation between theoretical and measured intensity of isotope peaks.
    ///
    /// Do not change this parameter without a good reason!
    ///
    /// Ideally use the `ms1_relative_intensity_error_default()` function.
    /// # Arguments
    /// * `ms1_relative_intensity_error` - The average relative deviation between theoretical and measured intensity of isotope peaks.
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

    /// Sets the noise threshold settings absolute threshold.
    /// # Arguments
    /// * `noise_threshold_settings_absolute_threshold` - The noise threshold settings absolute threshold.
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

    /// Sets the noise threshold settings maximal number of peaks.
    /// # Arguments
    /// * `noise_threshold_settings_maximal_number_of_peaks` - The noise threshold settings maximal number of peaks.
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

    /// Sets if you want to cluster compounds before running ZODIAC.
    /// # Arguments
    /// * `zodiac_cluster_compounds` - Whether to cluster compounds before running ZODIAC.
    pub fn zodiac_cluster_compounds(
        mut self,
        zodiac_cluster_compounds: bool,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::ZodiacClusterCompounds(zodiac_cluster_compounds))?;
        Ok(self)
    }

    /// Minimum number of candidates per compound which are forced to have at least \[minLocalConnections\] connections to other compounds.
    /// E.g. 2 candidates per compound must have at least 10 connections to other compounds.
    /// # Arguments
    /// * `zodiac_edge_filter_thresholds_min_local_candidates` - The minimum number of candidates per compound.
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

    ///  Minimum number of connections per candidate which are forced for at least \[minLocalCandidates\] candidates to other compounds.
    /// E.g. 2 candidates per compound must have at least 10 connections to other compounds.
    /// # Arguments
    /// * `zodiac_edge_filter_thresholds_min_local_connections` - The minimum number of connections per candidate.
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

    /// Defines the proportion of edges of the complete network which will be ignored.
    /// # Arguments
    /// * `zodiac_edge_filter_thresholds_threshold_filter` - The proportion of edges of the complete network which will be ignored.
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

    /// Number of epochs considered as 'burn-in period'.
    /// Samples from the beginning of a Markov chain do not accurately represent the desired distribution of candidates and are not used to estimate the ZODIAC score.
    /// # Arguments
    /// * `zodiac_epochs_burn_in_period` - The number of epochs considered as 'burn-in period'.
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

    /// Number of epochs to run the Gibbs sampling. When multiple Markov chains are computed, all chains' iterations sum up to this value.
    /// # Arguments
    /// * `zodiac_epochs_iterations` - The number of epochs to run the Gibbs sampling.
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

    /// Number of separate Gibbs sampling runs.
    /// # Arguments
    /// * `zodiac_epochs_number_of_markov_chains` - The number of separate Gibbs sampling runs.
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

    /// Lambda used in the scoring function of spectral library hits. The higher this value the higher are librar hits weighted in ZODIAC scoring.
    /// # Arguments
    /// * `zodiac_library_scoring_lambda` - The lambda used in the scoring function of spectral library hits.
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

    /// Set the minimal cosine value. Values must be between 0 and 1.
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
    /// If the value is not in the range 0 and 1.
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

    /// Maximum number of candidate molecular formulas (fragmentation trees computed by SIRIUS) per compound which are considered by ZODIAC.
    /// This is the threshold used for all compounds with mz below 300 m/z and is used to interpolate the number of candidates for larger compounds.
    /// If lower than 0, all available candidates are considered.
    /// # Arguments
    /// * `zodiac_number_of_considered_candidates_at_300_mz` - The maximum number of candidate molecular formulas.
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

    /// Maximum number of candidate molecular formulas (fragmentation trees computed by SIRIUS) per compound which are considered by ZODIAC.
    /// This is the threshold used for all compounds with mz below 800 m/z and is used to interpolate the number of candidates for larger compounds.
    /// If lower than 0, all available candidates are considered.
    /// # Arguments
    /// * `zodiac_number_of_considered_candidates_at_800_mz` - The maximum number of candidate molecular formulas.
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

    /// Ratio of candidate molecular formulas (fragmentation trees computed by SIRIUS) per compound which are forced for each ionization to be
    /// considered by ZODIAC. This depends on the number of candidates ZODIAC considers.
    /// E.g. if 50 candidates are considered and a ratio of 0.2 is set,
    /// at least 10 candidates per ionization will be considered, which might increase the number of candidates above 50.
    /// # Arguments
    /// * `zodiac_ratio_of_considered_candidates_per_ionization` - The ratio of candidate molecular formulas.
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

    /// As default ZODIAC runs a 2-step approach. First running 'good quality compounds' only, and afterwards including the remaining.
    /// # Arguments
    /// * `zodiac_run_in_two_steps` - Whether to run ZODIAC in two steps.
    pub fn zodiac_run_in_two_steps(
        mut self,
        zodiac_run_in_two_steps: bool,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::ZodiacRunInTwoSteps(zodiac_run_in_two_steps))?;
        Ok(self)
    }

    /// Mass accuracy setting for MS1 spectra. Mass accuracies are always written as "X ppm (Y Da)"
    /// with X and Y are numerical values. The ppm is a relative measure
    /// (parts per million), Da is an absolute measure. For each mass, the
    /// maximum of relative and absolute is used.
    /// # Arguments
    /// * `ms1_mass_deviation_allowed_mass_deviation` - The mass accuracy setting for MS1 spectra.
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

    /// The difference is mass deviation between two masses.
    /// # Arguments
    /// * `ms1_mass_deviation_mass_difference_deviation` - The mass deviation between two masses.
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

    /// The standard mass deviation for MS1 spectra.
    /// # Arguments
    /// * `ms1_mass_deviation_standard_mass_deviation` - The standard mass deviation for MS1 spectra.
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

    /// The standard mass deviation for MS2 spectra.
    /// # Arguments
    /// * `ms2_mass_deviation_mass_difference_deviation` - The standard mass deviation for MS2 spectra.
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

    /// Mass accuracy setting for MS2 spectra. Mass accuracies are always written as "X ppm (Y Da)"
    /// with X and Y are numerical values. The ppm is a relative measure
    /// (parts per million), Da is an absolute measure. For each mass, the
    /// maximum of relative and absolute is used.
    /// # Arguments
    /// * `ms2_mass_deviation_allowed_mass_deviation` - The mass accuracy setting for MS2 spectra.
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

    /// Detectable elements are added to the chemical alphabet, if there are indications for them (e.g. in isotope pattern)
    /// # Arguments
    /// * `formula_settings_detectable` - The detectable elements.
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

    /// These configurations hold the information how to
    /// autodetect elements based on the given formula constraints.
    /// Note: If the compound is already assigned to a
    /// specific molecular formula, this annotation is
    /// ignored. Enforced elements are always considered.
    /// # Arguments
    /// * `formula_settings_enforced` - The enforced elements.
    pub fn formula_settings_enforced(
        mut self,
        formula_settings_enforced: AtomVector,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::FormulaSettingsEnforced(formula_settings_enforced))?;
        Ok(self)
    }

    /// Fallback elements are used, if the auto-detection fails (e.g. no isotope pattern available)
    /// # Arguments
    /// * `formula_settings_fallback` - The fallback elements.
    pub fn formula_settings_fallback(
        mut self,
        formula_settings_fallback: AtomVector,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::FormulaSettingsFallback(formula_settings_fallback))?;
        Ok(self)
    }

    /// Enable/Disable the hypothesen driven recalibration of MS/MS spectra.
    /// Must be either 'ALLOWED' or FORBIDDEN'.
    /// # Arguments
    /// * `forbid_recalibration` - Whether to forbid recalibration.
    pub fn forbid_recalibration(
        mut self,
        forbid_recalibration: ForbidRecalibration,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::ForbidRecalibration(forbid_recalibration))?;
        Ok(self)
    }

    /// Set minimum m/z to enable heuristic preprocessing. The heuristic will be used to initially rank the formula candidates.
    /// The Top (NumberOfCandidates) candidates will then be computed exactly by solving the ILP.
    /// # Arguments
    /// * `use_heuristic_mz_to_use_heuristic` - The minimum m/z to enable heuristic preprocessing.
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

    /// Set minimum m/z to only use heuristic tree computation. No exact tree computation (ILP) will be performed for this compounds.
    /// # Arguments
    /// * `use_heuristic_mz_to_use_heuristic_only` - The minimum m/z to only use heuristic tree computation.
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

    ///  Detectable ion modes which are only considered if there is an indication in the MS1 scan (e.g. correct mass delta).
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

    /// Fallback ion modes which are considered if the auto detection did not find any indication for an ion mode.
    /// # Arguments
    /// * `adduct_settings_fallback` - The fallback ion modes.
    pub fn adduct_settings_fallback(
        mut self,
        adduct_settings_fallback: AdductsVector,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::AdductSettingsFallback(adduct_settings_fallback))?;
        Ok(self)
    }

    /// Configuration profile to store instrument specific algorithm properties. Some of the default profiles are: 'qtof', 'orbitrap', 'fticr'.
    /// # Arguments
    /// * `algorithm_profile` - The algorithm profile.
    pub fn algorithm_profile(mut self, algorithm_profile: Instruments) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::AlgorithmProfile(algorithm_profile))?;
        Ok(self)
    }

    /// Keywords that can be assigned to a input spectrum to judge its quality.
    /// Available keywords are: Good, LowIntensity, NoMS1Peak, FewPeaks, Chimeric, NotMonoisotopicPeak, PoorlyExplained
    /// # Arguments
    /// * `compound_quality` - The compound quality.
    pub fn compound_quality(mut self, compound_quality: CompoundQuality) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::CompoundQuality(compound_quality))?;
        Ok(self)
    }

    /// Enforced ion modes that are always considered.
    /// # Arguments
    /// * `adduct_settings_enforced` - The enforced ion modes.
    pub fn adduct_settings_enforced(
        mut self,
        adduct_settings_enforced: AdductSettingsEnforced,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::AdductSettingsEnforced(adduct_settings_enforced))?;
        Ok(self)
    }

    /// This configuration holds a set of user given formulas to be used as candidates for SIRIUS
    /// Note: This set might be merged with other sources like formulas from databases
    /// Set of Molecular Formulas to be used as candidates for molecular formula estimation with SIRIUS
    /// # Arguments
    /// * `candidate_formulas` - The candidate formulas.
    pub fn candidate_formulas(
        mut self,
        candidate_formulas: CandidateFormulas,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::CandidateFormulas(candidate_formulas))?;
        Ok(self)
    }

    /// Allows the USER to Specify the ScoreType that is used to rank the list of Molecular Formula Identifications
    /// before CSI:FingerID predictions are calculated. Auto means that this ScoreType is automatically set depending on the executed workflow.
    /// # Arguments
    /// * `formula_result_ranking_score` - The score type that is used to rank the list of Molecular Formula Identifications.
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

    /// Wheter to use the isotopes for the MS2 spectra.
    pub fn isotope_ms2_settings(
        mut self,
        isotope_ms2_settings: IsotopeMS2Settings,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::IsotopeMS2Settings(isotope_ms2_settings))?;
        Ok(self)
    }

    /// multiplier for the isotope score. Set to 0 to disable isotope scoring. Otherwise, the score from isotope pattern analysis is multiplied with this coefficient.
    /// Set to a value larger than one if your isotope pattern data is of much better quality than your MS/MS data.
    /// # Arguments
    /// * `isotope_settings_multiplier` - The multiplier for the isotope score.
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

    /// The absolute threshold for the noise
    /// # Arguments
    /// * `noise_threshold_settings_absolute_threshold` - The absolute threshold for the noise.
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

    /// The base peak for the noise.
    /// # Arguments
    /// * `noise_threshold_settings_base_peak` - The base peak for the noise.
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

    /// Setups the algorithm for the structure predictors. This should be CSI:FingerID
    /// # Arguments
    /// * `structure_predictors` - The algorithm for the structure predictors.
    pub fn structure_predictors(
        mut self,
        structure_predictors: StructurePredictors,
    ) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::StructurePredictors(structure_predictors))?;
        Ok(self)
    }

    /// An adduct switch is a switch of the ionization mode within a spectrum, e.g. an ion replaces an
    /// sodium adduct with a protonation during fragmentation. Such adduct switches heavily increase the
    /// complexity of the analysis, but for certain adducts they might happen
    /// regularly. Adduct switches are written in the form  {@literal a -> b, a -> c, d -> c} where a, b,
    /// c, and d are adducts and  {@literal a -> b} denotes an allowed switch from a to b within the MS/MS spectrum.
    /// # Arguments
    /// * `possible_adduct_switches` - The possible adduct switches.
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

    /// Whether to enable the WriteSummaries module.
    pub fn enable_write_summaries(mut self) -> Result<Self, String> {
        self.config
            .add_write_summaries_parameter(WriteSummariesV5::Enabled)?;
        Ok(self)
    }

    /// Set whether to display the help of WriteSummaries.
    pub fn write_summaries_help(mut self) -> Result<Self, String> {
        self.config
            .add_write_summaries_parameter(WriteSummariesV5::Help)?;
        Ok(self)
    }

    /// Set whether to display the version of WriteSummaries.
    pub fn write_summaries_version(mut self) -> Result<Self, String> {
        self.config
            .add_write_summaries_parameter(WriteSummariesV5::Version)?;
        Ok(self)
    }
}

impl<V: Version> SiriusBuilder<V> {
    /// Build the Sirius instance from the configuration.
    /// # Example
    /// ```
    /// use sirius::prelude::*;
    /// let sirius = SiriusBuilder::<Version5>::default().build();
    /// ```
    pub fn build(self) -> Sirius<V> {
        Sirius::from(self.config)
    }
}

impl SiriusBuilder<Version5> {
    /// Set to default maximal value of m/z ratio on which Sirius calculation will be carried.
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

    /// Set to default isotope settings filter.
    pub fn isotope_settings_filter_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::IsotopeSettingsFilter(bool::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default isotope settings intensity threshold.
    pub fn formula_search_db_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::FormulaSearchDB(crate::sirius_types::DBVector::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default structure search db.
    pub fn structure_search_db_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::StructureSearchDB(crate::sirius_types::DBVector::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default timeout seconds per tree.
    pub fn timeout_seconds_per_tree_default(mut self) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::TimeoutSecondsPerTree(u32::default()).into_default())?;
        Ok(self)
    }

    /// Set to default number of candidates.
    pub fn number_of_candidates_default(mut self) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::NumberOfCandidates(u32::default()).into_default())?;
        Ok(self)
    }

    /// Set to default number of candidates per ion.
    pub fn number_of_candidates_per_ion_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::NumberOfCandidatesPerIon(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default number of structure candidates.
    pub fn number_of_structure_candidates_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::NumberOfStructureCandidates(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default wheter to recompute results.
    pub fn recompute_results_default(mut self) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::RecomputeResults(bool::default()).into_default())?;
        Ok(self)
    }

    /// Set to default wheter to print citations.
    pub fn print_citations_default(mut self) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::PrintCitations(bool::default()).into_default())?;
        Ok(self)
    }

    /// Set to default timeout seconds per instance.
    pub fn timeout_seconds_per_instance_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::TimeoutSecondsPerInstance(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default wheter to use the formula result threshold.
    pub fn formula_result_threshold_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::FormulaResultThreshold(bool::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Whether to use the default El Gordo compounds setting.
    pub fn inject_el_gordo_compounds_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::InjectElGordoCompounds(bool::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the median noise intensity.
    pub fn median_noise_intensity_default(mut self) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::MedianNoiseIntensity(f32::default()).into_default())?;
        Ok(self)
    }

    /// Set to default the MS1 absolute intensity error.
    pub fn ms1_absolute_intensity_error_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS1AbsoluteIntensityError(f32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the MS1 minimal intensity to consider.
    pub fn ms1_minimal_intensity_to_consider_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS1MinimalIntensityToConsider(f32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the MS1 relative intensity error.
    pub fn ms1_relative_intensity_error_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS1RelativeIntensityError(f32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the noise threshold settings intensity threshold.
    pub fn noise_threshold_settings_intensity_threshold_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::NoiseThresholdSettingsIntensityThreshold(f32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the noise threshold settings maximal number of peaks.
    pub fn noise_threshold_settings_maximal_number_of_peaks_default(
        mut self,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::NoiseThresholdSettingsMaximalNumberOfPeaks(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Wheter to set to default the clustering of compounds before running zodiac.
    pub fn zodiac_cluster_compounds_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacClusterCompounds(bool::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the zodiac edge filter thresholds min local candidates.
    pub fn zodiac_edge_filter_thresholds_min_local_candidates_default(
        mut self,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacEdgeFilterThresholdsMinLocalCandidates(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the zodiac edge filter thresholds min local connections.
    pub fn zodiac_edge_filter_thresholds_min_local_connections_default(
        mut self,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacEdgeFilterThresholdsMinLocalConnections(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the zodiac edge filter thresholds theshold filter.
    pub fn zodiac_edge_filter_thresholds_threshold_filter_default(
        mut self,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacEdgeFilterThresholdsThresholdFilter(f32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the zodiac epochs burn in period.
    pub fn zodiac_epochs_burn_in_period_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacEpochsBurnInPeriod(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the zodiac epochs iterations.
    pub fn zodiac_epochs_iterations_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacEpochsIterations(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the zodiac epochs number of markov chains.
    pub fn zodiac_epochs_number_of_markov_chains_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacEpochsNumberOfMarkovChains(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Sdet to default the zodiac library scoring lambda.
    pub fn zodiac_library_scoring_lambda_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacLibraryScoringLambda(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the zodiac library scoring min cosine.
    pub fn zodiac_library_scoring_min_cosine_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacLibraryScoringMinCosine(f32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the number of considered candidates at 300 mz.
    pub fn zodiac_number_of_considered_candidates_at_300_mz_default(
        mut self,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacNumberOfConsideredCandidatesAt300Mz(i32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the number of considered candidates at 800 mz.
    pub fn zodiac_number_of_considered_candidates_at_800_mz_default(
        mut self,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacNumberOfConsideredCandidatesAt800Mz(i32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the ratio of considered candidates per ionization.
    pub fn zodiac_ratio_of_considered_candidates_per_ionization_default(
        mut self,
    ) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ZodiacRatioOfConsideredCandidatesPerIonization(f32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Whether to set to default the run in two steps.
    pub fn zodiac_run_in_two_steps_default(mut self) -> Result<Self, String> {
        self.config
            .add_config_parameter(ConfigV5::ZodiacRunInTwoSteps(bool::default()).into_default())?;
        Ok(self)
    }

    /// Set to default the allowed mass deviation for MS1 spectra.
    pub fn ms1_mass_deviation_allowed_mass_deviation_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS1MassDeviationAllowedMassDeviation(
                MassDeviation::Ppm(f32::default()).must_be_positive()?,
            )
            .into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the mass difference deviation for MS1 spectra.
    pub fn ms1_mass_deviation_mass_difference_deviation_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS1MassDeviationMassDifferenceDeviation(
                MassDeviation::Ppm(f32::default()).must_be_positive()?,
            )
            .into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the standard mass deviation for MS1 spectra.
    pub fn ms1_mass_deviation_standard_mass_deviation_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS1MassDeviationStandardMassDeviation(
                MassDeviation::Ppm(f32::default()).must_be_positive()?,
            )
            .into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the standard mass deviation for MS2 spectra.
    pub fn ms2_mass_deviation_standard_mass_deviation_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS2MassDeviationStandardMassDeviation(
                MassDeviation::Ppm(f32::default()).must_be_positive()?,
            )
            .into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the mass accuracy setting for MS2 spectra.
    pub fn ms2_mass_deviation_allowed_mass_deviation_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::MS2MassDeviationAllowedMassDeviation(
                MassDeviation::Ppm(f32::default()).must_be_positive()?,
            )
            .into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the detectable elements.
    pub fn formula_settings_detectable_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::FormulaSettingsDetectable(AtomVector::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the enforced elements.
    pub fn formula_settings_enforced_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::FormulaSettingsEnforced(AtomVector::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the fallback elements.
    pub fn formula_settings_fallback_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::FormulaSettingsFallback(AtomVector::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the forbid recalibration.
    pub fn forbid_recalibration_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::ForbidRecalibration(ForbidRecalibration::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the minimum m/z to enable heuristic preprocessing.
    pub fn use_heuristic_mz_to_use_heuristic_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::UseHeuristicMZToUseHeuristic(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the minimum m/z to only use heuristic tree computation.
    pub fn use_heuristic_mz_to_use_heuristic_only_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::UseHeuristicMZToUseHeuristicOnly(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the detectable adducts.
    pub fn adduct_settings_detectable_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::AdductSettingsDetectable(AdductsVector::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the fallback adducts.
    pub fn adduct_settings_fallback_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::AdductSettingsFallback(AdductsVector::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the algorithm profile.
    pub fn algorithm_profile_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::AlgorithmProfile(Instruments::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the compound quality.
    pub fn compound_quality_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::CompoundQuality(CompoundQuality::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the enforced adducts.
    pub fn adduct_settings_enforced_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::AdductSettingsEnforced(AdductSettingsEnforced::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the candidate formulas.
    pub fn candidate_formulas_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::CandidateFormulas(CandidateFormulas::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the formula result ranking score.
    pub fn formula_result_ranking_score_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::FormulaResultRankingScore(FormulaResultRankingScore::default())
                .into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the isotope ms2 settings.
    pub fn isotope_ms2_settings_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::IsotopeMS2Settings(IsotopeMS2Settings::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the isotope settings multiplier.
    pub fn isotope_settings_multiplier_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::IsotopeSettingsMultiplier(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the noise threshold settings absolute threshold.
    pub fn noise_threshold_settings_absolute_threshold_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::NoiseThresholdSettingsAbsoluteThreshold(u32::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the noise threshold settings base peak.
    pub fn noise_threshold_settings_base_peak_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::NoiseThresholdSettingsBasePeak(BasePeak::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the structure predictors algorithm.
    pub fn structure_predictors_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::StructurePredictors(StructurePredictors::default()).into_default(),
        )?;
        Ok(self)
    }

    /// Set to default the possible adduct switches.
    pub fn possible_adduct_switches_default(mut self) -> Result<Self, String> {
        self.config.add_config_parameter(
            ConfigV5::PossibleAdductSwitches(PossibleAdductSwitches::default()).into_default(),
        )?;
        Ok(self)
    }
}
