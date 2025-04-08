//! Enumeration for the errors associated with the CSR data structure.

use core::fmt::Debug;

use super::{SquareCSR2D, SymmetricCSR2D, UpperTriangularCSR2D, ValuedCSR2D, CSR2D};
use crate::traits::{IntoUsize, Matrix2D, PositiveInteger};

/// Enumeration for the errors associated with the CSR data structure.
pub enum Error<M: Matrix2D> {
    /// Mutability error.
    Mutability(MutabilityError<M>),
}

impl<M: Matrix2D> Debug for Error<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}

impl<M: Matrix2D> core::error::Error for Error<M> {}

impl<M: Matrix2D> core::fmt::Display for Error<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::Mutability(error) => write!(f, "Mutability error: {error}"),
        }
    }
}

/// Enumeration for the errors associated with failed mutable operations.
pub enum MutabilityError<M: Matrix2D + ?Sized> {
    /// Unexpected row index.
    UnorderedRowIndex(M::RowIndex),
    /// Unexpected column index.
    UnorderedColumnIndex(M::ColumnIndex),
    /// Duplicated entry.
    DuplicatedEntry(M::Coordinates),
    /// Entry out of bounds.
    OutOfBounds(M::Coordinates),
    /// When the row index type has been maxed out and it cannot
    /// be incremented anymore.
    MaxedOutRowIndex,
    /// When the column index type has been maxed out and it cannot
    /// be incremented anymore.
    MaxedOutColumnIndex,
    /// When the sparse index type has been maxed out and it cannot
    /// be incremented anymore.
    MaxedOutSparseIndex,
}

impl<M: Matrix2D> Debug for MutabilityError<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}

impl<M: Matrix2D> core::error::Error for MutabilityError<M> {}

impl<M: Matrix2D> From<MutabilityError<M>> for Error<M> {
    fn from(error: MutabilityError<M>) -> Self {
        Error::Mutability(error)
    }
}

impl<M: Matrix2D> core::fmt::Display for MutabilityError<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            MutabilityError::UnorderedRowIndex(index) => write!(f, "Unexpected row index: {index}"),
            MutabilityError::UnorderedColumnIndex(index) => {
                write!(f, "Unexpected column index: {index}")
            }
            MutabilityError::DuplicatedEntry(coordinates) => {
                write!(f, "Duplicated entry: {coordinates:?}")
            }
            MutabilityError::OutOfBounds(coordinates) => {
                write!(f, "Entry out of expected bounds: {coordinates:?}")
            }
            MutabilityError::MaxedOutRowIndex => {
                write!(f, "Row index type has been maxed out")
            }
            MutabilityError::MaxedOutColumnIndex => {
                write!(f, "Column index type has been maxed out")
            }
            MutabilityError::MaxedOutSparseIndex => {
                write!(f, "Sparse index type has been maxed out")
            }
        }
    }
}

impl<SparseIndex, Idx> From<MutabilityError<CSR2D<SparseIndex, Idx, Idx>>>
    for MutabilityError<UpperTriangularCSR2D<SparseIndex, Idx>>
where
    CSR2D<SparseIndex, Idx, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    UpperTriangularCSR2D<SparseIndex, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    fn from(error: MutabilityError<CSR2D<SparseIndex, Idx, Idx>>) -> Self {
        match error {
            MutabilityError::UnorderedRowIndex(index) => MutabilityError::UnorderedRowIndex(index),
            MutabilityError::UnorderedColumnIndex(index) => {
                MutabilityError::UnorderedColumnIndex(index)
            }
            MutabilityError::DuplicatedEntry(coordinates) => {
                MutabilityError::DuplicatedEntry(coordinates)
            }
            MutabilityError::OutOfBounds(coordinates) => MutabilityError::OutOfBounds(coordinates),
            MutabilityError::MaxedOutRowIndex => MutabilityError::MaxedOutRowIndex,
            MutabilityError::MaxedOutColumnIndex => MutabilityError::MaxedOutColumnIndex,
            MutabilityError::MaxedOutSparseIndex => MutabilityError::MaxedOutSparseIndex,
        }
    }
}

impl<SparseIndex, Idx> From<MutabilityError<CSR2D<SparseIndex, Idx, Idx>>>
    for MutabilityError<SquareCSR2D<SparseIndex, Idx>>
