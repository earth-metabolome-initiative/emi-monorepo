//! Simple test for undirected graph.

use ::graph::prelude::*;
use algebra::impls::{CSR2D, SymmetricCSR2D};
use sorted_vec::prelude::SortedVec;

#[test]
/// First simple test for undirected graph.
pub fn test_undirected_graph() {
    let nodes: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
    let edges: Vec<(usize, usize)> = vec![(1, 2), (1, 3), (2, 3), (3, 4), (4, 5)];
    let nodes: SortedVec<usize> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(nodes.len())
        .symbols(nodes.into_iter().enumerate())
        .build()
        .unwrap();
    let edges: SymmetricCSR2D<CSR2D<usize, usize, usize>> = UndiEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .expected_shape(nodes.len())
        .edges(edges.into_iter())
        .build()
        .unwrap();
    let graph: UndiGraph<usize> = UndiGraph::try_from((nodes, edges)).unwrap();
    assert_eq!(graph.number_of_nodes(), 6);
    assert_eq!(graph.number_of_edges(), 10);
}
