//! Errors raised in the algorithms module.

use crate::traits::Graph;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Errors that may occur when working with algorithms.
pub enum AlgorithmError {
    /// Error raised while computing connected components.
    ConnectedComponentsError(
        crate::traits::algorithms::connected_components::ConnectedComponentsError,
    ),
}

impl<G: Graph> From<AlgorithmError> for crate::errors::Error<G> {
    fn from(error: AlgorithmError) -> Self {
        crate::errors::Error::AlgorithmError(error)
    }
}

impl core::fmt::Display for AlgorithmError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            AlgorithmError::ConnectedComponentsError(e) => write!(f, "{e}"),
        }
    }
}
