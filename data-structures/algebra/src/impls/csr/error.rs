//! Enumeration for the errors associated with the CSR data structure.

use crate::traits::{IntoUsize, Matrix2D, PositiveInteger};
use core::fmt::Debug;

use super::{SquareCSR2D, UpperTriangularCSR2D, CSR2D};

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
pub enum MutabilityError<M: Matrix2D> {
    /// Unexpected row index.
    UnorderedRowIndex(M::RowIndex),
    /// Unexpected column index.
    UnorderedColumnIndex(M::ColumnIndex),
    /// Duplicated entry.
    DuplicatedEntry(M::Coordinates),
    /// Entry out of bounds.
    OutOfBounds(M::Coordinates),
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
        }
    }
}

impl<Offset, Idx> From<MutabilityError<CSR2D<Offset, Idx, Idx>>>
    for MutabilityError<UpperTriangularCSR2D<Offset, Idx>>
where
    CSR2D<Offset, Idx, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    UpperTriangularCSR2D<Offset, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    fn from(error: MutabilityError<CSR2D<Offset, Idx, Idx>>) -> Self {
        match error {
            MutabilityError::UnorderedRowIndex(index) => MutabilityError::UnorderedRowIndex(index),
            MutabilityError::UnorderedColumnIndex(index) => {
                MutabilityError::UnorderedColumnIndex(index)
            }
            MutabilityError::DuplicatedEntry(coordinates) => {
                MutabilityError::DuplicatedEntry(coordinates)
            }
            MutabilityError::OutOfBounds(coordinates) => MutabilityError::OutOfBounds(coordinates),
        }
    }
}

impl<Offset, Idx> From<MutabilityError<CSR2D<Offset, Idx, Idx>>>
    for MutabilityError<SquareCSR2D<Offset, Idx>>
where
    CSR2D<Offset, Idx, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<Offset, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    fn from(error: MutabilityError<CSR2D<Offset, Idx, Idx>>) -> Self {
        match error {
            MutabilityError::UnorderedRowIndex(index) => MutabilityError::UnorderedRowIndex(index),
            MutabilityError::UnorderedColumnIndex(index) => {
                MutabilityError::UnorderedColumnIndex(index)
            }
            MutabilityError::DuplicatedEntry(coordinates) => {
                MutabilityError::DuplicatedEntry(coordinates)
            }
            MutabilityError::OutOfBounds(coordinates) => MutabilityError::OutOfBounds(coordinates),
        }
    }
}

impl<Offset, Idx: PositiveInteger + IntoUsize> From<MutabilityError<SquareCSR2D<Offset, Idx>>>
    for MutabilityError<UpperTriangularCSR2D<Offset, Idx>>
where
    SquareCSR2D<Offset, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    UpperTriangularCSR2D<Offset, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    fn from(error: MutabilityError<SquareCSR2D<Offset, Idx>>) -> Self {
        match error {
            MutabilityError::UnorderedRowIndex(index) => MutabilityError::UnorderedRowIndex(index),
            MutabilityError::UnorderedColumnIndex(index) => {
                MutabilityError::UnorderedColumnIndex(index)
            }
            MutabilityError::DuplicatedEntry(coordinates) => {
                MutabilityError::DuplicatedEntry(coordinates)
            }
            MutabilityError::OutOfBounds(coordinates) => MutabilityError::OutOfBounds(coordinates),
        }
    }
}
