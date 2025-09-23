//! Submodule providing `Resnik` Errors for working with Resnik

use std::fmt::Display;

use algebra::prelude::KahnError;


/// Resnik Enum for Errors that may occur during Resnik process
#[derive(Debug, PartialEq)]
pub enum ResnikError {
    /// Error for when a graph is not a DAG / contains a cycle
    NotDag,
    /// Error for unexpected occurence size
    IneqOccurenceSize {
        /// The expected size for the uccurence
        expected: usize,
        /// The actual size found for the occurence
        found: usize,
    }
}

impl Display for ResnikError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotDag => write!(f,"The graph is not a DAG"),
            Self::IneqOccurenceSize { expected, found } => write!(f,"Received an occurence vector with {found} entries but expected {expected} entries")
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