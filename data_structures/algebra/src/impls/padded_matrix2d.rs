//! Submodule providing a padded matrix, which fills all of the values not
//! defined in the underlying sparse matrix with the value provided by the Map
//! function.

use std::fmt::Debug;

use numeric_common_traits::prelude::{Bounded, IntoUsize};

use super::MutabilityError;
use crate::traits::{Matrix2D, SparseMatrix2D, SparseValuedMatrix2D, ValuedMatrix};

mod imputed_row_values;
mod matrix;
pub mod padded_coordinates;
mod sparse_matrix;
mod valued_matrix;

/// A padded matrix that fills all of the values not defined in the
/// underlying sparse matrix with the value provided by the Map function.
pub struct PaddedMatrix2D<M, Map> {
    /// The underlying sparse matrix.
    matrix: M,
    /// The function to map the values not defined in the underlying sparse
    /// matrix.
    map: Map,
}

impl<M, Map> Debug for PaddedMatrix2D<M, Map>
where
    M: SparseMatrix2D,
    Self: SparseValuedMatrix2D + Matrix2D<RowIndex = M::RowIndex, ColumnIndex = M::ColumnIndex>,
    <Self as ValuedMatrix>::Value: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows: Vec<Vec<String>> = self
            .row_indices()
            .map(|row_index| {
                self.sparse_row(row_index)
                    .zip(self.sparse_row_values(row_index))
                    .map(|(column_index, value)| {
                        if self.is_imputed((row_index, column_index)) {
                            format!("I({value:?})")
                        } else {
                            format!("{value:?}")
                        }
                    })
                    .collect()
            })
            .collect();

        rows.fmt(f)
    }
}

impl<M: SparseMatrix2D, Map> PaddedMatrix2D<M, Map>
where
    M: SparseMatrix2D,
    M::RowIndex: IntoUsize,
    M::ColumnIndex: IntoUsize,
{
    /// Creates a new padded matrix with the given underlying sparse matrix and
    /// map function.
    ///
    /// # Arguments
    ///
    /// * `matrix` - The underlying sparse matrix.
    /// * `map` - The function to map the values not defined in the underlying
    ///   sparse matrix.
    ///
    /// # Errors
    ///
    /// * `MutabilityError::MaxedOutColumnIndex` - The number of columns in the
    ///   matrix exceeds the maximum column index.
    /// * `MutabilityError::MaxedOutRowIndex` - The number of rows in the matrix
    ///   exceeds the maximum row index.
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

    #[inline]
    /// Returns whether the value at the provided coordinates is imputed or
    /// not.
    ///
    /// # Arguments
    ///
    /// * `coordinates` - The coordinates to check.
    pub fn is_imputed(&self, (row_index, column_index): (M::RowIndex, M::ColumnIndex)) -> bool {
        if row_index >= self.matrix.number_of_rows()
            || column_index >= self.matrix.number_of_columns()
        {
            return true;
        }

        self.matrix.sparse_row(row_index).all(|column| column != column_index)
    }
}
