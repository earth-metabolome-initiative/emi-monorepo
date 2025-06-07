//! Test submodule for the Johnson algorithm for finding all cycles in a
//! directed graph.

use algebra::prelude::*;

#[test]
fn test_johnson_cycles() {
    let matrix: SquareCSR2D<CSR2D<usize, usize, usize>> =
        SquareCSR2D::from_entries(vec![(0, 1), (1, 2), (1, 3), (2, 0), (3, 4), (4, 5), (5, 3)])
            .expect("Failed to create matrix");
    let cycles: Vec<Vec<usize>> = matrix.johnson().collect();
    assert_eq!(cycles.len(), 2);
    assert_eq!(cycles[0], vec![0, 1, 2]);
    assert_eq!(cycles[1], vec![3, 4, 5]);
}

#[test]
/// Test case identified by fuzzing the Johnson algorithm.
fn test_johnson_cycles_crash1() {
    let matrix: SquareCSR2D<CSR2D<usize, usize, usize>> =
        SquareCSR2D::from_entries(vec![(0, 0), (0, 1), (1, 0), (1, 1), (1, 2)])
            .expect("Failed to create matrix");
    let cycles: Vec<Vec<usize>> = matrix.johnson().collect();
    assert_eq!(cycles.len(), 2);
    assert_eq!(cycles[0], vec![0]);
    assert_eq!(cycles[1], vec![0, 1]);
}
