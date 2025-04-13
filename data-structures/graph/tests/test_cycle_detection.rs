//! Test submodule for the `CycleDetection` trait.

use algebra::impls::SquareCSR2D;
use graph::prelude::Builder;
use graph::prelude::{
    CycleDetection, DiEdgesBuilder, DiGraph, GenericMonoplexMonopartiteGraphBuilder,
    GenericVocabularyBuilder, MonopartiteGraph, MonoplexGraph,
};
use graph::traits::EdgesBuilder;
use graph::traits::MonopartiteGraphBuilder;
use graph::traits::MonoplexGraphBuilder;
use graph::traits::VocabularyBuilder;
use sorted_vec::prelude::SortedVec;

#[test]
fn test_no_cycle_detection() -> Result<(), Box<dyn std::error::Error>> {
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

    assert!(!graph.has_cycle());

    Ok(())
}

#[test]
fn test_cycle_detection() -> Result<(), Box<dyn std::error::Error>> {
    let nodes: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
    let edges: Vec<(usize, usize)> = vec![(1, 2), (1, 3), (2, 3), (3, 2), (3, 4), (4, 5)];
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
    assert_eq!(graph.number_of_edges(), 6);

    assert!(graph.has_cycle());

    Ok(())
}
