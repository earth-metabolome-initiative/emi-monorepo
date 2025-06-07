//! Errors that may occur when working with bipartite graphs.

use super::nodes::NodeError;
use crate::traits::BipartiteGraph;

pub mod algorithms;
pub use algorithms::BipartiteAlgorithmError;
pub mod illegal_graph_states;
pub use illegal_graph_states::IllegalBipartiteGraphState;

/// Errors that may occur when working with graphs.
pub enum BipartiteError<G: BipartiteGraph + ?Sized> {
    /// Error relative to graphs.
    IllegalGraphState(IllegalBipartiteGraphState<G>),
    /// Error relative to left nodes partition.
    LeftNodeError(NodeError<G::LeftNodes>),
    /// Error relative to right nodes partition.
    RightNodeError(NodeError<G::RightNodes>),
    /// Error relative to algorithms.
    AlgorithmError(BipartiteAlgorithmError),
}

impl<G: BipartiteGraph + ?Sized> core::error::Error for BipartiteError<G> {}

impl<G: BipartiteGraph + ?Sized> core::fmt::Debug for BipartiteError<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}

impl<G: BipartiteGraph + ?Sized> core::fmt::Display for BipartiteError<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::IllegalGraphState(e) => write!(f, "{e}"),
            Self::LeftNodeError(e) => write!(f, "{e}"),
            Self::RightNodeError(e) => write!(f, "{e}"),
            Self::AlgorithmError(e) => write!(f, "{e}"),
        }
    }
}
