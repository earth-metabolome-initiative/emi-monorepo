//! Submodule providing errors associated with building a graph or a vocabulary.

pub mod edges;
pub mod graph;
pub mod vocabulary;

use crate::traits::Graph;

/// Enum representing the possible errors that can occur when building a graph.
pub enum BuilderError<G: Graph + ?Sized> {
    /// Error that occurs when building a graph.
    GraphBuilderError(graph::GraphBuilderError<G>),
    /// Error that occurs when building the edges of a graph.
    EdgesBuilderError(edges::EdgesBuilderError<G::Edges>),
}

impl<G: Graph + ?Sized> core::error::Error for BuilderError<G> {}

impl<G: Graph + ?Sized> core::fmt::Debug for BuilderError<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}

impl<G: Graph + ?Sized> core::fmt::Display for BuilderError<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            BuilderError::GraphBuilderError(e) => write!(f, "{e}"),
            BuilderError::EdgesBuilderError(e) => write!(f, "{e}"),
        }
    }
}
