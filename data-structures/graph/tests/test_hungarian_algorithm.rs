//! Test submodule evaluating several corner cases in the Hungarian algorithm.

use std::collections::HashMap;

use ::graph::{prelude::*, traits::assignment::hungarian_algorithm::HungarianAlgorithmError};
use algebra::prelude::ValuedCSR2D;
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
    let matching_error: HashMap<_, _> = graph.hungarian().expect("We expect no error here.");
    assert_eq!(matching_error.number_of_assigned_nodes(), 6);
}
