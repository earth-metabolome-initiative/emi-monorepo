//! Submodule providing a 2D matrix which pads the missing elements over the
//! diagonal, meaning it will not change the values of existing elements but
//! will add new elements where missing. If the underlying matrix is
//! rectangular, new rows and columns will be added to make it square.

use num_traits::{ConstOne, ConstZero};
use numeric_common_traits::prelude::{Bounded, IntoUsize, TryFromUsize};

use crate::traits::{
    EmptyRows, Matrix, Matrix2D, SparseMatrix, SparseMatrix2D, SparseValuedMatrix,
    SparseValuedMatrix2D, ValuedMatrix, ValuedMatrix2D,
};
mod sparse_row_with_padded_diagonal;
use sparse_row_with_padded_diagonal::SparseRowWithPaddedDiagonal;
mod sparse_rows_with_padded_diagonal;
use sparse_rows_with_padded_diagonal::SparseRowsWithPaddedDiagonal;
mod sparse_row_values_with_padded_diagonal;
use multi_ranged::{SimpleRange, Step};
use sparse_row_values_with_padded_diagonal::SparseRowValuesWithPaddedDiagonal;

use super::{CSR2DColumns, CSR2DView, M2DValues, MutabilityError};

#[cfg(feature = "arbitrary")]
mod arbitrary_impl;

/// A 2D matrix which pads the missing elements over the diagonal.
pub struct GenericMatrix2DWithPaddedDiagonal<M, Map> {
    /// The underlying matrix.
    matrix: M,
    /// The map function defining the values of the new elements.
    map: Map,
}

impl<M, Map> GenericMatrix2DWithPaddedDiagonal<M, Map>
where
    M::RowIndex: IntoUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
    M: SparseMatrix2D,
{
    /// Creates a new `GenericMatrix2DWithPaddedDiagonal` with the given matrix
    /// and map function.
    ///
    /// # Arguments
    ///
    /// * `matrix` - The underlying matrix.
    /// * `map` - The map function defining the values of the new elements.
    ///
    /// # Errors
    ///
    /// * Returns an error if the number of rows or columns exceeds the maximum
    ///   allowed size for the given row and column index types.
    pub fn new(matrix: M, map: Map) -> Result<Self, MutabilityError<M>> {
        let number_of_columns: usize = matrix.number_of_columns().into_usize();
        let number_of_rows: usize = matrix.number_of_rows().into_usize();
        if number_of_columns > M::RowIndex::MAX.into_usize() {
            return Err(MutabilityError::<M>::MaxedOutColumnIndex);
        }
        if number_of_rows > M::ColumnIndex::MAX.into_usize() {
            return Err(MutabilityError::<M>::MaxedOutRowIndex);
        }

        Ok(Self { matrix, map })
    }

    /// Returns a reference to the underlying matrix.
    pub fn matrix(&self) -> &M {
        &self.matrix
    }

    /// Returns whether the diagonal of the provided row is imputed or not.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index of the sparse row.
    ///
    /// # Panics
    ///
    /// * If the row index is out of bounds.
    pub fn is_diagonal_imputed(&self, row: M::RowIndex) -> bool {
        if row >= self.matrix.number_of_rows() {
            return true;
        }

        let row_as_column = M::ColumnIndex::try_from_usize(row.into_usize())
            .map_err(|_| MutabilityError::<M>::MaxedOutColumnIndex)
            .unwrap();

        self.matrix.sparse_row(row).all(|column| column != row_as_column)
    }
}

impl<M, Map> Matrix for GenericMatrix2DWithPaddedDiagonal<M, Map>
where
    M: Matrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
    type Coordinates = M::Coordinates;

    fn shape(&self) -> Vec<usize> {
        vec![self.number_of_rows().into_usize(), self.number_of_columns().into_usize()]
    }
}

impl<M, Map> Matrix2D for GenericMatrix2DWithPaddedDiagonal<M, Map>
where
    M: Matrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
    type RowIndex = M::RowIndex;
    type ColumnIndex = M::ColumnIndex;

    fn number_of_columns(&self) -> Self::ColumnIndex {
        let number_of_columns: usize = self.matrix.number_of_columns().into_usize();
        let number_of_rows: usize = self.matrix.number_of_rows().into_usize();
        let max = number_of_columns.max(number_of_rows);
        let Ok(number_of_columns) = Self::ColumnIndex::try_from_usize(max) else {
            panic!("The number of columns {max} is too large to be represented as a ColumnIndex")
        };
        number_of_columns
    }

    fn number_of_rows(&self) -> Self::RowIndex {
        let number_of_columns: usize = self.matrix.number_of_columns().into_usize();
        let number_of_rows: usize = self.matrix.number_of_rows().into_usize();
        let max = number_of_columns.max(number_of_rows);
        let Ok(number_of_rows) = Self::RowIndex::try_from_usize(max) else {
            panic!("The number of rows {max} is too large to be represented as a RowIndex")
        };
        number_of_rows
    }
}

