//! Implementation of the `Arbitrary` trait for the `SquareCSR2D` struct.

use arbitrary::{Arbitrary, Unstructured};
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use crate::{
    prelude::{Matrix2D, MatrixMut, SquareCSR2D},
    traits::SparseMatrix2D,
};

impl<'a, M> Arbitrary<'a> for SquareCSR2D<M>
where
    M: Arbitrary<'a> + MatrixMut + SparseMatrix2D<ColumnIndex = <M as Matrix2D>::RowIndex>,
    M::RowIndex: TryFromUsize + IntoUsize + PositiveInteger,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut matrix: M = M::arbitrary(u)?;
        let number_of_diagonal_values = M::RowIndex::try_from_usize(
            matrix
                .sparse_coordinates()
                .filter(|(row_index, column_index)| row_index == column_index)
                .count(),
        )
        .map_err(|_| arbitrary::Error::IncorrectFormat)?;

        let side = matrix.number_of_rows().max(matrix.number_of_columns());
        matrix.increase_shape((side, side)).map_err(|_| arbitrary::Error::IncorrectFormat)?;

        Ok(Self { matrix, number_of_diagonal_values })
    }
}
