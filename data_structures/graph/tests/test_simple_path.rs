//! Test submodule for the `SimplePath` trait.

use algebra::impls::{CSR2D, SquareCSR2D};
use graph::{
    prelude::{DiEdgesBuilder, DiGraph, GenericVocabularyBuilder, SimplePath},
    traits::{EdgesBuilder, VocabularyBuilder},
};
use sorted_vec::prelude::SortedVec;

#[test]
fn test_simple_path_on_empty_graph() -> Result<(), Box<dyn std::error::Error>> {
    let nodes: Vec<usize> = vec![];
    let edges: Vec<(usize, usize)> = vec![];
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

    assert!(!graph.is_simple_path(), "An empty graph is not a simple path");

    Ok(())
}

#[test]
fn test_simple_path_on_singleton_node() -> Result<(), Box<dyn std::error::Error>> {
    let nodes: Vec<usize> = vec![0];
    let edges: Vec<(usize, usize)> = vec![];
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

    assert!(graph.is_simple_path(), "A singleton node is a simple path");

    Ok(())
}

#[test]
fn test_simple_path_on_selflooping_singleton_node() -> Result<(), Box<dyn std::error::Error>> {
    let nodes: Vec<usize> = vec![0];
    let edges: Vec<(usize, usize)> = vec![(0, 0)];
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

    assert!(!graph.is_simple_path(), "A self-looping singleton node is not a simple path");

    Ok(())
}

#[test]
fn test_simple_path_on_edge() -> Result<(), Box<dyn std::error::Error>> {
    let nodes: Vec<usize> = vec![0, 1];
    let edges: Vec<(usize, usize)> = vec![(0, 1)];
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

    assert!(graph.is_simple_path(), "A simple path with an edge is a simple path");

    Ok(())
}

#[test]
fn test_simple_path_on_circle() -> Result<(), Box<dyn std::error::Error>> {
    let nodes: Vec<usize> = vec![0, 1, 2];
    let edges: Vec<(usize, usize)> = vec![(0, 1), (1, 2), (2, 0)];
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

    assert!(!graph.is_simple_path(), "A simple path with a cycle is not a simple path");

    Ok(())
}

#[test]
fn test_simple_path_on_tree() -> Result<(), Box<dyn std::error::Error>> {
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

    assert!(!graph.is_simple_path(), "A simple path with a tree is not a simple path");

    Ok(())
}