impl<M, Map> SparseMatrix for GenericMatrix2DWithPaddedDiagonal<M, Map>
where
    M: SparseMatrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
    type SparseIndex = M::SparseIndex;
    type SparseCoordinates<'a>
        = CSR2DView<'a, Self>
    where
        Self: 'a;

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        CSR2DView::from(self)
    }

    fn last_sparse_coordinates(&self) -> Option<Self::Coordinates> {
        // Since the diagonal is padded, the last coordinates are the last
        // row and column of the matrix, unless the matrix is empty.
        if self.is_empty() {
            return None;
        }
        Some((
            self.number_of_rows() - M::RowIndex::ONE,
            self.number_of_columns() - M::ColumnIndex::ONE,
        ))
    }

    fn is_empty(&self) -> bool {
        // The matrix is solely empty when it has no rows and no columns.
        self.number_of_rows() == M::RowIndex::ZERO
            && self.number_of_columns() == M::ColumnIndex::ZERO
    }
}

impl<M, Map> SparseMatrix2D for GenericMatrix2DWithPaddedDiagonal<M, Map>
where
    M: SparseMatrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
    type SparseRow<'a>
        = SparseRowWithPaddedDiagonal<'a, M>
    where
        Self: 'a;
    type SparseColumns<'a>
        = CSR2DColumns<'a, Self>
    where
        Self: 'a;

    type SparseRows<'a>
        = SparseRowsWithPaddedDiagonal<'a, Self>
    where
        Self: 'a;

    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        SparseRowWithPaddedDiagonal::new(&self.matrix, row).unwrap()
    }

    fn has_entry(&self, row: Self::RowIndex, column: Self::ColumnIndex) -> bool {
        self.sparse_row(row).any(|col| col == column)
    }

    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        SparseRowsWithPaddedDiagonal::from(self)
    }

    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        CSR2DColumns::from(self)
    }
}

impl<M, Map> EmptyRows for GenericMatrix2DWithPaddedDiagonal<M, Map>
where
    M: EmptyRows,
    M::RowIndex: IntoUsize + TryFromUsize + Step,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
    type EmptyRowIndices<'a>
        = core::iter::Empty<Self::RowIndex>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a>
        = SimpleRange<Self::RowIndex>
    where
        Self: 'a;
    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        // Since we are artificially always adding rows and columns, we
        // will never have empty rows.
        core::iter::empty()
    }

    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        // Since we are artificially always adding rows and columns, we
        // will always have non-empty rows.
        SimpleRange::try_from((Self::RowIndex::ZERO, self.number_of_rows())).unwrap()
    }

    fn number_of_empty_rows(&self) -> Self::RowIndex {
        // Since we are artificially always adding rows and columns, we
        // will never have empty rows.
        Self::RowIndex::ZERO
    }

    fn number_of_non_empty_rows(&self) -> Self::RowIndex {
        // Since we are artificially always adding rows and columns, we
        // will always have non-empty rows.
        self.number_of_rows()
    }
}

impl<M, Map> ValuedMatrix for GenericMatrix2DWithPaddedDiagonal<M, Map>
where
    M: ValuedMatrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
    type Value = M::Value;
}

impl<M, Map> ValuedMatrix2D for GenericMatrix2DWithPaddedDiagonal<M, Map>
where
    M: ValuedMatrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
}

impl<M, Map> SparseValuedMatrix for GenericMatrix2DWithPaddedDiagonal<M, Map>
where
    M: SparseValuedMatrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
    Map: Fn(M::RowIndex) -> M::Value,
    M::Value: Clone,
{
    type SparseValues<'a>
        = M2DValues<'a, Self>
    where
        Self: 'a;

    fn sparse_values(&self) -> Self::SparseValues<'_> {
        M2DValues::from(self)
    }
}

impl<M, Map> SparseValuedMatrix2D for GenericMatrix2DWithPaddedDiagonal<M, Map>
where
    M: SparseValuedMatrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
    Map: Fn(M::RowIndex) -> M::Value,
    M::Value: Clone,
{
    type SparseRowValues<'a>
        = SparseRowValuesWithPaddedDiagonal<'a, M, &'a Map>
    where
        Self: 'a;

    fn sparse_row_values(&self, row: Self::RowIndex) -> Self::SparseRowValues<'_> {
        SparseRowValuesWithPaddedDiagonal::new(&self.matrix, row, &self.map).unwrap()
    }
}
