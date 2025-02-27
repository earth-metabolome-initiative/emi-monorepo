//! Submodule defining the trait for a undirected edges builder.

use super::DirectedEdgesBuilder;
use crate::traits::UndirectedEdges;

/// Trait for Options for building edges.
pub trait UndirectedEdgesBuilder:
    DirectedEdgesBuilder<DirectedEdges = <Self as UndirectedEdgesBuilder>::UndirectedEdges>
{
    /// The type of the edges being built.
    type UndirectedEdges: UndirectedEdges;
}
