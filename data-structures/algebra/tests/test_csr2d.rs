//! Submodule to test the CSR2D struct

use algebra::{
    impls::CSR2D,
    prelude::{MatrixMut, SparseMatrix, SparseMatrix2D, SparseMatrixMut},
};

#[test]
fn test_sparse_coordinates() {
    let mut csr = CSR2D::with_sparse_shaped_capacity((1u8, 1u8), 1u8);
    csr.add((0u8, 0u8)).expect("Failed to add (0, 0)");

    let edges = csr.sparse_coordinates().collect::<Vec<_>>();
    assert_eq!(edges.len(), 1, "Expected 1 edge, found {edges:?}");
    assert_eq!(edges[0], (0u8, 0u8), "Expected edge (0, 0), found {:?}", edges[0]);
}

#[test]
/// Test case identified by fuzzing.
fn test_csr2d_fuzz_case2() {
    let matrix: CSR2D<usize, usize, usize> =
        CSR2D::from_entries(vec![(0, 0), (0, 1), (1, 0), (1, 1), (1, 2)]).unwrap();
    let first_row = matrix.sparse_row(0).collect::<Vec<usize>>();
    let second_row = matrix.sparse_row(1).collect::<Vec<usize>>();
    assert_eq!(first_row, vec![0, 1]);
    assert_eq!(second_row, vec![0, 1, 2]);
}
