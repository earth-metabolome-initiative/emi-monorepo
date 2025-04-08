//! Submodule defining the trait for a undirected monopartite edges builder.

use super::DirectedMonopartiteEdgesBuilder;
use crate::traits::UndirectedMonopartiteEdges;

/// Trait for Options for building edges.
pub trait UndirectedMonopartiteEdgesBuilder:
    DirectedMonopartiteEdgesBuilder<
    DirectedEdges = <Self as UndirectedMonopartiteEdgesBuilder>::UndirectedMonopartiteEdges,
>
{
    /// The type of the edges being built.
    type UndirectedMonopartiteEdges: UndirectedMonopartiteEdges;
}