where
    CSR2D<SparseIndex, Idx, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    fn from(error: MutabilityError<CSR2D<SparseIndex, Idx, Idx>>) -> Self {
        match error {
            MutabilityError::UnorderedRowIndex(index) => MutabilityError::UnorderedRowIndex(index),
            MutabilityError::UnorderedColumnIndex(index) => {
                MutabilityError::UnorderedColumnIndex(index)
            }
            MutabilityError::DuplicatedEntry(coordinates) => {
                MutabilityError::DuplicatedEntry(coordinates)
            }
            MutabilityError::OutOfBounds(coordinates) => MutabilityError::OutOfBounds(coordinates),
            MutabilityError::MaxedOutRowIndex => MutabilityError::MaxedOutRowIndex,
            MutabilityError::MaxedOutColumnIndex => MutabilityError::MaxedOutColumnIndex,
            MutabilityError::MaxedOutSparseIndex => MutabilityError::MaxedOutSparseIndex,
        }
    }
}

impl<SparseIndex, Idx: PositiveInteger + IntoUsize>
    From<MutabilityError<SquareCSR2D<SparseIndex, Idx>>>
    for MutabilityError<UpperTriangularCSR2D<SparseIndex, Idx>>
where
    SquareCSR2D<SparseIndex, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    UpperTriangularCSR2D<SparseIndex, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    fn from(error: MutabilityError<SquareCSR2D<SparseIndex, Idx>>) -> Self {
        match error {
            MutabilityError::UnorderedRowIndex(index) => MutabilityError::UnorderedRowIndex(index),
            MutabilityError::UnorderedColumnIndex(index) => {
                MutabilityError::UnorderedColumnIndex(index)
            }
            MutabilityError::DuplicatedEntry(coordinates) => {
                MutabilityError::DuplicatedEntry(coordinates)
            }
            MutabilityError::OutOfBounds(coordinates) => MutabilityError::OutOfBounds(coordinates),
            MutabilityError::MaxedOutRowIndex => MutabilityError::MaxedOutRowIndex,
            MutabilityError::MaxedOutColumnIndex => MutabilityError::MaxedOutColumnIndex,
            MutabilityError::MaxedOutSparseIndex => MutabilityError::MaxedOutSparseIndex,
        }
    }
}

impl<SparseIndex, Idx: PositiveInteger + IntoUsize>
    From<MutabilityError<UpperTriangularCSR2D<SparseIndex, Idx>>>
    for MutabilityError<SymmetricCSR2D<SparseIndex, Idx>>
where
    UpperTriangularCSR2D<SparseIndex, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SymmetricCSR2D<SparseIndex, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    fn from(error: MutabilityError<UpperTriangularCSR2D<SparseIndex, Idx>>) -> Self {
        match error {
            MutabilityError::UnorderedRowIndex(index) => MutabilityError::UnorderedRowIndex(index),
            MutabilityError::UnorderedColumnIndex(index) => {
                MutabilityError::UnorderedColumnIndex(index)
            }
            MutabilityError::DuplicatedEntry(coordinates) => {
                MutabilityError::DuplicatedEntry(coordinates)
            }
            MutabilityError::OutOfBounds(coordinates) => MutabilityError::OutOfBounds(coordinates),
            MutabilityError::MaxedOutRowIndex => MutabilityError::MaxedOutRowIndex,
            MutabilityError::MaxedOutColumnIndex => MutabilityError::MaxedOutColumnIndex,
            MutabilityError::MaxedOutSparseIndex => MutabilityError::MaxedOutSparseIndex,
        }
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value>
    From<MutabilityError<CSR2D<SparseIndex, RowIndex, ColumnIndex>>>
    for MutabilityError<ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>>
where
    CSR2D<SparseIndex, RowIndex, ColumnIndex>:
        Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
    ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>:
        Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
{
    fn from(error: MutabilityError<CSR2D<SparseIndex, RowIndex, ColumnIndex>>) -> Self {
        match error {
            MutabilityError::UnorderedRowIndex(index) => MutabilityError::UnorderedRowIndex(index),
            MutabilityError::UnorderedColumnIndex(index) => {
                MutabilityError::UnorderedColumnIndex(index)
            }
            MutabilityError::DuplicatedEntry(coordinates) => {
                MutabilityError::DuplicatedEntry(coordinates)
            }
            MutabilityError::OutOfBounds(coordinates) => MutabilityError::OutOfBounds(coordinates),
            MutabilityError::MaxedOutRowIndex => MutabilityError::MaxedOutRowIndex,
            MutabilityError::MaxedOutColumnIndex => MutabilityError::MaxedOutColumnIndex,
            MutabilityError::MaxedOutSparseIndex => MutabilityError::MaxedOutSparseIndex,
        }
    }
}