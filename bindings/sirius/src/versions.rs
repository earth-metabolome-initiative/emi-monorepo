use std::fmt::Debug;

use crate::traits::Enablable;
use crate::traits::IntoDefault;
use crate::traits::NamedParametersSet;

/// Trait for the different versions of Sirius
pub trait Version: Default {
    /// The version number
    const VERSION: usize;

    /// The core parameters
    type Core: ToString + Debug + IntoDefault;

    /// The config parameters
    type Config: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;

    /// The Canopus parameters
    type Canopus: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;

    /// The Formula parameters
    type Formula: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;

    /// The Zodiac parameters
    type Zodiac: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;

    /// The Fingerprint parameters
    type Fingerprint: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;

    /// The Structure parameters
    type Structure: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;

    /// Whether to write summaries
    type WriteSummaries: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;
}

/// Implementation of the Sirius version 5
#[derive(Default)]
pub struct Version5;

impl Version for Version5 {
    const VERSION: usize = 5;
    type Core = crate::parameters::core::CoreV5;
    type Config = crate::parameters::config::ConfigV5;
    type Canopus = crate::parameters::canopus::CanopusV5;
    type Formula = crate::parameters::formula::FormulaV5;
    type Zodiac = crate::parameters::zodiac::ZodiacV5;
    type Fingerprint = crate::parameters::fingerprint::FingerprintV5;
    type Structure = crate::parameters::structure::StructureV5;
    type WriteSummaries = crate::parameters::write_summaries::WriteSummariesV5;
}

/// Implementation of the Sirius version 6
#[derive(Default)]
pub struct Version6;

impl Version for Version6 {
    const VERSION: usize = 6;
    type Core = crate::parameters::core::CoreV6;
    type Config = crate::parameters::config::ConfigV6;
    // Reuse unchanged parameter sets from V5
    type Canopus = crate::parameters::canopus::CanopusV5;
    type Formula = crate::parameters::formula::FormulaV5;
    type Zodiac = crate::parameters::zodiac::ZodiacV5;
    type Fingerprint = crate::parameters::fingerprint::FingerprintV5;
    type Structure = crate::parameters::structure::StructureV5;
    type WriteSummaries = crate::parameters::write_summaries::WriteSummariesV5;
}
