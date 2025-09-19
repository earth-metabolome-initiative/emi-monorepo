//! Implementation of the `Arbitrary` trait for the `CSR2D` struct.

use arbitrary::{Arbitrary, Unstructured};
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use crate::{
    prelude::{MutabilityError, ValuedCSR2D},
    traits::{MatrixMut, SparseMatrixMut},
};

impl<'a, SparseIndex, RowIndex, ColumnIndex, Value> Arbitrary<'a>
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    SparseIndex: TryFromUsize + IntoUsize + PositiveInteger,
    RowIndex: PositiveInteger + for<'b> Arbitrary<'b> + IntoUsize + TryFromUsize,
    ColumnIndex:
        PositiveInteger + for<'b> Arbitrary<'b> + TryFrom<SparseIndex> + IntoUsize + TryFromUsize,
    Value: for<'b> Arbitrary<'b>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let number_of_rows = RowIndex::arbitrary(u)?;
        let number_of_columns = ColumnIndex::arbitrary(u)?;
        let mut edges: Vec<(RowIndex, ColumnIndex, Value)> = Vec::arbitrary(u)?;

        // Ensure uniqueness of edges
        edges.sort_unstable_by(|a, b| if a.0 == b.0 { a.1.cmp(&b.1) } else { a.0.cmp(&b.0) });
        edges.dedup_by(|a, b| if a.0 == b.0 { a.1 == b.1 } else { a.0 == b.0 });

        let number_of_edges: SparseIndex = SparseIndex::try_from_usize(edges.len())
            .map_err(|_| arbitrary::Error::IncorrectFormat)?;

        let mut csr = ValuedCSR2D::with_sparse_shaped_capacity(
            (number_of_rows, number_of_columns),
            number_of_edges,
        );

        for edge in edges {
            match csr.add(edge) {
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
