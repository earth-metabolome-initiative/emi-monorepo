//! Test submodule to test the Tarjan algorithm.

use algebra::prelude::*;

#[test]
fn test_tarjan() {
    let matrix: SquareCSR2D<CSR2D<usize, usize, usize>> =
        SquareCSR2D::from_entries(vec![(0, 1), (1, 2), (1, 3), (2, 0), (3, 4), (4, 5), (5, 3)])
            .expect("Failed to create matrix");
    let components: Vec<Vec<usize>> = matrix.tarjan().collect();
    assert_eq!(components.len(), 2);
    assert_eq!(components[1], vec![2, 1, 0]);
    assert_eq!(components[0], vec![5, 4, 3]);
}

#[test]
/// Test case identified by fuzzing the Tarjan algorithm.
fn test_tarjan_fuzz_case1() {
    let matrix: LowerBoundedSquareMatrix<SquareCSR2D<CSR2D<usize, usize, usize>>> =
        LowerBoundedSquareMatrix::new(
            SquareCSR2D::from_entries(vec![(0, 0), (0, 1), (1, 0), (1, 1), (1, 2)]).unwrap(),
            0,
        )
        .expect("Failed to create matrix");
    let components: Vec<Vec<usize>> = matrix.tarjan().collect();
    assert_eq!(components.len(), 2);
    assert_eq!(components[0], vec![2]);
    assert_eq!(components[1], vec![1, 0]);
}

#[test]
/// Test case identified by fuzzing the Tarjan algorithm.
fn test_tarjan_fuzz_case2() {
    let matrix: LowerBoundedSquareMatrix<SquareCSR2D<CSR2D<usize, usize, usize>>> =
        LowerBoundedSquareMatrix::new(
            SquareCSR2D::from_entries(vec![(0, 0), (0, 1), (1, 0), (1, 1), (1, 2)]).unwrap(),
            1,
        )
        .expect("Failed to create matrix");
    let components: Vec<Vec<usize>> = matrix.tarjan().collect();
    assert_eq!(components.len(), 3);
    assert_eq!(components[0], vec![0]);
    assert_eq!(components[1], vec![2]);
    assert_eq!(components[2], vec![1]);
}

#[test]
/// Test case identified by fuzzing the Tarjan algorithm.
fn test_tarjan_fuzz_case3() {
    let matrix: SquareCSR2D<CSR2D<usize, usize, usize>> =
        SquareCSR2D::from_entries(vec![(1, 1), (1, 2)]).unwrap();
    let components: Vec<Vec<usize>> = matrix.tarjan().collect();
    assert_eq!(components.len(), 3);
    assert_eq!(components[0], vec![0]);
    assert_eq!(components[1], vec![2]);
    assert_eq!(components[2], vec![1]);
}
