//! Submodule for the transposed directed graph traits.

use algebra::prelude::*;

use super::{DirectedEdges, TransposedEdges};

/// Trait defining the properties of a transposed directed graph.
pub trait TransposedDirectedEdges:
    TransposedEdges<BiMatrix = <Self as TransposedDirectedEdges>::DirectedBiMatrix> + DirectedEdges
{
    /// The type of matrix required to store the transposed edges.
    type DirectedBiMatrix: SparseBiMatrix2D<RowIndex = Self::SourceNodeId, ColumnIndex = Self::DestinationNodeId>;
}

impl<E> TransposedDirectedEdges for E
where
    E: DirectedEdges + TransposedEdges,
    E::BiMatrix: SparseBiMatrix2D<RowIndex = E::SourceNodeId, ColumnIndex = E::DestinationNodeId>,
{
    type DirectedBiMatrix = E::BiMatrix;
}

/// Trait defining the properties of a directed graph.
pub trait TransposedDirectedGraph: super::DirectedGraph {}

impl<G> TransposedDirectedGraph for G where G: super::DirectedGraph {}
