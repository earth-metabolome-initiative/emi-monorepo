//! Implementation for the 'Wu Palmer' trait based on the algorithm
//! implementation

use core::f64;

use algebra::prelude::{Kahn, KahnError};
use functional_properties::prelude::ScalarSimilarity;

use crate::traits::{Edges, MonoplexMonopartiteGraph, algorithms::root_nodes::RootNodes};

/// Struct for the Wu-Palmer similarity trait
#[derive(Debug)]
pub struct WuPalmerResult<'graph, G: ?Sized + MonoplexMonopartiteGraph> {
    /// the graph to be analyzed
    graph: &'graph G,
    /// Root nodes of the graph
    root_nodes: Vec<G::NodeId>,
}

/// Trait providing the `Wu-Palmer` depth based similarity
///
/// # Reference
/// The implementation of the Wu-Palmer based similarity score as described in [Verb Semantics and Lexical Selection](https://arxiv.org/pdf/cmp-lg/9406033)
pub trait WuPalmer: MonoplexMonopartiteGraph {
    /// The method for applying the Wu-Palmer algorithm
    ///
    /// # Errors
    /// - If the graph is not a dag
    fn wu_palmer(&self) -> Result<WuPalmerResult<'_, Self>, KahnError> {
        // Check whether the graph is a DAG (characterize by having no cycles)
        let _topological_ordering = self.edges().matrix().kahn()?;
        let root_nodes = self.root_nodes();
        Ok(WuPalmerResult { graph: self, root_nodes })
    }
}

impl<G> ScalarSimilarity<G::NodeId, G::NodeId> for WuPalmerResult<'_, G>
where
    G: MonoplexMonopartiteGraph,
{
    type Similarity = f64;
    #[allow(clippy::cast_precision_loss)]
    fn similarity(&self, left: &G::NodeId, right: &G::NodeId) -> Self::Similarity {
        if left == right {
            return 1.0;
        }
        let mut max_similarity = 0.0;
        for root_node in &self.root_nodes {
            let (Some(n1), Some(n2), n3) =
                wu_palmer_depth(self.graph, 0, *root_node, *left, *right)
            else {
                continue;
            };
            let mut denominator = n1 as f64 + n2 as f64 + (2.0 * n3 as f64);
            if denominator < f64::EPSILON {
                denominator = f64::EPSILON;
            }
            let similarity = (2.0 * n3 as f64) / denominator;
            if similarity > max_similarity {
                max_similarity = similarity;
            }
        }
        max_similarity
    }
}

fn wu_palmer_depth<G>(
    graph: &G,
    depth: usize,
    current_node: G::NodeId,
    left: G::NodeId,
    right: G::NodeId,
) -> (Option<usize>, Option<usize>, usize)
where
    G: MonoplexMonopartiteGraph + ?Sized,
{
    let (mut n1, mut n2, mut n3) = (None, None, depth);
    if current_node == left {
        n1 = Some(depth);
    } else if current_node == right {
        n2 = Some(depth);
    }

    for successor in graph.successors(current_node) {
        match wu_palmer_depth(graph, depth + 1, successor, left, right) {
            (Some(rec_n1), None, _) => {
                n1 = Some(n1.map_or(rec_n1, |n1| if n1 < rec_n1 { n1 } else { rec_n1 }));
            }
            (None, Some(rec_n2), _) => {
                n2 = Some(n2.map_or(rec_n2, |n2| if n2 < rec_n2 { n2 } else { rec_n2 }));
            }
            (None, None, _) => {}
            (rec_n1, rec_n2, rec_n3) => {
                n3 = if n3 < rec_n3 {
                    n1 = rec_n1;
                    n2 = rec_n2;
                    rec_n3
                } else {
                    n3
                };
            }
        }
    }
    (n1, n2, n3)
}

impl<G> WuPalmer for G where G: MonoplexMonopartiteGraph {}
