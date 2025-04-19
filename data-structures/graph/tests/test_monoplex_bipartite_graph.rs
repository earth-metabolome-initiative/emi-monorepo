//! Submodule to test the primary properties of a bipartite graph.

use ::graph::prelude::*;
use algebra::impls::ValuedCSR2D;
use sorted_vec::prelude::SortedVec;

#[test]
/// Test checking that the bipartite graph is correctly built.
pub fn test_bipartite_graph() {
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
    assert_eq!(graph.number_of_left_nodes(), 9);
    assert_eq!(graph.number_of_right_nodes(), 6);
    assert_eq!(graph.number_of_edges(), 5);
}
