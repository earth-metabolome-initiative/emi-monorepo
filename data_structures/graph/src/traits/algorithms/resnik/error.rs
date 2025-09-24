//! Submodule providing `Resnik` Errors for working with Resnik

use std::fmt::Display;

use algebra::prelude::KahnError;


/// Resnik Enum for Errors that may occur during Resnik process
#[derive(Debug, PartialEq)]
pub enum ResnikError {
    /// Error for when a graph is not a DAG / contains a cycle
    NotDag,
    /// Error for unexpected occurrence size
    InequalOccurrenceSize {
        /// The expected size for the uccurence
        expected: usize,
        /// The actual size found for the occurrence
        found: usize,
    },
    /// Error for negative occurrences
    NegativeOccurrence,
    /// Error for non finite occurrences (infinite and NaN)
    NonFiniteOccurrence,
}

impl Display for ResnikError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotDag => write!(f,"The graph is not a DAG"),
            Self::InequalOccurrenceSize { expected, found } => write!(f,"Received an occurrence vector with {found} entries but expected {expected} entries"),
            Self::NegativeOccurrence => write!(f, "Occurrences must never be negative - negative occurrence found"),
            Self::NonFiniteOccurrence => write!(f, "Non Finite Occurrence found - occurrences must be finite"),
        }
    }
}

impl core::error::Error for ResnikError {
    
}

impl From <KahnError> for ResnikError {
    fn from(value: KahnError) -> Self {
        match value {
            KahnError::Cycle => Self::NotDag,
        }
    }
}