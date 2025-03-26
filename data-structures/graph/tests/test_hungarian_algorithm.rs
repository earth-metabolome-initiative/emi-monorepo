//! Test submodule evaluating several corner cases in the Hungarian algorithm.

use std::collections::HashMap;

use ::graph::{prelude::*, traits::weighted_assignment::hungarian_algorithm::HungarianAlgorithmError};
use algebra::{
    impls::{ranged::SimpleRanged, GenericImplicitValuedMatrix2D, RangedCSR2D, CSR2D},
    prelude::ValuedCSR2D,
};
use common_traits::builder::Builder;
use sorted_vec::prelude::SortedVec;

#[test]
/// Test checking that the hungarian completes nominally on an empty graph, i.e.
/// a graph with no vertices (or, of course, edges).
pub fn test_hungarian_on_empty_graph() {
    let graph: GenericBiGraph<
        SortedVec<usize>,
        SortedVec<usize>,
        ValuedCSR2D<usize, usize, usize, f64>,
    > = WeightedBiGraph::default();
    let matching: HashMap<usize, (usize, f64)> = graph.hungarian().unwrap();
    assert_eq!(matching.number_of_assigned_nodes(), 0);
}

#[test]
/// Test checking that the hungarian raises an appropriate error when the graph
/// does not have any edges but has vertices.
pub fn test_hungarian_on_graph_with_no_edges() {
    let left_nodes: Vec<u16> = vec![0, 1, 2, 3, 4, 5];
    let right_nodes: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
    let edges: Vec<(usize, usize, f64)> = Vec::new();
    let left_nodes: SortedVec<u16> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(left_nodes.len())
        .symbols(left_nodes.into_iter().enumerate())
        .build()
        .unwrap();
    let right_nodes: SortedVec<u8> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(right_nodes.len())
        .symbols(right_nodes.into_iter().enumerate())
        .build()
        .unwrap();
    let edges: ValuedCSR2D<usize, usize, usize, f64> = GenericEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .expected_shape((left_nodes.len(), right_nodes.len()))
        .edges(edges.into_iter())
        .build()
        .unwrap();
    let graph: GenericBiGraph<
        SortedVec<u16>,
        SortedVec<u8>,
        ValuedCSR2D<usize, usize, usize, f64>,
    > = GenericMonoplexBipartiteGraphBuilder::default()
        .left_nodes(left_nodes)
        .right_nodes(right_nodes)
        .edges(edges)
        .build()
        .unwrap();
    let matching: Result<HashMap<usize, (usize, f64)>, HungarianAlgorithmError> = graph.hungarian();
    let matching_error = matching.unwrap_err();
    assert_eq!(matching_error, HungarianAlgorithmError::NoEdges);
}

#[test]
/// Test checking that the hungarian raises an appropriate error when the graph
/// does not have any edges but has vertices.
pub fn test_hungarian_on_empty_edge_list() {
    let edges: Vec<(usize, usize, f64)> = Vec::new();
    let graph: ValuedCSR2D<usize, usize, usize, f64> = GenericEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .expected_shape((6, 6))
        .edges(edges.into_iter())
        .build()
        .unwrap();
    let matching: Result<HashMap<usize, (usize, f64)>, HungarianAlgorithmError> = graph.hungarian();
    let matching_error = matching.unwrap_err();
    assert_eq!(matching_error, HungarianAlgorithmError::NoEdges);
}

#[test]
/// Test checking that the hungarian completes nominally on a graph
/// that starts off with a perfect matching.
pub fn test_hungarian_on_graph_with_matching() {
    let left_nodes: Vec<u16> = vec![0, 1, 2, 3, 4, 5];
    let right_nodes: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
    let edges: Vec<(usize, usize, f64)> =
        vec![(0, 0, 1.0), (1, 1, 1.0), (2, 2, 1.0), (3, 3, 1.0), (4, 4, 1.0), (5, 5, 1.0)];
    let left_nodes: SortedVec<u16> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(left_nodes.len())
        .symbols(left_nodes.into_iter().enumerate())
        .build()
        .unwrap();
    let right_nodes: SortedVec<u8> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(right_nodes.len())
        .symbols(right_nodes.into_iter().enumerate())
        .build()
        .unwrap();
    let edges: ValuedCSR2D<usize, usize, usize, f64> = GenericEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .expected_shape((left_nodes.len(), right_nodes.len()))
        .edges(edges.into_iter())
        .build()
        .unwrap();
    let graph: GenericBiGraph<
        SortedVec<u16>,
        SortedVec<u8>,
        ValuedCSR2D<usize, usize, usize, f64>,
    > = GenericMonoplexBipartiteGraphBuilder::default()
        .left_nodes(left_nodes)
        .right_nodes(right_nodes)
        .edges(edges)
        .build()
        .unwrap();
    let matching: HashMap<_, _> = graph.hungarian().expect("We expect no error here.");
    assert_eq!(matching.number_of_assigned_nodes(), 6);

    for (left_node, (right_node, _weight)) in matching.iter() {
        assert_eq!(left_node, right_node);
    }

    assert_eq!(matching.cost(), 6.0);
    let inner_matching: HashMap<_, _> =
        graph.edges().hungarian().expect("We expect no error here.");
    assert_eq!(inner_matching, matching);
}

