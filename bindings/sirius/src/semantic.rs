use crate::sirius_types::SearchDB;
use crate::sirius_types::MassDeviation;

/// Version-agnostic core parameters
#[derive(Debug, Clone, PartialEq)]
pub enum CoreParam {
    /// Maximum precursor m/z filter
    MaxMz(f64),
}

/// Version-agnostic config parameters (subset; extend as needed)
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigParam {
    /// Enable/disable isotope filter
    IsotopeFilter(bool),

    /// Databases for formula search
    FormulaDb(SearchDB),

    /// Databases for structure search
    StructureDb(SearchDB),

    /// Number of candidates (global)
    NumberOfCandidates(u32),

    /// Number of candidates per ionization (V6 name), used for V5 too
    NumberOfCandidatesPerIonization(u32),

    /// Whether to use formula result threshold
    FormulaResultThreshold(bool),

    /// Median noise intensity
    MedianNoiseIntensity(f32),

    /// MS1 isotopic intensity settings
    /// MS1 isotopic intensity settings; set any subset via the options
    Ms1IsotopicIntensity {
        /// absolute intensity error
        abs: Option<f32>,
        /// minimal intensity to consider
        min: Option<f32>,
        /// relative intensity error
        rel: Option<f32>,
    },

    /// Heuristic thresholds
    /// Heuristic thresholds
    Heuristic {
        /// enable heuristic for compounds >= m/z
        use_above: Option<u32>,
        /// use only heuristic for compounds >= m/z
        only_above: Option<u32>,
    },

    /// Timeouts
    /// Timeout (seconds) per fragmentation tree
    TimeoutPerTree(u32),
    /// Timeout (seconds) per compound/instance
    TimeoutPerInstance(u32),

    /// MS1/MS2 mass deviations (allowed)
    /// MS1 allowed mass deviation
    Ms1Allowed(MassDeviation),
    /// MS2 allowed mass deviation
    Ms2Allowed(MassDeviation),

    /// Raw pass-through flag (emitted as-is). Use sparingly for version-specific features.
    Raw(String),

    /// SpectralSearchDB: list of spectral DBs (V6+)
    SpectralSearchDb(Vec<SearchDB>),

    /// FormulaSearchDB list (replaces single)
    FormulaDbList(Vec<SearchDB>),

    /// StructureSearchDB list (replaces single)
    StructureDbList(Vec<SearchDB>),

    /// IdentitySearchSettings.precursorDeviation (V6+)
    IdentitySearchPrecursorDeviation(MassDeviation),

    /// FormulaSearchSettings.performBottomUpAboveMz (V6+)
    FormulaSearchPerformBottomUpAboveMz(u32),
}
