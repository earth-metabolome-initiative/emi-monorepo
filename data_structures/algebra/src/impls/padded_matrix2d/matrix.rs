//! Submodule implementing the `Matrix` trait for the `PaddedMatrix` struct.

use super::PaddedMatrix2D;
use crate::traits::{IntoUsize, Matrix, Matrix2D, TryFromUsize};

impl<M, Map> Matrix for PaddedMatrix2D<M, Map>
where
    M: Matrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
    type Coordinates = M::Coordinates;

    #[inline]
    fn shape(&self) -> Vec<usize> {
        vec![self.number_of_rows().into_usize(), self.number_of_columns().into_usize()]
    }
}

impl<M, Map> Matrix2D for PaddedMatrix2D<M, Map>
where
    M: Matrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
    type RowIndex = M::RowIndex;
    type ColumnIndex = M::ColumnIndex;

    #[inline]
    fn number_of_columns(&self) -> Self::ColumnIndex {
        let number_of_columns: usize = self.matrix.number_of_columns().into_usize();
        let number_of_rows: usize = self.matrix.number_of_rows().into_usize();
        let max = number_of_columns.max(number_of_rows);
        let Ok(number_of_columns) = Self::ColumnIndex::try_from_usize(max) else {
            panic!("The number of columns {max} is too large to be represented as a ColumnIndex")
        };
        number_of_columns
    }

    #[inline]
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
