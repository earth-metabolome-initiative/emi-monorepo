//! Implementation of the `Arbitrary` trait for the `GenericGraph` struct.

use algebra::prelude::{CSR2D, Matrix2D, SquareCSR2D};
use arbitrary::{Arbitrary, Unstructured};
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use super::GenericGraph;

impl<'a, SparseIndex, Idx> Arbitrary<'a>
    for GenericGraph<Idx, SquareCSR2D<CSR2D<SparseIndex, Idx, Idx>>>
where
    SparseIndex: TryFromUsize + IntoUsize + PositiveInteger,
    Idx: PositiveInteger + for<'b> Arbitrary<'b> + TryFrom<SparseIndex> + IntoUsize + TryFromUsize,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let edges = SquareCSR2D::arbitrary(u)?;
        let nodes: Idx = edges.number_of_rows();
        Ok(Self { nodes, edges })
    }
}
