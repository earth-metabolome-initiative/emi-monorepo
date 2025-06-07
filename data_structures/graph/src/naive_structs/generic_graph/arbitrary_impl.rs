//! Implementation of the `Arbitrary` trait for the `GenericGraph` struct.

use algebra::prelude::{IntoUsize, Matrix2D, PositiveInteger, SquareCSR2D, TryFromUsize};
use arbitrary::{Arbitrary, Unstructured};

use super::GenericGraph;

impl<'a, SparseIndex, Idx> Arbitrary<'a> for GenericGraph<Idx, SquareCSR2D<SparseIndex, Idx>>
where
    SparseIndex: TryFromUsize + IntoUsize + PositiveInteger,
    Idx: PositiveInteger + for<'b> Arbitrary<'b> + TryFrom<SparseIndex> + IntoUsize + TryFromUsize,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let edges: SquareCSR2D<SparseIndex, Idx> = SquareCSR2D::arbitrary(u)?;
        let nodes: Idx = edges.number_of_rows();
        Ok(Self { nodes, edges })
    }
}
