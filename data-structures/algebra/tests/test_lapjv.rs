//! Unit tests to verify the correctness of the `LAPjv` algorithm
//! implementation.

use algebra::{
    impls::ValuedCSR2D,
    prelude::{MatrixMut, SparseLAPJV, SparseMatrixMut},
};

#[test]
/// Tests whether a matrix with zero columns raises an error.
fn test_lapjv_zero_columns() {
    let csr: ValuedCSR2D<u8, u8, u8, f64> = ValuedCSR2D::with_sparse_shaped_capacity((10, 0), 0);
    let assignment = csr.sparse_lapjv(900.0, 1000.0).unwrap();
    assert_eq!(assignment, Vec::new());
}

#[test]
/// Tests whether a matrix with zero rows raises an error.
fn test_lapjv_zero_rows() {
    let csr: ValuedCSR2D<u8, u8, u8, f64> = ValuedCSR2D::with_sparse_shaped_capacity((0, 10), 0);
    let assignment = csr.sparse_lapjv(900.0, 1000.0).unwrap();
    assert_eq!(assignment, Vec::new());
}

#[test]
/// Tests lapjv execution on a trivial example.
fn test_lapjv() {
    let csr: ValuedCSR2D<u8, u8, u8, f64> =
        ValuedCSR2D::try_from([[1.0, 0.5, 10.0], [0.5, 10.0, 20.0], [10.0, 20.0, 0.5]])
            .expect("Failed to create CSR matrix");

    let mut assignment = csr.sparse_lapjv(900.0, 1000.0).expect("LAPjv failed");
    assignment.sort_unstable_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    assert_eq!(assignment, vec![(0, 1), (1, 0), (2, 2)]);
}

#[test]
/// Tests lapjv wide-rectangular execution on a trivial example.
fn test_lapjv_wide_rectangular() {
    let csr: ValuedCSR2D<u8, u8, u8, f64> = ValuedCSR2D::try_from([
        [1.0, 0.5, 10.0, 20.0],
        [0.5, 10.0, 20.0, 20.0],
        [10.0, 20.0, 0.5, 10.0],
    ])
    .expect("Failed to create CSR matrix");

    let mut assignment = csr.sparse_lapjv(900.0, 1000.0).expect("LAPjv failed");
    assignment.sort_unstable_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    assert_eq!(assignment, vec![(0, 1), (1, 0), (2, 2)]);
}

#[test]
/// Tests lapjv tall-rectangular execution on a trivial example.
fn test_lapjv_tall_rectangular() {
    let csr: ValuedCSR2D<u8, u8, u8, f64> = ValuedCSR2D::try_from([
        [1.0, 0.5, 10.0],
        [0.5, 10.0, 20.0],
        [10.0, 20.0, 0.5],
        [10.0, 20.0, 0.1],
    ])
    .expect("Failed to create CSR matrix");

    let mut assignment = csr.sparse_lapjv(900.0, 1000.0).expect("LAPjv failed");
    assignment.sort_unstable_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    assert_eq!(assignment, vec![(0, 1), (1, 0), (3, 2)]);
}

#[test]
/// Tests a corner case that caused an infinite loop in the `LAPjv` algorithm.
/// The algorithm should not hang and should return a valid assignment.
fn test_lapjv_infinite_loop1() {
    let mut csr: ValuedCSR2D<u8, u8, u8, f64> = ValuedCSR2D::with_sparse_shaped_capacity((3, 3), 2);
    csr.add((0, 0, 1.0)).expect("Failed to add value");
    csr.add((2, 2, 800.0)).expect("Failed to add value");

    let mut assignment = csr.sparse_lapjv(900.0, 1000.0).expect("LAPjv failed");
    assignment.sort_unstable_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    assert_eq!(assignment, vec![(0, 0), (2, 2)]);
}

#[test]
/// Tests a corner case that caused an infinite loop in the `LAPjv` algorithm.
/// The algorithm should not hang and should return a valid assignment.
fn test_lapjv_infinite_loop2() {
    let mut csr: ValuedCSR2D<u8, u8, u8, f64> = ValuedCSR2D::with_sparse_shaped_capacity((3, 3), 2);
    csr.add((1, 0, 1.0)).expect("Failed to add value");
    csr.add((1, 1, 2.0)).expect("Failed to add value");

    let mut assignment = csr.sparse_lapjv(900.0, 1000.0).expect("LAPjv failed");
    assignment.sort_unstable_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    assert_eq!(assignment, vec![(1, 0)]);
}

#[test]
/// Tests a corner case that caused an infinite loop in the `LAPjv` algorithm.
/// The algorithm should not hang and should return a valid assignment.
fn test_lapjv_infinite_loop3() {
    let mut csr: ValuedCSR2D<u8, u8, u8, f64> = ValuedCSR2D::with_sparse_shaped_capacity((3, 3), 2);
    csr.add((0, 0, 1.0)).expect("Failed to add value");

    let mut assignment = csr.sparse_lapjv(900.0, 1000.0).expect("LAPjv failed");
    assignment.sort_unstable_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    assert_eq!(assignment, vec![(0, 0)]);
}

#[test]
/// Tests a corner case that caused an infinite loop in the `LAPjv` algorithm.
/// The algorithm should not hang and should return a valid assignment.
fn test_lapjv_infinite_loop4() {
    let mut csr: ValuedCSR2D<u8, u8, u8, f64> = ValuedCSR2D::with_sparse_shaped_capacity((3, 3), 2);
    csr.add((0, 0, 2e-5)).expect("Failed to add value");
    csr.add((0, 2, 3e-5)).expect("Failed to add value");
    csr.add((2, 0, 4.778_309_726_7e-5)).expect("Failed to add value");

    let mut assignment = csr.sparse_lapjv(900.0, 1000.0).expect("LAPjv failed");
    assignment.sort_unstable_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    assert_eq!(assignment, vec![(0, 2), (2, 0)]);
}

#[test]
fn test_raising_inconsistent_unassigned_rows() {
    let mut csr: ValuedCSR2D<u8, u8, u8, f64> = ValuedCSR2D::with_sparse_shaped_capacity((3, 3), 2);
    csr.add((0, 0, 2.0)).expect("Failed to add value");
    csr.add((0, 1, 1e-3)).expect("Failed to add value");
    csr.add((1, 1, 1e-2)).expect("Failed to add value");

    let mut assignment = csr.sparse_lapjv(900.0, 1000.0).expect("LAPjv failed");
    assignment.sort_unstable_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    assert_eq!(assignment, vec![(0, 0), (1, 1)]);
}

#[test]
/// Test a corner case where at the time of report, the resulting assignment
/// is inconsistent with the Hopcroft-Karp algorithm.
fn test_lapjv_inconsistent_with_hopcroft_karp2() {
    let mut csr: ValuedCSR2D<u8, u8, u8, f64> = ValuedCSR2D::with_sparse_shaped_capacity((5, 5), 1);
    csr.add((3, 3, 0.1)).expect("Failed to add value");
    csr.add((3, 4, 2.0)).expect("Failed to add value");
    csr.add((4, 3, 2.0)).expect("Failed to add value");

    let mut assignment = csr.sparse_lapjv(900.0, 1_000_000.0).expect("LAPjv failed");
    assignment.sort_unstable_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    assert_eq!(assignment, vec![(3, 4), (4, 3)]);
}
