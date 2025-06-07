//! Submodule providing the traits for a generic graph that has weighted edges.

use algebra::prelude::{SparseValuedMatrix2D, ValuedSizedSparseBiMatrix2D};

use super::{
    Edges, TransposedEdges, TransposedMonoplexGraph, WeightedEdges, WeightedMonoplexGraph,
};

/// Trait defining an edge data structure that has weighted edges.
pub trait TransposedWeightedEdges:
    TransposedEdges<BiMatrix = <Self as TransposedWeightedEdges>::WeightedBiMatrix>
    + WeightedEdges<WeightedMatrix = <Self as TransposedWeightedEdges>::WeightedBiMatrix>
{
    /// The type of the underlying matrix.
    type WeightedBiMatrix: ValuedSizedSparseBiMatrix2D<
            Value = <Self as WeightedEdges>::Weight,
            RowIndex = Self::SourceNodeId,
            ColumnIndex = Self::DestinationNodeId,
            SparseIndex = Self::EdgeId,
        >;

    /// Returns the weights of the predecessors of a node.
    ///
    /// # Arguments
    ///
    /// * `destination_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The weights of the predecessors of the node.
    fn predecessor_weights(
        &self,
        destination_node_id: Self::DestinationNodeId,
    ) ->  <<Self::WeightedBiMatrix as ValuedSizedSparseBiMatrix2D>::ValuedSparseTransposedMatrix as SparseValuedMatrix2D>::SparseRowValues<'_>{
        self.matrix().sparse_column_values(destination_node_id)
    }

    /// Returns the largest weight of the predecessors of a node.
    ///
    /// # Arguments
    ///
    /// * `destination_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The largest weight of the predecessors of the node, if any.
    fn max_predecessor_weight(
        &self,
        destination_node_id: Self::DestinationNodeId,
    ) -> Option<Self::Weight> {
        self.matrix().sparse_column_max_value(destination_node_id)
    }

    /// Returns the smallest weight of the predecessors of a node.
    ///
    /// # Arguments
    ///
    /// * `destination_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The smallest weight of the predecessors of the node, if any.
    fn min_predecessor_weight(
        &self,
        destination_node_id: Self::DestinationNodeId,
    ) -> Option<Self::Weight> {
        self.matrix().sparse_column_min_value(destination_node_id)
    }
}

impl<E> TransposedWeightedEdges for E
where
    E: WeightedEdges,
    E::WeightedMatrix: ValuedSizedSparseBiMatrix2D<
            RowIndex = E::SourceNodeId,
            ColumnIndex = E::DestinationNodeId,
            SparseIndex = E::EdgeId,
            Value = E::Weight,
        >,
{
    type WeightedBiMatrix = E::WeightedMatrix;
}

/// Trait defining a graph that has weighted edges.
pub trait TransposedWeightedMonoplexGraph:
    TransposedMonoplexGraph<
        TransposedEdges = <Self as TransposedWeightedMonoplexGraph>::TransposedWeightedEdges,
    > + WeightedMonoplexGraph<
        WeightedEdges = <Self as TransposedWeightedMonoplexGraph>::TransposedWeightedEdges,
    >
{
    /// The type of the weighted edges.
    type TransposedWeightedEdges: TransposedWeightedEdges<
        Weight = <Self as WeightedMonoplexGraph>::Weight,
    >;

    /// Returns the weights of the predecessors of a node.
    ///
    /// # Arguments
    ///
    /// * `destination_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The weights of the predecessors of the node.
    fn predecessor_weights(
        &self,
        destination_node_id: <Self::TransposedWeightedEdges as Edges>::DestinationNodeId,
    ) -> <<<Self::TransposedWeightedEdges as TransposedWeightedEdges>::WeightedBiMatrix as ValuedSizedSparseBiMatrix2D>::ValuedSparseTransposedMatrix as SparseValuedMatrix2D>::SparseRowValues<'_>{
        self.edges().predecessor_weights(destination_node_id)
    }

    /// Returns the largest weight of the predecessors of a node.
    ///
    /// # Arguments
    ///
    /// * `destination_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The largest weight of the predecessors of the node, if any.
    fn max_predecessor_weight(
        &self,
        destination_node_id: <Self::TransposedWeightedEdges as Edges>::DestinationNodeId,
    ) -> Option<Self::Weight> {
        self.edges().max_predecessor_weight(destination_node_id)
    }

    /// Returns the smallest weight of the predecessors of a node.
    ///
    /// # Arguments
    ///
    /// * `destination_node_id`: The node identifier.
    ///
    /// # Returns
    ///
    /// The smallest weight of the predecessors of the node, if any.
    fn min_predecessor_weight(
        &self,
        destination_node_id: <Self::TransposedWeightedEdges as Edges>::DestinationNodeId,
    ) -> Option<Self::Weight> {
        self.edges().min_predecessor_weight(destination_node_id)
    }
}

impl<G> TransposedWeightedMonoplexGraph for G
where
    G: TransposedMonoplexGraph<TransposedEdges = <G as WeightedMonoplexGraph>::WeightedEdges>
        + WeightedMonoplexGraph,
    G::WeightedEdges: TransposedWeightedEdges,
{
    type TransposedWeightedEdges = G::WeightedEdges;
}
