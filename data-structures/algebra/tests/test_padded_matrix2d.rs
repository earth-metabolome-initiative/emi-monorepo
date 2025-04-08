//! Submodule testing the `PaddedMatrix2D` struct.

use algebra::prelude::*;

#[test]
fn test_padded_valued_csr2d() {
    let mut csr: ValuedCSR2D<u8, u8, u8, u8> = ValuedCSR2D::with_sparse_shaped_capacity((2, 2), 1);
    csr.add((1, 1, 63)).expect("Failed to add value");
    let padded_matrix = PaddedMatrix2D::new(&csr, |_: (u8, u8)| 1).unwrap();
    assert_eq!(padded_matrix.sparse_row_values(1).collect::<Vec<_>>(), vec![1, 63]);
    assert_eq!(padded_matrix.sparse_row_values(0).collect::<Vec<_>>(), vec![1, 1]);
}
