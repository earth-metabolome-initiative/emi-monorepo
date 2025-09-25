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
pub struct ResnikResult<'graph, G: ?Sized + MonoplexMonopartiteGraph>  {
    /// The graph to be analyzed
    graph: &'graph G,
    /// Contains the information content of each node of the graph
    information_contents: Vec<f64>,
    /// Root nodes of the graph
    root_nodes: Vec<G::NodeId>,
    /// Maximum Information Content
    max_information_content: f64,
}

impl<'graph, G> ScalarSimilarity<G::NodeId, G::NodeId> for ResnikResult<'graph, G> where G: MonoplexMonopartiteGraph {
    type Similarity = f64;
    fn similarity(&self, left: &G::NodeId, right: &G::NodeId) -> Self::Similarity {
        if left == right {
            return 1.0;
        }
        let mut max_score = 0.0;
        for root_node in &self.root_nodes {
            if let InformationContentSearch::BothNodesFound(score ) = information_content_search(self, left, right, root_node) {
                if max_score < score {
                    max_score = score;
                }
            }
        }
        max_score / self.max_information_content
    }   
}

fn information_content_search<'graph, G>(resnik_result: &ResnikResult<'graph, G>, left: &G::NodeId, right: &G::NodeId, current_node: &G::NodeId) -> InformationContentSearch where G:MonoplexMonopartiteGraph + ?Sized {
    // Base cases
    let mut state = if left == current_node {
        InformationContentSearch::LeftNodeFound
    } else if right == current_node {
        InformationContentSearch::RightNodeFound
    } else {
        InformationContentSearch::NotFound
    };
    for successor in resnik_result.graph.successors(*current_node) {
        state = match (state,information_content_search(resnik_result, left, right, &successor)) {
            (InformationContentSearch::LeftNodeFound, InformationContentSearch::RightNodeFound) | (InformationContentSearch::RightNodeFound, InformationContentSearch::LeftNodeFound)=> InformationContentSearch::BothNodesFound(resnik_result.information_contents[current_node.into_usize()]), 
            (InformationContentSearch::NotFound, other ) => other,
            (InformationContentSearch::BothNodesFound(current), InformationContentSearch::BothNodesFound(other)) => InformationContentSearch::BothNodesFound(if current > other {
                current
            } else {
                other
            }), 
            (_, InformationContentSearch::BothNodesFound(other)) => InformationContentSearch::BothNodesFound(other),
            (_, InformationContentSearch::NotFound) | (InformationContentSearch::LeftNodeFound, InformationContentSearch::LeftNodeFound) | (InformationContentSearch::RightNodeFound, InformationContentSearch::RightNodeFound) | (InformationContentSearch::BothNodesFound(_), _) => continue,

        };
    }
    state
}

/// Enum for tracking IC Search possibilities
#[derive(Clone, Copy)]
enum InformationContentSearch {
    NotFound,
    LeftNodeFound,
    RightNodeFound,
    BothNodesFound(f64),
}

/// Trait providing the `Resnik` method 
/// TODO: Finish
pub trait Resnik: MonoplexMonopartiteGraph {
    /// The method for applying the Resnik
    /// 
    /// # Errors 
    /// - If the graph is not a dag
    /// - If the occurrences are not equal
    fn resnik(&self, occurrences: &[f64]) -> Result<ResnikResult<'_, Self>, ResnikError> {
        // Check whether the graph is a DAG (characterize by having no cycles)
        let _topological_ordering = self.edges().matrix().kahn()?;
        if occurrences.len() != self.number_of_nodes().into_usize() {
            return Err(ResnikError::InequalOccurrenceSize { expected: self.number_of_nodes().into_usize(), found: occurrences.len() });
        }
        for occurrence in occurrences {
            if !occurrence.is_finite() {
                return Err(ResnikError::NonFiniteOccurrence);
            }
            if *occurrence < 0.0 {
                return  Err(ResnikError::NegativeOccurrence);
            }
        }

        let root_nodes: Vec<<<Self as MonoplexMonopartiteGraph>::MonoplexMonopartiteEdges as Edges>::SourceNodeId> = self.root_nodes();
        let mut propagated_occurrences = vec![None;self.number_of_nodes().into_usize()];
        for root_node in &root_nodes {
            propagate_occurrences(self, *root_node, occurrences, &mut propagated_occurrences);
        }
        let total_occurrences: f64 = occurrences.iter().sum();
        // maximum information content, initialized to 0
        let mut max_information_content = 0.0;
        // Compute the information content
        let information_contents = propagated_occurrences.into_iter().map(|propagated_occurrence| {
            let propagated_occurrence = propagated_occurrence.unwrap_or_default(); 
            if propagated_occurrence == 0.0 {
                0.0
            } else {
                let information_content = -(propagated_occurrence/total_occurrences).ln();
                if information_content > max_information_content {
                    max_information_content = information_content;
                }
                information_content
            }
        }
        ).collect::<Vec<f64>>();
        if max_information_content == 0.0 {
            return Err(ResnikError::NoOccurrencesAboveZero);
        }
        Ok(ResnikResult { graph: self, information_contents, root_nodes, max_information_content })
    } 


}

/// Helper function for propagating occurrences, returns a usize 
fn propagate_occurrences<G>(graph:&G, node: G::NodeId, occurrences: &[f64], propagated_occurrences: &mut [Option<f64>]) -> f64 where G:MonoplexMonopartiteGraph + ?Sized {
    // Check whether node has been visited by propagation
    if let Some(propagated_occurrence) = propagated_occurrences[node.into_usize()] {
        return propagated_occurrence;
    }
    let mut propagated_occurrence = occurrences[node.into_usize()];
    // Recursively call propagate_occurrences()
    for successor in graph.successors(node) {
        
        propagated_occurrence += propagate_occurrences(graph, successor, occurrences, propagated_occurrences);
    }
    propagated_occurrences[node.into_usize()] = Some(propagated_occurrence);
    propagated_occurrence
}

impl <G>Resnik for G  where G: MonoplexMonopartiteGraph {

}

