//! Trait defining a bidimensional matrix.

use multi_ranged::{SimpleRange, Step};
use num_traits::ConstZero;
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger};

use super::SquareMatrix;
use crate::traits::Matrix;

/// Trait defining a bidimensional matrix.
pub trait Matrix2D:
    Matrix<Coordinates = (<Self as Matrix2D>::RowIndex, <Self as Matrix2D>::ColumnIndex)>
{
    /// Type of the row index.
    type RowIndex: Step + PositiveInteger + IntoUsize;
    /// Type of the column index.
    type ColumnIndex: Step + PositiveInteger + IntoUsize;

    /// Returns the number of rows of the matrix.
    fn number_of_rows(&self) -> Self::RowIndex;

    /// Returns the number of columns of the matrix.
    fn number_of_columns(&self) -> Self::ColumnIndex;

    #[inline]
    /// Returns an iterator over the rows of the matrix.
    fn row_indices(&self) -> SimpleRange<Self::RowIndex> {
        SimpleRange::try_from((Self::RowIndex::ZERO, self.number_of_rows())).unwrap()
    }

    #[inline]
    /// Returns an iterator over the columns of the matrix.
    fn column_indices(&self) -> SimpleRange<Self::ColumnIndex> {
        SimpleRange::try_from((Self::ColumnIndex::ZERO, self.number_of_columns())).unwrap()
    }
}

impl<M: Matrix2D> Matrix2D for &M {
    type RowIndex = M::RowIndex;
    type ColumnIndex = M::ColumnIndex;

    #[inline]
    fn number_of_rows(&self) -> Self::RowIndex {
        (*self).number_of_rows()
    }

    #[inline]
    fn number_of_columns(&self) -> Self::ColumnIndex {
        (*self).number_of_columns()
    }
}

/// Trait defining a bidimensional matrix which can return references to
/// the number of rows and columns.
pub trait Matrix2DRef: Matrix2D {
    /// Returns a reference to the number of rows of the matrix.
    fn number_of_rows_ref(&self) -> &Self::RowIndex;

    /// Returns a reference to the number of columns of the matrix.
    fn number_of_columns_ref(&self) -> &Self::ColumnIndex;
}

/// Trait defining a transposable bidimensional matrix.
pub trait TransposableMatrix2D<
    Transposed: Matrix2D<RowIndex = Self::ColumnIndex, ColumnIndex = Self::RowIndex>,
>: Matrix2D
{
    /// Returns the transpose of the matrix.
    fn transpose(&self) -> Transposed;
}

/// Trait defining a matrix which supports efficient operations on columns.
pub trait BiMatrix2D: Matrix2D {
    /// The type of the underlying matrix.
    type Matrix: Matrix2D<RowIndex = Self::RowIndex, ColumnIndex = Self::ColumnIndex>;
    /// The type of the underlying transposed matrix.
    type TransposedMatrix: Matrix2D<RowIndex = Self::ColumnIndex, ColumnIndex = Self::RowIndex>;

    /// Returns a reference to the underlying matrix.
    fn matrix(&self) -> &Self::Matrix;

    /// Returns a reference to the underlying transposed matrix.
    fn transposed(&self) -> &Self::TransposedMatrix;
}

/// Trait defining a symmetric matrix.
pub trait SymmetricMatrix2D:
    SquareMatrix
    + BiMatrix2D<
        Matrix = <Self as SymmetricMatrix2D>::SymmetricMatrix,
        TransposedMatrix = <Self as SymmetricMatrix2D>::SymmetricMatrix,
    >
{
    /// The underlying symmetric matrix type.
    type SymmetricMatrix: Matrix2D<RowIndex = Self::Index, ColumnIndex = Self::Index>;
}

impl<M> SymmetricMatrix2D for M
where
    M: SquareMatrix + BiMatrix2D<TransposedMatrix = <M as BiMatrix2D>::Matrix>,
    M::Matrix: Matrix2D<RowIndex = Self::RowIndex, ColumnIndex = Self::ColumnIndex>,
{
    type SymmetricMatrix = M::Matrix;
}