#[test]
/// Test checking that the hungarian completes nominally on a graph
/// that does not have the same number of left and right nodes.
pub fn test_hungarian_on_left_unbalanced_bigraph() {
    let left_nodes: Vec<u16> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let right_nodes: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
    let edges: Vec<(usize, usize, f64)> =
        vec![(1, 1, 1.0), (2, 2, 1.0), (3, 3, 1.0), (4, 4, 1.0), (5, 5, 1.0)];
    let left_nodes: SortedVec<u16> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(left_nodes.len())
        .symbols(left_nodes.into_iter().enumerate())
        .build()
        .unwrap();
    let right_nodes: SortedVec<u8> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(right_nodes.len())
        .symbols(right_nodes.into_iter().enumerate())
        .build()
        .unwrap();
    let edges: ValuedCSR2D<usize, usize, usize, f64> = GenericEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .expected_shape((left_nodes.len(), right_nodes.len()))
        .edges(edges.into_iter())
        .build()
        .unwrap();
    let graph: GenericBiGraph<
        SortedVec<u16>,
        SortedVec<u8>,
        ValuedCSR2D<usize, usize, usize, f64>,
    > = GenericMonoplexBipartiteGraphBuilder::default()
        .left_nodes(left_nodes)
        .right_nodes(right_nodes)
        .edges(edges)
        .build()
        .unwrap();
    let matching: HashMap<_, _> = graph.hungarian().expect("We expect no error here.");
    assert_eq!(
        matching.number_of_assigned_nodes(),
        5,
        "The number of assigned nodes is not as expected: {:?}",
        matching
    );

    for (left_node, (right_node, _weight)) in matching.iter() {
        assert_eq!(left_node, right_node);
    }

    assert_eq!(matching.cost(), 5.0);

    let inner_matching: HashMap<_, _> =
        graph.edges().hungarian().expect("We expect no error here.");
    assert_eq!(inner_matching, matching);
}

#[test]
/// Test checking that the hungarian completes nominally on a graph
/// that does not have the same number of left and right nodes.
pub fn test_hungarian_on_right_unbalanced_bigraph() {
    let left_nodes: Vec<u8> = vec![0, 1, 2, 3, 4];
    let right_nodes: Vec<u16> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let edges: Vec<(usize, usize, f64)> = vec![(1, 1, 1.0), (2, 2, 1.0), (3, 3, 1.0), (4, 4, 1.0)];
    let left_nodes: SortedVec<u8> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(left_nodes.len())
        .symbols(left_nodes.into_iter().enumerate())
        .build()
        .unwrap();
    let right_nodes: SortedVec<u16> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(right_nodes.len())
        .symbols(right_nodes.into_iter().enumerate())
        .build()
        .unwrap();
    let edges: ValuedCSR2D<usize, usize, usize, f64> = GenericEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .expected_shape((left_nodes.len(), right_nodes.len()))
        .edges(edges.into_iter())
        .build()
        .unwrap();
    let graph: GenericBiGraph<
        SortedVec<u8>,
        SortedVec<u16>,
        ValuedCSR2D<usize, usize, usize, f64>,
    > = GenericMonoplexBipartiteGraphBuilder::default()
        .left_nodes(left_nodes)
        .right_nodes(right_nodes)
        .edges(edges)
        .build()
        .unwrap();
    let matching: HashMap<_, _> = graph.hungarian().expect("We expect no error here.");
    assert_eq!(
        matching.number_of_assigned_nodes(),
        4,
        "The number of assigned nodes is not as expected: {:?}",
        matching
    );

    for (left_node, (right_node, _weight)) in matching.iter() {
        assert_eq!(left_node, right_node);
    }

    assert_eq!(matching.cost(), 4.0);

    let inner_matching: HashMap<_, _> =
        graph.edges().hungarian().expect("We expect no error here.");
    assert_eq!(inner_matching, matching);
}

