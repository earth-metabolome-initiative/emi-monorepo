//! Submodule providing `Resnik` trait
//! implementation.

mod error;
pub use error::ResnikError;
use functional_properties::prelude::ScalarSimilarity;
use algebra::prelude::Kahn;
use numeric_common_traits::prelude::IntoUsize;
use crate::traits::{RootNodes, MonoplexMonopartiteGraph};
use crate::traits::edges::Edges;

/// Struct to provide methods to compute Resnik Similarity Score
#[derive(Debug, PartialEq)]
pub struct ResnikResult<'graph, G: ?Sized> {
    /// The graph to be analyzed
    graph: &'graph G,

}

impl<'graph, G> ScalarSimilarity<G::NodeId, G::NodeId> for ResnikResult<'graph, G> where G: MonoplexMonopartiteGraph {
    type Similarity = f64;
    fn similarity(&self, left: &G::NodeId, right: &G::NodeId) -> Self::Similarity {
        1.0
    }   
}

/// Trait providing the `Resnik` method 
/// TODO: Finish
pub trait Resnik: MonoplexMonopartiteGraph {
    /// The method for applying the Resnik
    /// 
    /// # Errors 
    /// - If the graph is not a dag
    fn resnik(&self, occurrences: Option<&[usize]>) -> Result<ResnikResult<'_, Self>, ResnikError> {
        // Check whether the graph is a DAG (characterize by having no cycles)
        let _topological_ordering = self.edges().matrix().kahn()?;
        if let Some(occurences) = occurrences {
            if occurences.len() != self.number_of_nodes().into_usize() {
                return Err(ResnikError::IneqOccurenceSize { expected: self.number_of_nodes().into_usize(), found: occurences.len() });
            }
        }
        let root_nodes = self.root_nodes();
        let mut propagated_occurences = vec![0_usize;self.number_of_nodes().into_usize()];
        for root_node in root_nodes {
            propagate_occurences(self, root_node, occurrences, &mut propagated_occurences);
        }

        Ok(ResnikResult { graph: self })
    } 


}

/// Helper function for propagating occurences, returns a usize 
fn propagate_occurences<G>(graph:&G, node: G::NodeId, occurrences: Option<&[usize]>, propagated_occurences: &mut [usize]) -> usize where G:MonoplexMonopartiteGraph + ?Sized {
    let mut propagated_occurence = occurrences.as_ref().map(|occurences|{
        occurences[node.into_usize()]
    }).unwrap_or_default();
    for successor in graph.successors(node) {
        propagated_occurence += propagate_occurences(graph, successor, occurrences, propagated_occurences);
    }
    propagated_occurences[node.into_usize()] = propagated_occurence;
    propagated_occurence
}

impl <G>Resnik for G  where G: MonoplexMonopartiteGraph {

}

