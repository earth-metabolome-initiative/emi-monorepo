//! Test submodule for the `Rensik` trait.

use algebra::impls::{CSR2D, SquareCSR2D};
use functional_properties::similarity::ScalarSimilarity;
use graph::{
    prelude::{
        Builder, DiEdgesBuilder, DiGraph, GenericMonoplexMonopartiteGraphBuilder,
        GenericVocabularyBuilder, Resnik,
    },
    traits::{
        EdgesBuilder, MonopartiteGraphBuilder, MonoplexGraphBuilder, VocabularyBuilder,
        resnik::ResnikError,
    },
};
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
    let resnik = graph.resnik(&[1.0, 1.0, 1.0])?;
    assert!(resnik.similarity(&0, &0) > 0.99, "Self Similarity Must be 1");
    assert!(resnik.similarity(&0, &1) < 0.99, "Score should not be 1");
    Ok(())
}

#[test]
fn test_resnik_not_dag() -> Result<(), Box<dyn std::error::Error>> {
    let nodes: Vec<usize> = vec![0, 1, 2];
    let edges: Vec<(usize, usize)> = vec![(0, 1), (0, 2), (1, 2), (2, 0)];
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
    let resnik = graph.resnik(&[1.0, 1.0, 1.0]);
    assert_eq!(resnik, Err(ResnikError::NotDag));
    Ok(())
}

#[test]
fn test_resnik_incorrect_occurrences() -> Result<(), Box<dyn std::error::Error>> {
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
    let resnik = graph.resnik(&Vec::new());
    // length mismatch
    assert_eq!(resnik, Err(ResnikError::UnequalOccurrenceSize { expected: 3, found: 0 }));
    // Negative occurrences found
    let resnik = graph.resnik(&[-1.0, -2.0, -3.0]);
    assert_eq!(resnik, Err(ResnikError::NegativeOccurrence));
    // Non finite cases found
    let resnik = graph.resnik(&[f64::INFINITY, f64::NAN, f64::INFINITY]);
    assert_eq!(resnik, Err(ResnikError::NonFiniteOccurrence));
    // No occurrences above zero
    let resnik = graph.resnik(&[0.0, 0.0, 0.0]);
    assert_eq!(resnik, Err(ResnikError::NoOccurrencesAboveZero));
    Ok(())
}
