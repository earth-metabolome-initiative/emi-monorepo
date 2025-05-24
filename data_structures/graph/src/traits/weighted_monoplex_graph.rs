//! Submodule providing the traits for a generic graph that has weighted edges.

use algebra::prelude::{SparseValuedMatrix, SparseValuedMatrix2D};
use numeric_common_traits::prelude::Number;

use super::{AttributedEdge, Edges, MonoplexGraph};

/// Trait defining a weighted edge.
pub trait WeightedEdge: AttributedEdge<Attribute = Self::Weight> {
    /// Type of the weight.
    type Weight: Number;

    /// Returns the weight of the edge.
    fn weight(&self) -> Self::Weight;
}

impl<E> WeightedEdge for E
where
    E: AttributedEdge,
    E::Attribute: Number,
{
    type Weight = E::Attribute;

    fn weight(&self) -> Self::Weight {
        self.attribute()
    }
}

/// Trait defining an edge data structure that has weighted edges.
pub trait WeightedEdges:
    Edges<
        Edge = <Self as WeightedEdges>::WeightedEdge,
        Matrix = <Self as WeightedEdges>::WeightedMatrix,
    >
{
    /// The type of the weight.
    type Weight: Number;
    /// The type of the weighted edge.
    type WeightedEdge: WeightedEdge<
            Weight = Self::Weight,
            SourceNodeId = Self::SourceNodeId,
            DestinationNodeId = Self::DestinationNodeId,
        >;
    /// The type of the underlying matrix.
    type WeightedMatrix: SparseValuedMatrix2D<
            Value = Self::Weight,
            RowIndex = Self::SourceNodeId,
            ColumnIndex = Self::DestinationNodeId,
            SparseIndex = Self::EdgeId,
        >;

    /// Returns the weights of the successors of a node.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The weights of the successors of the node.
    fn successor_weights(
        &self,
        source_node_id: Self::SourceNodeId,
    ) -> <Self::WeightedMatrix as SparseValuedMatrix2D>::SparseRowValues<'_> {
        self.matrix().sparse_row_values(source_node_id)
    }

    /// Returns the largest weight of the successors of a node.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The largest weight of the successors of the node, if any.
    fn max_successor_weight(&self, source_node_id: Self::SourceNodeId) -> Option<Self::Weight> {
        self.matrix().sparse_row_max_value(source_node_id)
    }

    /// Returns the largest weight of the successors of a node and the
    /// corresponding successor node identifier.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The largest weight of the successors of the node and the corresponding
    /// successor node identifier, if any.
    fn max_successor_weight_and_id(
        &self,
        source_node_id: Self::SourceNodeId,
    ) -> Option<(Self::Weight, Self::DestinationNodeId)> {
        self.matrix().sparse_row_max_value_and_column(source_node_id)
    }

    /// Returns the smallest weight of the successors of a node.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The smallest weight of the successors of the node, if any.
    fn min_successor_weight(&self, source_node_id: Self::SourceNodeId) -> Option<Self::Weight> {
        self.matrix().sparse_row_min_value(source_node_id)
    }

    /// Returns the smallest weight of the successors of a node and the
    /// corresponding successor node identifier.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The smallest weight of the successors of the node and the corresponding
    /// successor node identifier, if any.
    fn min_successor_weight_and_id(
        &self,
        source_node_id: Self::SourceNodeId,
    ) -> Option<(Self::Weight, Self::DestinationNodeId)> {
        self.matrix().sparse_row_min_value_and_column(source_node_id)
    }

    /// Returns the sparse weights of the edges.
    fn sparse_weights(&self) -> <Self::WeightedMatrix as SparseValuedMatrix>::SparseValues<'_> {
        self.matrix().sparse_values()
    }
}

impl<E> WeightedEdges for E
where
    E: Edges,
    E::Edge: WeightedEdge,
    E::Matrix: SparseValuedMatrix2D<Value = <E::Edge as WeightedEdge>::Weight>,
{
    type Weight = <E::Edge as WeightedEdge>::Weight;
    type WeightedEdge = E::Edge;
    type WeightedMatrix = E::Matrix;
}

/// Trait defining a graph that has weighted edges.
pub trait WeightedMonoplexGraph:
    MonoplexGraph<
        Edges = <Self as WeightedMonoplexGraph>::WeightedEdges,
        Edge = <Self as WeightedMonoplexGraph>::WeightedEdge,
    >
{
    /// The type of the weight.
    type Weight: Number;
    /// The type of the weighted edge.
    type WeightedEdge: WeightedEdge<Weight = Self::Weight>;
    /// The type of the weighted edges.
    type WeightedEdges: WeightedEdges<Weight = Self::Weight, WeightedEdge = Self::WeightedEdge>;

    /// Returns the weights of the successors of a node.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The weights of the successors of the node.
    fn successor_weights(
        &self,
        source_node_id: <Self::WeightedEdges as Edges>::SourceNodeId,
    ) -> <<Self::WeightedEdges as WeightedEdges>::WeightedMatrix as SparseValuedMatrix2D>::SparseRowValues<'_>
    {
        self.edges().successor_weights(source_node_id)
    }

    /// Returns the largest weight of the successors of a node.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The largest weight of the successors of the node, if any.
    fn max_successor_weight(
        &self,
        source_node_id: <Self::WeightedEdges as Edges>::SourceNodeId,
    ) -> Option<Self::Weight> {
        self.edges().max_successor_weight(source_node_id)
    }

    /// Returns the largest weight of the successors of a node and the
    /// corresponding successor node identifier.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The largest weight of the successors of the node and the corresponding
    /// successor node identifier, if any.
    fn max_successor_weight_and_id(
        &self,
        source_node_id: <Self::WeightedEdges as Edges>::SourceNodeId,
    ) -> Option<(Self::Weight, <Self::WeightedEdges as Edges>::DestinationNodeId)> {
        self.edges().max_successor_weight_and_id(source_node_id)
    }

    /// Returns the smallest weight of the successors of a node.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The smallest weight of the successors of the node, if any.
    fn min_successor_weight(
        &self,
        source_node_id: <Self::WeightedEdges as Edges>::SourceNodeId,
    ) -> Option<Self::Weight> {
        self.edges().min_successor_weight(source_node_id)
    }

    /// Returns the smallest weight of the successors of a node and the
    /// corresponding successor node identifier.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The smallest weight of the successors of the node and the corresponding
    /// successor node identifier, if any.
    fn min_successor_weight_and_id(
        &self,
        source_node_id: <Self::WeightedEdges as Edges>::SourceNodeId,
    ) -> Option<(Self::Weight, <Self::WeightedEdges as Edges>::DestinationNodeId)> {
        self.edges().min_successor_weight_and_id(source_node_id)
    }

    /// Returns the sparse weights of the edges.
    fn sparse_weights(&self) -> <<Self::WeightedEdges as WeightedEdges>::WeightedMatrix as SparseValuedMatrix>::SparseValues<'_>{
        self.edges().sparse_weights()
    }
}

impl<G> WeightedMonoplexGraph for G
where
    G: MonoplexGraph,
    G::Edges: WeightedEdges<WeightedEdge = G::Edge, Weight = <G::Edge as WeightedEdge>::Weight>,
    G::Edge: WeightedEdge,
{
    type Weight = <G::Edges as WeightedEdges>::Weight;
    type WeightedEdges = G::Edges;
    type WeightedEdge = G::Edge;
}
