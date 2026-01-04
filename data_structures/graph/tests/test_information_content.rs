//! Test submodule for the `Information Content` train
use algebra::impls::{CSR2D, SquareCSR2D};
use graph::{
    prelude::{DiEdgesBuilder, DiGraph, GenericVocabularyBuilder, InformationContent},
    traits::{EdgesBuilder, VocabularyBuilder, information_content::InformationContentError},
};
use sorted_vec::prelude::SortedVec;
#[test]
fn test_information_content_incorrect_occurrences() -> Result<(), Box<dyn std::error::Error>> {
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
    // length mismatch
    let information_content = graph.information_content(&Vec::new());
    assert_eq!(
        information_content,
        Err(InformationContentError::UnequalOccurrenceSize { expected: 3, found: 0 })
    );
    // No occurrences above zero found
    let ic = graph.information_content(&[1, 1, 0]);
    assert_eq!(ic, Err(InformationContentError::SinkNodeZeroOccurrence));
    //
    let _ic = graph.information_content(&[0, 0, 1])?;

    Ok(())
}

#[test]
fn test_ic_not_dag() -> Result<(), Box<dyn std::error::Error>> {
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
    let graph: DiGraph<usize> = DiGraph::try_from((nodes, edges))?;
    let ic = graph.information_content(&[1, 1, 1]);
    assert_eq!(ic, Err(InformationContentError::NotDag));
    Ok(())
}
