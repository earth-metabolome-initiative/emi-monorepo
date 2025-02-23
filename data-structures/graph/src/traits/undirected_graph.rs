//! Subtype of directed graph where the edges are undirected.

use algebra::prelude::*;

use crate::prelude::*;

/// Trait defining the properties of a directed graph.
pub trait UndirectedEdges:
    TransposedDirectedEdges<BiMatrix = <Self as UndirectedEdges>::SymmetricMatrix>
{
    /// Neighbors of a node.
    type SymmetricMatrix: SparseSymmetricMatrix2D<Index = Self::NodeId>;

    /// Returns the neighbors of the node with the given identifier.
    fn neighbors(
        &self,
        node: Self::NodeId,
    ) -> <Self::SymmetricMatrix as SparseMatrix2D>::SparseRow<'_> {
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
}

/// Trait defining the coversion of directed edges to undirected edges.
pub trait FromDirectedEdges<DE: DirectedEdges>: UndirectedEdges
where
    DE::Matrix: Symmetrize<Self::SymmetricMatrix>,
{
    /// Converts the directed edges to undirected edges.
    fn from_directed_edges(edges: DE) -> Self;
}

/// Subtype of directed graph where the edges are undirected.
pub trait UndirectedGraph:
    super::DirectedGraph<DirectedEdges = <Self as UndirectedGraph>::UndirectedEdges>
{
    /// The undirected edges of the graph.
    type UndirectedEdges: UndirectedEdges<NodeId = Self::NodeId, Edge = Self::Edge>;

    /// Returns the neighbors of the node with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `id`: The identifier of the node.
    ///
    fn neighbors(&self, id: Self::NodeId) -> <<Self::UndirectedEdges as UndirectedEdges>::SymmetricMatrix as SparseMatrix2D>::SparseRow<'_>{
        self.edges().neighbors(id)
    }

    /// Returns the degree of the node with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `id`: The identifier of the node.
    ///
    fn degree(&self, id: Self::NodeId) -> Self::NodeId {
        self.edges().degree(id)
    }
}
