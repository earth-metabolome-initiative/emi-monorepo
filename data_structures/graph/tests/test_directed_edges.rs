//! Simple test for directed edges.

use ::graph::prelude::*;
use algebra::impls::{CSR2D, SquareCSR2D, UpperTriangularCSR2D};

#[test]
/// First simple test for directed edges.
pub fn test_square_directed_edges() {
    let edges: Vec<(usize, usize)> = vec![(1, 2), (1, 3), (2, 2), (2, 3), (3, 4), (4, 5)];
    let edges: SquareCSR2D<_> = DiEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .edges(edges.into_iter())
        .build()
        .unwrap();

    assert_eq!(edges.number_of_edges(), 6);
    assert_eq!(edges.number_of_self_loops(), 1);

    assert_eq!(edges.out_degree(0), 0, "The node 0 should have no successors.");
    assert_eq!(edges.out_degree(1), 2, "The node 1 should have 2 successors.");
    assert_eq!(edges.out_degree(2), 2, "The node 2 should have 2 successors.");
    assert_eq!(edges.out_degree(3), 1, "The node 3 should have 1 successors.");
    assert_eq!(edges.out_degree(4), 1, "The node 4 should have 1 successors.");
    assert_eq!(edges.out_degree(5), 0, "The node 5 should have no successors.");

    assert_eq!(edges.out_degrees().collect::<Vec<_>>(), vec![0, 2, 2, 1, 1, 0]);

    assert_eq!(
        edges.successors(0).collect::<Vec<_>>(),
        vec![],
        "The node 0 should have no successors."
    );
    assert_eq!(
        edges.successors(1).collect::<Vec<_>>(),
        vec![2, 3],
        "The node 1 should have successors 2 and 3."
    );
    assert_eq!(
        edges.successors(2).collect::<Vec<_>>(),
        vec![2, 3],
        "The node 2 should have successors 1, 2, and 3."
    );
    assert_eq!(
        edges.successors(3).collect::<Vec<_>>(),
        vec![4],
        "The node 3 should have successors 1, 2, and 4."
    );
    assert_eq!(
        edges.successors(4).collect::<Vec<_>>(),
        vec![5],
        "The node 4 should have successors 3 and 5."
    );
    assert_eq!(
        edges.successors(5).collect::<Vec<_>>(),
        vec![],
        "The node 5 should have successor 4."
    );
    assert_eq!(
        edges.sparse_coordinates().collect::<Vec<_>>(),
        vec![(1, 2), (1, 3), (2, 2), (2, 3), (3, 4), (4, 5),],
        "The edges are not correctly stored."
    );
}

#[test]
/// First simple test for triangular edges.
pub fn test_triangular_directed_edges() {
    let edges: Vec<(usize, usize)> = vec![(1, 2), (1, 3), (2, 2), (2, 3), (3, 4), (4, 5)];
    let edges: UpperTriangularCSR2D<CSR2D<usize, usize, usize>> = GenericEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .edges(edges.into_iter())
        .build()
        .unwrap();

    assert_eq!(edges.number_of_edges(), 6);
    assert_eq!(edges.number_of_self_loops(), 1);

    assert_eq!(edges.out_degree(0), 0, "The node 0 should have no successors.");
    assert_eq!(edges.out_degree(1), 2, "The node 1 should have 2 successors.");
    assert_eq!(edges.out_degree(2), 2, "The node 2 should have 2 successors.");
    assert_eq!(edges.out_degree(3), 1, "The node 3 should have 1 successors.");
    assert_eq!(edges.out_degree(4), 1, "The node 4 should have 1 successors.");
    assert_eq!(edges.out_degree(5), 0, "The node 5 should have no successors.");

    assert_eq!(edges.out_degrees().collect::<Vec<_>>(), vec![0, 2, 2, 1, 1, 0]);

    assert_eq!(
        edges.successors(0).collect::<Vec<_>>(),
        vec![],
        "The node 0 should have no successors."
    );
    assert_eq!(
        edges.successors(1).collect::<Vec<_>>(),
        vec![2, 3],
        "The node 1 should have successors 2 and 3."
    );
    assert_eq!(
        edges.successors(2).collect::<Vec<_>>(),
        vec![2, 3],
        "The node 2 should have successors 1, 2, and 3."
    );
    assert_eq!(
        edges.successors(3).collect::<Vec<_>>(),
        vec![4],
        "The node 3 should have successors 1, 2, and 4."
    );
    assert_eq!(
        edges.successors(4).collect::<Vec<_>>(),
        vec![5],
        "The node 4 should have successors 3 and 5."
    );
    assert_eq!(
        edges.successors(5).collect::<Vec<_>>(),
        vec![],
        "The node 5 should have successor 4."
    );
    assert_eq!(
        edges.sparse_coordinates().collect::<Vec<_>>(),
        vec![(1, 2), (1, 3), (2, 2), (2, 3), (3, 4), (4, 5),],
        "The edges are not correctly stored."
    );
}
