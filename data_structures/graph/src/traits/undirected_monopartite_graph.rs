//! Traits for monopartite undirected graphs.
//!
//! These graphs have the following properties:
//!
//! * All nodes are of the same type.
//! * All edges are bidirectional.

use algebra::prelude::{
    SizedRowsSparseMatrix2D, SizedSparseBiMatrix2D, SparseMatrix2D, SparseSymmetricMatrix2D,
};

use super::{MonopartiteEdges, MonoplexMonopartiteGraph, TransposedEdges};
use crate::traits::TransposedMonoplexGraph;

/// Trait defining the properties of a directed graph.
pub trait UndirectedMonopartiteEdges:
    MonopartiteEdges<
        MonopartiteMatrix = <Self as UndirectedMonopartiteEdges>::SymmetricSquaredMatrix,
    > + TransposedEdges<BiMatrix = <Self as UndirectedMonopartiteEdges>::SymmetricSquaredMatrix>
{
    /// Neighbors of a node.
    type SymmetricSquaredMatrix: SparseSymmetricMatrix2D<Index = Self::NodeId>
        + SizedSparseBiMatrix2D;

    /// Returns the neighbors of the node with the given identifier.
    fn neighbors(
        &self,
        node: Self::NodeId,
    ) -> <Self::SymmetricSquaredMatrix as SparseMatrix2D>::SparseRow<'_> {
        self.matrix().sparse_row(node)
    }

    /// Returns the degree of the node with the given identifier.
    fn degree(&self, node: Self::NodeId) -> Self::NodeId {
        debug_assert_eq!(
            self.matrix().number_of_defined_values_in_row(node),
            self.matrix().number_of_defined_values_in_column(node)
        );
        self.matrix().number_of_defined_values_in_row(node)
    }

    /// Returns the iterator over the degrees of the nodes in the graph.
    fn degrees(
        &self,
    ) -> <Self::SymmetricSquaredMatrix as SizedRowsSparseMatrix2D>::SparseRowSizes<'_> {
        self.matrix().sparse_row_sizes()
    }
}

impl<E> UndirectedMonopartiteEdges for E
where
    E: TransposedEdges + MonopartiteEdges<MonopartiteMatrix = E::BiMatrix>,
    E::BiMatrix: SparseSymmetricMatrix2D<Index = E::NodeId, SparseIndex = E::EdgeId>
        + SizedSparseBiMatrix2D<
            SizedSparseMatrix = <E::BiMatrix as SparseSymmetricMatrix2D>::SymmetricSparseMatrix,
            SizedSparseTransposedMatrix = <E::BiMatrix as SparseSymmetricMatrix2D>::SymmetricSparseMatrix,
        >,
{
    type SymmetricSquaredMatrix = E::BiMatrix;
}

/// Trait defining the properties of monopartite undirected graphs.
pub trait UndirectedMonopartiteMonoplexGraph: MonoplexMonopartiteGraph<
    MonoplexMonopartiteEdges = <Self as UndirectedMonopartiteMonoplexGraph>::UndirectedMonopartiteEdges,
> + TransposedMonoplexGraph<
    TransposedEdges = <Self as UndirectedMonopartiteMonoplexGraph>::UndirectedMonopartiteEdges,
> {
    /// The undirected edges of the graph.
    type UndirectedMonopartiteEdges: UndirectedMonopartiteEdges;

    /// Returns the neighbors of the node with the given identifier.
    fn neighbors(
        &self,
        node: Self::NodeId,
    ) -> <<Self::UndirectedMonopartiteEdges as UndirectedMonopartiteEdges>::SymmetricSquaredMatrix as SparseMatrix2D>::SparseRow<'_>{
        self.edges().neighbors(node)
    }

    /// Returns the degree of the node with the given identifier.
    fn degree(&self, node: Self::NodeId) -> Self::NodeId {
        self.edges().degree(node)
    }

    /// Returns the iterator over the degrees of the nodes in the graph.
    fn degrees(&self) -> <<Self::UndirectedMonopartiteEdges as UndirectedMonopartiteEdges>::SymmetricSquaredMatrix as SizedRowsSparseMatrix2D>::SparseRowSizes<'_>{
        self.edges().degrees()
    }
}

impl<G> UndirectedMonopartiteMonoplexGraph for G
where
    G: MonoplexMonopartiteGraph
        + TransposedMonoplexGraph<TransposedEdges = G::MonoplexMonopartiteEdges>,
    G::MonoplexMonopartiteEdges: UndirectedMonopartiteEdges,
{
    type UndirectedMonopartiteEdges = G::MonoplexMonopartiteEdges;
}
