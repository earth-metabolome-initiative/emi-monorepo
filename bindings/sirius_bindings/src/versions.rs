//! Trait and structs to define the different versions of Sirius
use core::fmt::Debug;

use crate::traits::{Enablable, IntoDefault, NamedParametersSet};

/// Trait for the different versions of Sirius
pub trait Version: Default {
    /// The version number
    const VERSION: usize;

    /// The core parameters
    ///
    /// # Errors
    /// This method does not return a `Result`, so no errors are expected.
    type Core: ToString + Debug + IntoDefault;

    /// The config parameters
    ///
    /// # Errors
    /// This method does not return a `Result`, so no errors are expected.
    type Config: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;

    /// The Canopus parameters
    ///
    /// # Errors
    /// This method does not return a `Result`, so no errors are expected.
    type Canopus: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;

    /// The Formula parameters
    ///
    /// # Errors
    /// This method does not return a `Result`, so no errors are expected.
    type Formula: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;

    /// The Zodiac parameters
    ///
    /// # Errors
    /// This method does not return a `Result`, so no errors are expected.
    type Zodiac: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;

    /// The Fingerprint parameters
    ///
    /// # Errors
    /// This method does not return a `Result`, so no errors are expected.
    type Fingerprint: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;

    /// The Structure parameters
    ///
    /// # Errors
    /// This method does not return a `Result`, so no errors are expected.
    type Structure: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;

    /// Whether to write summaries
    ///
    /// # Errors
    /// This method does not return a `Result`, so no errors are expected.
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