#[test]
/// Test checking that the hungarian completes with the minimum cost matching.
pub fn test_hungarian() {
    let left_nodes: Vec<u8> = vec![0, 1, 2, 3, 4];
    let right_nodes: Vec<u16> = vec![0, 1, 2, 3, 4];
    let edges: Vec<(usize, usize, f64)> = vec![
        (0, 0, 1.0),
        (0, 1, 2.0),
        (1, 0, 1.0),
        (1, 1, 1.0),
        (2, 0, 20.0),
        (2, 1, 10.0),
        (2, 2, 1.0),
        (3, 3, 1.0),
        (3, 4, 10.0),
        (4, 2, 10.0),
        (4, 4, 1.0),
    ];
    let left_nodes: SortedVec<u8> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(left_nodes.len())
        .symbols(left_nodes.into_iter().enumerate())
        .build()
        .unwrap();
    let right_nodes: SortedVec<u16> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(right_nodes.len())
        .symbols(right_nodes.into_iter().enumerate())
        .build()
        .unwrap();
    let edges: ValuedCSR2D<usize, usize, usize, f64> = GenericEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .expected_shape((left_nodes.len(), right_nodes.len()))
        .edges(edges.into_iter())
        .build()
        .unwrap();
    let graph: GenericBiGraph<
        SortedVec<u8>,
        SortedVec<u16>,
        ValuedCSR2D<usize, usize, usize, f64>,
    > = GenericMonoplexBipartiteGraphBuilder::default()
        .left_nodes(left_nodes)
        .right_nodes(right_nodes)
        .edges(edges)
        .build()
        .unwrap();
    let matching: HashMap<_, _> = graph.hungarian().expect("We expect no error here.");
    assert_eq!(
        matching.number_of_assigned_nodes(),
        5,
        "The number of assigned nodes is not as expected: {:?}",
        matching
    );

    for (left_node, (right_node, _weight)) in matching.iter() {
        assert_eq!(left_node, right_node);
    }

    assert_eq!(matching.cost(), 5.0, "The cost is not as expected: {:?}", matching);

    let inner_matching: HashMap<_, _> =
        graph.edges().hungarian().expect("We expect no error here.");
    assert_eq!(inner_matching, matching);
}

#[test]
/// Test checking that the hungarian completes with the minimum cost matching.
pub fn test_hungarian_implicit_weighted_matrix() {
    let left_nodes: Vec<u8> = vec![0, 1, 2, 3, 4];
    let right_nodes: Vec<u16> = vec![0, 1, 2, 3, 4];
    let edges: Vec<(usize, usize)> = vec![
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (2, 0),
        (2, 1),
        (2, 2),
        (3, 3),
        (3, 4),
        (4, 2),
        (4, 4),
    ];
    let left_nodes: SortedVec<u8> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(left_nodes.len())
        .symbols(left_nodes.into_iter().enumerate())
        .build()
        .unwrap();
    let right_nodes: SortedVec<u16> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(right_nodes.len())
        .symbols(right_nodes.into_iter().enumerate())
        .build()
        .unwrap();
    let unweighted_edges: CSR2D<usize, usize, usize> = GenericEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .expected_shape((left_nodes.len(), right_nodes.len()))
        .edges(edges.into_iter())
        .build()
        .unwrap();
    let weighted_edges: GenericImplicitValuedMatrix2D<_, _, f64> =
        GenericImplicitValuedMatrix2D::new(
            unweighted_edges,
            |(i, j)| {
                if i == j {
                    1.0
                } else {
                    10.0
                }
            },
        );
    let graph: GenericBiGraph<
        SortedVec<u8>,
        SortedVec<u16>,
        GenericImplicitValuedMatrix2D<_, _, f64>
    > = GenericMonoplexBipartiteGraphBuilder::default()
        .left_nodes(left_nodes)
        .right_nodes(right_nodes)
        .edges(weighted_edges)
        .build()
        .unwrap();
    let matching: HashMap<_, _> = graph.hungarian().expect("We expect no error here.");
    assert_eq!(
        matching.number_of_assigned_nodes(),
        5,
        "The number of assigned nodes is not as expected: {:?}",
        matching
    );

    for (left_node, (right_node, _weight)) in matching.iter() {
        assert_eq!(left_node, right_node);
    }

    assert_eq!(matching.cost(), 5.0, "The cost is not as expected: {:?}", matching);

    let inner_matching: HashMap<_, _> =
        graph.edges().hungarian().expect("We expect no error here.");
    assert_eq!(inner_matching, matching);
}


#[test]
/// Test checking that the hungarian completes on an implicit weighted matrix
/// defined on a range of values.
pub fn test_hungarian_implicit_weighted_ranged_matrix() {
    let edges: Vec<(u16, u16)> = vec![
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (2, 0),
        (2, 1),
        (2, 2),
        (3, 3),
        (3, 4),
        (4, 2),
        (4, 3),
        (4, 4),
    ];
    let unweighted_edges: RangedCSR2D<u16, u16, SimpleRanged<u16>> = GenericEdgesBuilder::default()
        .expected_number_of_edges(edges.len() as u16)
        .expected_shape((5, 5))
        .edges(edges.into_iter())
        .build()
        .unwrap();
    let weighted_edges: GenericImplicitValuedMatrix2D<RangedCSR2D<u16, u16, SimpleRanged<u16>>, _, f64> =
        GenericImplicitValuedMatrix2D::new(
            unweighted_edges,
            |(i, j)| {
                if i as usize == j as usize {
                    1.0
                } else {
                    10.0
                }
            },
        );
    
    let matching: HashMap<_, _> = weighted_edges.hungarian().expect("We expect no error here.");
    assert_eq!(
        matching.number_of_assigned_nodes(),
        5,
        "The number of assigned nodes is not as expected: {:?}",
        matching
    );

    for (left_node, (right_node, _weight)) in matching.iter() {
        assert_eq!(*left_node as u16, *right_node);
    }

    assert_eq!(matching.cost(), 5.0, "The cost is not as expected: {:?}", matching);
}
