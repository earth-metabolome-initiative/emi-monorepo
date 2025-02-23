//! Submodule defining common errors relative to graphs.

pub mod illegal_graph_states;

use crate::traits::Graph;

/// Error enumeration relative to nodes.
#[derive(Clone, PartialEq, Eq)]
pub enum GraphError<G: Graph + ?Sized> {
    /// The graph structure is in an illegal state.
    IllegalGraphState(illegal_graph_states::IllegalGraphState<G>),
}

impl<G: Graph + ?Sized> core::fmt::Display for GraphError<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            GraphError::IllegalGraphState(error) => write!(f, "{error}"),
        }
    }
}

impl<G: Graph + ?Sized> core::fmt::Debug for GraphError<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}

impl<G: Graph> core::error::Error for GraphError<G> {}

impl<G: Graph> From<GraphError<G>> for super::Error<G> {
    fn from(error: GraphError<G>) -> Self {
        super::Error::GraphError(error)
    }
}
