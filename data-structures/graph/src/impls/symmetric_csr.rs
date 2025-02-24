//! Submodule implementing Edges-related traits for [`SquaredSymmetricCSR2D`].

use crate::prelude::*;
use algebra::prelude::*;

impl<
        SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
        Idx: PositiveInteger + TryFromUsize + IntoUsize + TryFrom<SparseIndex>,
    > Edges for SymmetricCSR2D<SparseIndex, Idx>
{
    type Edge = <Self as Matrix>::Coordinates;
    type SourceNodeId = Idx;
    type DestinationNodeId = Idx;
    type EdgeId = SparseIndex;
    type Matrix = Self;

    fn matrix(&self) -> &Self::Matrix {
        self
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
        Idx: PositiveInteger + TryFromUsize + IntoUsize + TryFrom<SparseIndex>,
        DE: DirectedEdges<NodeId = Idx>,
    > FromDirectedEdges<DE> for SymmetricCSR2D<SparseIndex, Idx>
where
    DE::Matrix: Symmetrize<Self>,
{
    fn from_directed_edges(edges: DE) -> Self {
        edges.matrix().symmetrize()
    }
}
