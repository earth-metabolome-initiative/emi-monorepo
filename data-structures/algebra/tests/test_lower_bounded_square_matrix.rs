//! Test submodule to test the `LowerBoundedSquareMatrix` struct.

use algebra::prelude::*;

#[test]
/// Test case identified by fuzzing.
fn test_lbsm_fuzz_case2() {
    let matrix: LowerBoundedSquareMatrix<SquareCSR2D<CSR2D<usize, usize, usize>>> =
        LowerBoundedSquareMatrix::new(
            SquareCSR2D::from_entries(vec![(0, 0), (0, 1), (1, 0), (1, 1), (1, 2)]).unwrap(),
            1,
        )
        .expect("Failed to create matrix");
    let first_row = matrix.sparse_row(0).collect::<Vec<usize>>();
    let second_row = matrix.sparse_row(1).collect::<Vec<usize>>();
    let third_row = matrix.sparse_row(2).collect::<Vec<usize>>();
    assert_eq!(first_row, Vec::new());
    assert_eq!(second_row, vec![1, 2]);
    assert_eq!(third_row, Vec::new());
}
