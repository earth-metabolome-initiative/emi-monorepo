//! Errors raised in algorithms defined for [`MonopartiteGraph`]s.

use crate::traits::{MonopartiteGraph, connected_components::ConnectedComponentsError};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Errors that may occur when executing algorithms on a [`MonopartiteGraph`].
pub enum MonopartiteAlgorithmError {
    /// Error raised while computing connected components.
    ConnectedComponentsError(ConnectedComponentsError),
}

impl<G: MonopartiteGraph> From<MonopartiteAlgorithmError> for crate::errors::MonopartiteError<G> {
    fn from(error: MonopartiteAlgorithmError) -> Self {
        Self::AlgorithmError(error)
    }
}

impl core::fmt::Display for MonopartiteAlgorithmError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            MonopartiteAlgorithmError::ConnectedComponentsError(e) => write!(f, "{e}"),
        }
    }
}
