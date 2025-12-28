//! Simple test for symmetric edges.

use ::graph::prelude::*;
use algebra::impls::{CSR2D, SymmetricCSR2D};

#[test]
/// First simple test for symmetric edges.
pub fn test_symmetric_edges() {
    let edges: Vec<(usize, usize)> = vec![(1, 2), (1, 3), (2, 2), (2, 3), (3, 4), (4, 5)];
    let edges: SymmetricCSR2D<CSR2D<usize, usize, usize>> = UndiEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .edges(edges.into_iter())
        .build()
        .unwrap();

    assert_eq!(edges.number_of_edges(), 11);
    assert_eq!(edges.number_of_self_loops(), 1);

    assert_eq!(edges.degrees().collect::<Vec<_>>(), vec![0, 2, 3, 3, 2, 1]);

    assert_eq!(edges.degree(0), 0, "The node 0 should have no neighbors.");
    assert_eq!(edges.degree(1), 2, "The node 1 should have 2 neighbors.");
    assert_eq!(edges.degree(2), 3, "The node 2 should have 3 neighbors.");
    assert_eq!(edges.degree(3), 3, "The node 3 should have 3 neighbors.");
    assert_eq!(edges.degree(4), 2, "The node 4 should have 2 neighbors.");
    assert_eq!(edges.degree(5), 1, "The node 5 should have 1 neighbor.");

    assert_eq!(
        edges.neighbors(0).collect::<Vec<_>>(),
        vec![],
        "The node 0 should have no neighbors."
    );
    assert_eq!(
        edges.neighbors(1).collect::<Vec<_>>(),
        vec![2, 3],
        "The node 1 should have neighbors 2 and 3."
    );
    assert_eq!(
        edges.neighbors(2).collect::<Vec<_>>(),
        vec![1, 2, 3],
        "The node 2 should have neighbors 1, 2, and 3."
    );
    assert_eq!(
        edges.neighbors(3).collect::<Vec<_>>(),
        vec![1, 2, 4],
        "The node 3 should have neighbors 1, 2, and 4."
    );
    assert_eq!(
        edges.neighbors(4).collect::<Vec<_>>(),
        vec![3, 5],
        "The node 4 should have neighbors 3 and 5."
    );
    assert_eq!(
        edges.neighbors(5).collect::<Vec<_>>(),
        vec![4],
        "The node 5 should have neighbor 4."
    );
    assert_eq!(
        edges.sparse_coordinates().collect::<Vec<_>>(),
        vec![
            (1, 2),
            (1, 3),
            (2, 1),
            (2, 2),
            (2, 3),
            (3, 1),
            (3, 2),
            (3, 4),
            (4, 3),
            (4, 5),
            (5, 4)
        ],
        "The edges are not correctly stored."
    );
}
