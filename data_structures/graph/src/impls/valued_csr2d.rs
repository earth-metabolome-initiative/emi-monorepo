//! Submodule implementing Edges for
//! [`ValuedCSR2D`](algebra::prelude::ValuedCSR2D).

use algebra::prelude::*;
use multi_ranged::Step;
use numeric_common_traits::prelude::{IntoUsize, Number, PositiveInteger, TryFromUsize};

use crate::{errors::builder::edges::EdgesBuilderError, prelude::*};

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + TryFromUsize + IntoUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize + TryFrom<SparseIndex>,
    Value: Number,
> Edges for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
{
    type Edge = (RowIndex, ColumnIndex, Value);
    type SourceNodeId = RowIndex;
    type DestinationNodeId = ColumnIndex;
    type EdgeId = SparseIndex;
    type Matrix = Self;

    fn matrix(&self) -> &Self::Matrix {
        self
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + TryFromUsize + IntoUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize + TryFrom<SparseIndex>,
    Value: Number,
> GrowableEdges for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
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

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + TryFromUsize + IntoUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize + TryFrom<SparseIndex>,
    Value: Number,
> Graph for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
{
    fn has_nodes(&self) -> bool {
        self.number_of_rows() > RowIndex::ZERO && self.number_of_columns() > ColumnIndex::ZERO
    }

    fn has_edges(&self) -> bool {
        self.number_of_defined_values() > SparseIndex::ZERO
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + TryFromUsize + IntoUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize + TryFrom<SparseIndex>,
    Value: Number,
> MonoplexGraph for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
{
    type Edge = (RowIndex, ColumnIndex, Value);
    type Edges = Self;

    fn edges(&self) -> &Self::Edges {
        self
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step
        + PositiveInteger
        + TryFromUsize
        + IntoUsize
        + BidirectionalVocabulary<SourceSymbol = RowIndex, DestinationSymbol = RowIndex>,
    ColumnIndex: Step
        + PositiveInteger
        + IntoUsize
        + TryFrom<SparseIndex>
        + TryFromUsize
        + BidirectionalVocabulary<SourceSymbol = ColumnIndex, DestinationSymbol = ColumnIndex>,
    Value: Number,
> BipartiteGraph for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    Self: Matrix2DRef<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
{
    type LeftNodeId = RowIndex;
    type RightNodeId = ColumnIndex;
    type LeftNodeSymbol = RowIndex;
    type RightNodeSymbol = ColumnIndex;
    type LeftNodes = RowIndex;
    type RightNodes = ColumnIndex;

    fn left_nodes_vocabulary(&self) -> &Self::LeftNodes {
        self.number_of_rows_ref()
    }

    fn right_nodes_vocabulary(&self) -> &Self::RightNodes {
        self.number_of_columns_ref()
    }
}
