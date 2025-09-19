//! Implementation of the `Arbitrary` trait for the `CSR2D` struct.

use arbitrary::{Arbitrary, Unstructured};
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use crate::{
    prelude::{CSR2D, MutabilityError},
    traits::{MatrixMut, SparseMatrixMut},
};

impl<'a, SparseIndex, RowIndex, ColumnIndex> Arbitrary<'a>
    for CSR2D<SparseIndex, RowIndex, ColumnIndex>
where
    SparseIndex: TryFromUsize + IntoUsize + PositiveInteger,
    RowIndex: PositiveInteger + for<'b> Arbitrary<'b> + IntoUsize + TryFromUsize,
    ColumnIndex:
        PositiveInteger + for<'b> Arbitrary<'b> + TryFrom<SparseIndex> + IntoUsize + TryFromUsize,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let number_of_rows = RowIndex::arbitrary(u)?;
        let number_of_columns = ColumnIndex::arbitrary(u)?;
        let mut edges: Vec<(RowIndex, ColumnIndex)> = Vec::arbitrary(u)?;

        // Ensure uniqueness of edges
        edges.sort_unstable();
        edges.dedup();

        let number_of_edges: SparseIndex = SparseIndex::try_from_usize(edges.len())
            .map_err(|_| arbitrary::Error::IncorrectFormat)?;

        let mut csr = CSR2D::with_sparse_shaped_capacity(
            (number_of_rows, number_of_columns),
            number_of_edges,
        );

        for (row, column) in edges {
            match csr.add((row, column)) {
                Ok(()) => {}
                Err(err) => {
                    match err {
                        MutabilityError::MaxedOutSparseIndex
                        | MutabilityError::MaxedOutRowIndex
                        | MutabilityError::MaxedOutColumnIndex => {}
                        _ => return Err(arbitrary::Error::IncorrectFormat),
                    }
                }
            }
        }

        Ok(csr)
    }
}
