//! Errors that may occur when working with monopartite graphs.

use super::nodes::NodeError;
use crate::traits::MonopartiteGraph;

pub mod algorithms;
pub use algorithms::MonopartiteAlgorithmError;
pub mod illegal_graph_states;
pub use illegal_graph_states::IllegalMonopartiteGraphState;

/// Errors that may occur when working with graphs.
pub enum MonopartiteError<G: MonopartiteGraph + ?Sized> {
    /// Error relative to graphs.
    IllegalGraphState(IllegalMonopartiteGraphState<G>),
    /// Error relative to nodes.
    NodeError(NodeError<G::Nodes>),
    /// Error relative to algorithms.
    AlgorithmError(MonopartiteAlgorithmError),
}

impl<G: MonopartiteGraph + ?Sized> core::error::Error for MonopartiteError<G> {}

impl<G: MonopartiteGraph + ?Sized> core::fmt::Debug for MonopartiteError<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}

impl<G: MonopartiteGraph + ?Sized> core::fmt::Display for MonopartiteError<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::IllegalGraphState(e) => write!(f, "{e}"),
            Self::NodeError(e) => write!(f, "{e}"),
            Self::AlgorithmError(e) => write!(f, "{e}"),
        }
    }
}
