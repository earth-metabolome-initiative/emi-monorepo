//! Submodule defining common errors for the graph crate.

pub mod algorithms;
pub mod builder;
pub mod graph;
pub mod nodes;

use algorithms::AlgorithmError;
use builder::BuilderError;
use graph::GraphError;
use nodes::NodeError;

use crate::traits::Graph;

/// Errors that may occur when working with graphs.
pub enum Error<G: Graph + ?Sized> {
    /// Error relative to graphs.
    GraphError(GraphError<G>),
    /// Error relative to nodes.
    NodeError(NodeError<G>),
    /// Error relative to algorithms.
    AlgorithmError(AlgorithmError),
    /// Error relative to builders.
    BuilderError(BuilderError<G>),
}

impl<G: Graph + ?Sized> core::error::Error for Error<G> {}

impl<G: Graph + ?Sized> core::fmt::Debug for Error<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}

impl<G: Graph + ?Sized> core::fmt::Display for Error<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::GraphError(e) => write!(f, "{e}"),
            Error::NodeError(e) => write!(f, "{e}"),
            Error::AlgorithmError(e) => write!(f, "{e}"),
            Error::BuilderError(e) => write!(f, "{e}"),
        }
    }
}
