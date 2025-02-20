//! Submodule defining common errors for the graph crate.

pub mod algorithms;
pub mod graph;
pub mod nodes;

use algorithms::AlgorithmError;
use graph::GraphError;
use nodes::NodeError;

use crate::traits::Graph;

/// Errors that may occur when working with graphs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error<G: Graph + ?Sized> {
    /// Error relative to graphs.
    GraphError(GraphError<G>),
    /// Error relative to nodes.
    NodeError(NodeError<G>),
    /// Error relative to algorithms.
    AlgorithmError(AlgorithmError),
}
