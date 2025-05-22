//! Test submodule to test the Kahn algorithm.

use algebra::prelude::*;

#[test]
fn test_kahn() {
    let mut matrix: SquareCSR2D<CSR2D<usize, usize, usize>> =
        SquareCSR2D::with_sparse_shaped_capacity(5, 2);
    matrix.extend(vec![(0, 1), (4, 0)]).expect("Failed to extend matrix");
    let ordering = matrix.kahn().unwrap();

    assert_eq!(ordering, vec![3, 4, 0, 1, 2]);
}
