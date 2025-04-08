//! Submodule providing the [`FromDirectedMonopartiteEdges`] trait, which
//! defines a conversion from directed monopartite edges to undirected
//! monopartite edges.

use super::{MonopartiteEdges, UndirectedMonopartiteEdges};

/// Trait to convert directed monopartite edges into undirected monopartite
/// edges.
pub trait FromDirectedMonopartiteEdges<DE>: UndirectedMonopartiteEdges
where
    DE: MonopartiteEdges,
{
    /// Converts the directed monopartite edges into undirected monopartite
    /// edges.
    fn from_directed_edges(edges: DE) -> Self;
}
