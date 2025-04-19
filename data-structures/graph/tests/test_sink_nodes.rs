//! Test submodule for the `SinkNodes` trait.

use algebra::impls::SquareCSR2D;
use graph::prelude::Builder;
use graph::prelude::{
    DiEdgesBuilder, DiGraph, GenericMonoplexMonopartiteGraphBuilder, GenericVocabularyBuilder,
    MonopartiteGraph, MonoplexGraph, SinkNodes,
};
use graph::traits::EdgesBuilder;
use graph::traits::MonopartiteGraphBuilder;
use graph::traits::MonoplexGraphBuilder;
use graph::traits::VocabularyBuilder;
use sorted_vec::prelude::SortedVec;

#[test]
fn test_no_sink_nodes() -> Result<(), Box<dyn std::error::Error>> {
    let nodes: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
    let edges: Vec<(usize, usize)> =
        vec![(0, 1), (1, 2), (1, 3), (2, 1), (2, 3), (3, 4), (4, 5), (5, 0), (5, 1), (5, 3)];
    let nodes: SortedVec<usize> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(nodes.len())
        .symbols(nodes.into_iter().enumerate())
        .build()?;
    let edges: SquareCSR2D<usize, usize> = DiEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .expected_shape(nodes.len())
        .edges(edges.into_iter())
        .build()?;
    let graph: DiGraph<usize> =
        GenericMonoplexMonopartiteGraphBuilder::default().nodes(nodes).edges(edges).build()?;

    assert_eq!(graph.number_of_nodes(), 6);
    assert_eq!(graph.number_of_edges(), 10);

    assert_eq!(graph.sink_nodes(), Vec::new(), "There should be no sink nodes");

    Ok(())
}

#[test]
fn test_sink_nodes() -> Result<(), Box<dyn std::error::Error>> {
    let nodes: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
    let edges: Vec<(usize, usize)> = vec![(1, 2), (1, 3), (2, 3), (3, 4), (4, 5)];
    let nodes: SortedVec<usize> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(nodes.len())
        .symbols(nodes.into_iter().enumerate())
        .build()
        .unwrap();
    let edges: SquareCSR2D<usize, usize> = DiEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .expected_shape(nodes.len())
        .edges(edges.into_iter())
        .build()
        .unwrap();
    let graph: DiGraph<usize> = GenericMonoplexMonopartiteGraphBuilder::default()
        .nodes(nodes)
        .edges(edges)
        .build()
        .unwrap();

    assert_eq!(graph.number_of_nodes(), 6);
    assert_eq!(graph.number_of_edges(), 5);

    assert_eq!(graph.sink_nodes(), vec![5]);

    Ok(())
}
