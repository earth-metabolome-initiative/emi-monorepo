//! Test submodule for the `Rensik` trait.

use algebra::impls::{CSR2D, SquareCSR2D};
use graph::{
    prelude::{
        Builder, DiEdgesBuilder, DiGraph, GenericMonoplexMonopartiteGraphBuilder,
        GenericVocabularyBuilder, Resnik,
    },
    traits::{EdgesBuilder, MonopartiteGraphBuilder, MonoplexGraphBuilder, VocabularyBuilder},
};
use functional_properties::similarity::ScalarSimilarity;
use sorted_vec::prelude::SortedVec;

#[test]
fn test_resnik_on_tree() -> Result<(), Box<dyn std::error::Error>> {
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
    let graph: DiGraph<usize> =
        GenericMonoplexMonopartiteGraphBuilder::default().nodes(nodes).edges(edges).build()?;
    let resnik = graph.resnik()?;
    assert!(resnik.similarity(&0,&0) > 0.99, "Self Similarity Must be 1");
    assert!(resnik.similarity(&0,&1) < 0.99, "Score should not be 1");
    Ok(())
}


