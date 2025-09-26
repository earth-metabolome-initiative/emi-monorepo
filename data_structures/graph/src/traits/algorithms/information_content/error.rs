//! Submodule providing `Information Content` Errors for working with IC based
//! Algorithms

use std::fmt::Display;

/// Information Content Enum for Errors that may occur during IC calculation
/// process
#[derive(Debug, PartialEq)]
pub enum InformationContentError {
    /// Error for unexpected occurrence size
    UnequalOccurrenceSize {
        /// The expected size for the occurrence
        expected: usize,
        /// The actual size found for the occurrence
        found: usize,
    },
    /// Error for negative occurrences
    NegativeOccurrence,
    /// Error for non finite occurrences (infinite and NaN)
    NonFiniteOccurrence,
    /// All occurrences are 0
    NoOccurrencesAboveZero,
}

impl Display for InformationContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnequalOccurrenceSize { expected, found } => {
                write!(
                    f,
                    "Received an occurrence vector with {found} entries but expected {expected} entries"
                )
            }
            Self::NegativeOccurrence => {
                write!(f, "Occurrences must never be negative - negative occurrence found")
            }
            Self::NonFiniteOccurrence => {
                write!(f, "Non Finite Occurrence found - occurrences must be finite")
            }
            Self::NoOccurrencesAboveZero => write!(f, "None of the occurrences had values above 0"),
        }
    }
}

impl core::error::Error for InformationContentError {}
