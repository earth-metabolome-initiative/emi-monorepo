//! Errors raised in algorithms defined for [`BipartiteGraph`]s.

use crate::traits::{weighted_assignment::hungarian_algorithm::HungarianAlgorithmError, BipartiteGraph};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Errors that may occur when executing algorithms on a [`BipartiteGraph`].
pub enum BipartiteAlgorithmError {
    /// Error raised while executing the `HungarianAlgorithm`.
    HungarianAlgorithm(HungarianAlgorithmError),
}

impl<G: BipartiteGraph> From<BipartiteAlgorithmError> for crate::errors::BipartiteError<G> {
    fn from(error: BipartiteAlgorithmError) -> Self {
        Self::AlgorithmError(error)
    }
}

impl core::fmt::Display for BipartiteAlgorithmError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            BipartiteAlgorithmError::HungarianAlgorithm(e) => write!(f, "{e}"),
        }
    }
}
