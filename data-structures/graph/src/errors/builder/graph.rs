//! Error enumeration for the graph builder.

use core::fmt::Debug;

use crate::traits::{Edge, GraphBuilderOptions};

#[derive(Clone)]
/// Enum representing the possible errors that can occur when building a graph.
pub enum GraphBuilderError<G: crate::traits::Graph + ?Sized> {
    /// Error that occurs when building a graph.
    BuilderError(common_traits::builder::BuilderError<GraphBuilderOptions>),
    /// An edge is duplicated in a non-multigraph.
    DuplicatedEdgeError(G::Edge),
}

impl<G: crate::traits::Graph + ?Sized> Debug for GraphBuilderError<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}

impl<G: crate::traits::Graph + ?Sized> From<common_traits::builder::BuilderError<GraphBuilderOptions>>
    for GraphBuilderError<G>
{
    fn from(e: common_traits::builder::BuilderError<GraphBuilderOptions>) -> Self {
        GraphBuilderError::BuilderError(e)
    }
}

impl<G: crate::traits::Graph + ?Sized> core::error::Error for GraphBuilderError<G> {}

impl<G: crate::traits::Graph + ?Sized> core::fmt::Display for GraphBuilderError<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            GraphBuilderError::BuilderError(e) => write!(f, "{e}"),
            GraphBuilderError::DuplicatedEdgeError(e) => {
                write!(f, "Duplicated edge: (src: {}, dst: {})", e.source(), e.destination())
            }
        }
    }
}
