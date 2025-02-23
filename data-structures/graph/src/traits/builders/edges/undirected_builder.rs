//! Submodule defining the trait for a undirected edges builder.

use crate::traits::UndirectedEdges;

use super::DirectedEdgesBuilder;

/// Trait for Options for building edges.
pub trait UndirectedEdgesBuilder:
    DirectedEdgesBuilder<DirectedEdges = <Self as UndirectedEdgesBuilder>::UndirectedEdges>
{
    /// The type of the edges being built.
    type UndirectedEdges: UndirectedEdges;
}
