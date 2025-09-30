//! Submodule providing `Information Content` Errors for working with IC based
//! Algorithms
use std::fmt::Display;

use algebra::prelude::KahnError;

/// Information Content Enum for Errors that may occur during IC calculation
/// process
#[derive(Debug, PartialEq)]
pub enum InformationContentError {
    /// Error for when a graph is not a DAG / contains a cycle
    NotDag,
    /// Error for unexpected occurrence size
    UnequalOccurrenceSize {
        /// The expected size for the occurrence
        expected: usize,
        /// The actual size found for the occurrence
        found: usize,
    },
    /// Sink Node found with 0 occurrence count
    SinkNodeZeroOccurrence,
}

impl Display for InformationContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotDag => write!(f, "The graph is not a DAG"),
            Self::UnequalOccurrenceSize { expected, found } => {
                write!(
                    f,
                    "Received an occurrence vector with {found} entries but expected {expected} entries"
                )
            }
            Self::SinkNodeZeroOccurrence => write!(f, "Found a Sink Node with a 0 Occurrence"),
        }
    }
}

impl core::error::Error for InformationContentError {}

impl From<KahnError> for InformationContentError {
    fn from(value: KahnError) -> Self {
        match value {
            KahnError::Cycle => Self::NotDag,
        }
    }
}
