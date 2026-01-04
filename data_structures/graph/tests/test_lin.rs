//! Test submodule for the `Lin` trait.

use algebra::impls::{CSR2D, SquareCSR2D};
use functional_properties::similarity::ScalarSimilarity;
use graph::{
    prelude::{DiEdgesBuilder, DiGraph, GenericVocabularyBuilder, Lin},
    traits::{EdgesBuilder, MonopartiteGraph, VocabularyBuilder},
};
use sorted_vec::prelude::SortedVec;

#[test]
fn test_lin_on_tree() -> Result<(), Box<dyn std::error::Error>> {
    let nodes: Vec<usize> = vec![0, 1, 2];
    let edges: Vec<(usize, usize)> = vec![(0, 1), (0, 2), (1, 2)];
    let nodes: SortedVec<usize> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(nodes.len())
        .symbols(nodes.into_iter().enumerate())
        .build()?;
    let edges: SquareCSR2D<CSR2D<usize, usize, usize>> = DiEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .expected_shape(nodes.len())
        .edges(edges.into_iter())
        .build()?;
    let graph: DiGraph<usize> = DiGraph::try_from((nodes, edges))?;
    let lin = graph.lin(&[1, 1, 1])?;
    for nodeid in graph.node_ids() {
        let self_similarity = lin.similarity(&nodeid, &nodeid);
        assert!(self_similarity > 0.99, "Self Similarity Must be 1 but was {self_similarity}")
    }
    assert!(lin.similarity(&0, &1) < 0.99, "Score should not be 1");
    Ok(())
}
