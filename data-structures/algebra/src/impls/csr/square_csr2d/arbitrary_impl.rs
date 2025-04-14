//! Implementation of the `Arbitrary` trait for the `SquareCSR2D` struct.

use arbitrary::{Arbitrary, Unstructured};

use crate::{
    prelude::{CSR2D, SquareCSR2D},
    traits::{IntoUsize, PositiveInteger, SparseMatrix, TryFromUsize},
};

impl<'a, SparseIndex, Idx> Arbitrary<'a> for SquareCSR2D<SparseIndex, Idx>
where
    SparseIndex: TryFromUsize + IntoUsize + PositiveInteger,
    Idx: PositiveInteger + for<'b> Arbitrary<'b> + TryFrom<SparseIndex> + IntoUsize + TryFromUsize,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut csr: CSR2D<SparseIndex, Idx, Idx> = CSR2D::arbitrary(u)?;
        let number_of_diagonal_values = Idx::try_from_usize(
            csr.sparse_coordinates()
                .filter(|(row_index, column_index)| row_index == column_index)
                .count(),
        )
        .map_err(|_| arbitrary::Error::IncorrectFormat)?;

		csr.number_of_columns = csr.number_of_columns.max(csr.number_of_rows);
        csr.number_of_rows = csr.number_of_rows.max(csr.number_of_columns);

        Ok(Self { csr, number_of_diagonal_values })
    }
}
