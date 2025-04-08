//! Submodule to test the CSR2D struct

use algebra::{
    impls::CSR2D,
    prelude::{MatrixMut, SparseMatrix, SparseMatrixMut},
};

#[test]
fn test_sparse_coordinates() {
    let mut csr = CSR2D::with_sparse_shaped_capacity((1u8, 1u8), 1u8);
    csr.add((0u8, 0u8)).expect("Failed to add (0, 0)");

    let edges = csr.sparse_coordinates().collect::<Vec<_>>();
    assert_eq!(edges.len(), 1, "Expected 1 edge, found {edges:?}");
    assert_eq!(edges[0], (0u8, 0u8), "Expected edge (0, 0), found {:?}", edges[0]);
}
