//! Test submodule for the `TopologicalSorting` trait.

use algebra::impls::{SquareCSR2D, CSR2D};
use graph::{
    prelude::{
        Builder, DiEdgesBuilder, DiGraph, GenericMonoplexMonopartiteGraphBuilder,
        GenericVocabularyBuilder, MonopartiteGraph, MonoplexGraph, TopologicalSorting,
    },
    traits::{
        EdgesBuilder, MonopartiteGraphBuilder, MonoplexGraphBuilder, VocabularyBuilder,
        topological_sorting::TopologicalSortingError,
    },
};
use sorted_vec::prelude::SortedVec;

#[test]
fn test_topological_sorting() -> Result<(), Box<dyn std::error::Error>> {
    let nodes: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
    let edges: Vec<(usize, usize)> = vec![
        (0, 1),
        (0, 2),
        (1, 3),
        (1, 4),
        (2, 0),
        (2, 1),
        (2, 3),
        (3, 4),
        (4, 5),
        (5, 1),
        (5, 3),
    ];
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

    assert_eq!(graph.number_of_nodes(), 6);
    assert_eq!(graph.number_of_edges(), 11);

    assert_eq!(graph.topological_sort_from_node(0).unwrap(), vec![0, 1, 2, 3, 4, 5]);
    assert_eq!(
        graph.topological_sort_from_node(1).unwrap_err(),
        TopologicalSortingError::UnreachableNodes
    );
    assert_eq!(graph.topological_sort_from_node(2).unwrap(), vec![1, 2, 0, 3, 4, 5]);

    Ok(())
}
