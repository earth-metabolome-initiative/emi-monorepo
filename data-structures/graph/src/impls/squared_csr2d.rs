//! Submodule implementing Edges-related traits for [`SquaredSquareCSR2D`].

use crate::{errors::builder::edges::EdgesBuilderError, prelude::*};
use algebra::prelude::*;

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        Idx: PositiveInteger + TryFromUsize + IntoUsize + From<SparseIndex>,
    > Edges for SquareCSR2D<SparseIndex, Idx>
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
        SparseIndex: PositiveInteger + IntoUsize,
        Idx: PositiveInteger + TryFromUsize + IntoUsize + From<SparseIndex>,
    > GrowableEdges for SquareCSR2D<SparseIndex, Idx>
{
    type GrowableMatrix = Self;
    type Error = EdgesBuilderError<Self>;

    fn matrix_mut(&mut self) -> &mut Self::GrowableMatrix {
        self
    }

    fn with_capacity(number_of_edges: Self::EdgeId) -> Self {
        <Self as SparseMatrixMut>::with_sparse_capacity(number_of_edges)
    }

    fn with_shaped_capacity(
        shape: <Self::GrowableMatrix as SparseMatrixMut>::MinimalShape,
        number_of_edges: Self::EdgeId,
    ) -> Self {
        <Self as SparseMatrixMut>::with_sparse_shaped_capacity(shape, number_of_edges)
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        Idx: PositiveInteger + TryFromUsize + IntoUsize + From<SparseIndex>,
    > DirectedEdges for SquareCSR2D<SparseIndex, Idx>
{
    type DirectedMatrix = Self;
    type NodeId = Idx;

    fn number_of_self_loops(&self) -> Idx {
        self.number_of_defined_diagonal_values()
    }
}
