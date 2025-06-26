//! Implementations of traits from the graph crate for the
//! [`RaggedVector`](algebra::prelude::RaggedVector) data structure.

use algebra::{
    impls::RaggedVector,
    prelude::{Matrix2D, Matrix2DRef, SizedSparseMatrix, SparseMatrix, SparseMatrixMut},
};
use multi_ranged::Step;
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use crate::{
    errors::builder::edges::EdgesBuilderError,
    traits::{BidirectionalVocabulary, BipartiteGraph, Edges, Graph, GrowableEdges, MonoplexGraph},
};

impl<SparseIndex, RowIndex, ColumnIndex> Edges for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    SparseIndex: TryFromUsize + IntoUsize + PositiveInteger,
{
    type Edge = (<Self as Matrix2D>::RowIndex, <Self as Matrix2D>::ColumnIndex);
    type SourceNodeId = <Self as Matrix2D>::RowIndex;
    type DestinationNodeId = <Self as Matrix2D>::ColumnIndex;
    type EdgeId = SparseIndex;
    type Matrix = Self;

    fn matrix(&self) -> &Self::Matrix {
        self
    }
}

impl<SparseIndex, RowIndex, ColumnIndex> GrowableEdges
    for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    SparseIndex: TryFromUsize + IntoUsize + PositiveInteger,
{
    type GrowableMatrix = Self;
    type Error = EdgesBuilderError<Self>;

    fn matrix_mut(&mut self) -> &mut Self::GrowableMatrix {
        self
    }

    fn with_capacity(number_of_edges: Self::EdgeId) -> Self {
        <Self as SparseMatrixMut>::with_sparse_capacity(number_of_edges)
    }

    fn with_shape(shape: <Self::GrowableMatrix as SparseMatrixMut>::MinimalShape) -> Self {
        <Self as SparseMatrixMut>::with_sparse_shape(shape)
    }

    fn with_shaped_capacity(
        shape: <Self::GrowableMatrix as SparseMatrixMut>::MinimalShape,
        number_of_edges: Self::EdgeId,
    ) -> Self {
        <Self as SparseMatrixMut>::with_sparse_shaped_capacity(shape, number_of_edges)
    }
}

impl<SparseIndex, RowIndex, ColumnIndex> Graph for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    SparseIndex: TryFromUsize + IntoUsize + PositiveInteger,
{
    fn has_nodes(&self) -> bool {
        self.number_of_rows() > <Self as Matrix2D>::RowIndex::ZERO
            && self.number_of_columns() > <Self as Matrix2D>::ColumnIndex::ZERO
    }

    fn has_edges(&self) -> bool {
        self.number_of_defined_values() > <Self as SparseMatrix>::SparseIndex::ZERO
    }
}

impl<SparseIndex, RowIndex, ColumnIndex> MonoplexGraph
    for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    SparseIndex: TryFromUsize + IntoUsize + PositiveInteger,
{
    type Edge = (<Self as Matrix2D>::RowIndex, <Self as Matrix2D>::ColumnIndex);
    type Edges = Self;

    fn edges(&self) -> &Self::Edges {
        self
    }
}

impl<SparseIndex, RowIndex, ColumnIndex> BipartiteGraph
    for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    RowIndex: Step
        + PositiveInteger
        + IntoUsize
        + TryFromUsize
        + BidirectionalVocabulary<
            SourceSymbol = <Self as Matrix2D>::RowIndex,
            DestinationSymbol = <Self as Matrix2D>::RowIndex,
        >,
    SparseIndex: TryFromUsize + IntoUsize + PositiveInteger,
    ColumnIndex: Step
        + PositiveInteger
        + IntoUsize
        + TryFromUsize
        + BidirectionalVocabulary<
            SourceSymbol = <Self as Matrix2D>::ColumnIndex,
            DestinationSymbol = <Self as Matrix2D>::ColumnIndex,
        >,
{
    type LeftNodeId = <Self as Matrix2D>::RowIndex;
    type RightNodeId = <Self as Matrix2D>::ColumnIndex;
    type LeftNodeSymbol = <Self as Matrix2D>::RowIndex;
    type RightNodeSymbol = <Self as Matrix2D>::ColumnIndex;
    type LeftNodes = <Self as Matrix2D>::RowIndex;
    type RightNodes = <Self as Matrix2D>::ColumnIndex;

    fn left_nodes_vocabulary(&self) -> &Self::LeftNodes {
        self.number_of_rows_ref()
    }

    fn right_nodes_vocabulary(&self) -> &Self::RightNodes {
        self.number_of_columns_ref()
    }
}
