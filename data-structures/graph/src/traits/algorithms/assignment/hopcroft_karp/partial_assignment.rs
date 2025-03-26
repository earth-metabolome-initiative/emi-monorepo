//! Partial assignment for Hopcroft-Karp algorithm.

use std::collections::HashMap;

use algebra::prelude::{IntoUsize, Number};

use super::HopcroftKarpError;
use crate::traits::MonoplexBipartiteGraph;

/// Struct representing a partial assignment.
pub struct PartialAssignment<'a, G: MonoplexBipartiteGraph + ?Sized, Distance = u16> {
    predecessors: Vec<Option<G::LeftNodeId>>,
    successors: Vec<Option<G::RightNodeId>>,
    left_distances: Vec<Distance>,
	null_distance: Distance,
    graph: &'a G,
}

impl<'a, G: MonoplexBipartiteGraph + ?Sized, Distance: Number>
    From<PartialAssignment<'a, G, Distance>> for Vec<(G::LeftNodeId, G::RightNodeId)>
{
    fn from(assignment: PartialAssignment<'a, G, Distance>) -> Self {
        assignment
            .successors
            .iter()
            .copied()
            .filter_map(|right_node_id: Option<G::RightNodeId>| {
                let Some(right_node_id) = right_node_id else {
                    return None;
                };
                let Some(left_node_id) = assignment.predecessors[right_node_id.into_usize()] else {
                    return None;
                };
                Some((left_node_id, right_node_id))
            })
            .collect()
    }
}

impl<'a, G: MonoplexBipartiteGraph + ?Sized, Distance: Number>
    From<PartialAssignment<'a, G, Distance>> for HashMap<G::LeftNodeId, G::RightNodeId>
{
    fn from(assignment: PartialAssignment<'a, G, Distance>) -> Self {
        assignment
            .successors
            .iter()
            .copied()
            .filter_map(|right_node_id: Option<G::RightNodeId>| {
                let Some(right_node_id) = right_node_id else {
                    return None;
                };
                let Some(left_node_id) = assignment.predecessors[right_node_id.into_usize()] else {
                    return None;
                };
                Some((left_node_id, right_node_id))
            })
            .collect()
    }
}

impl<'a, G: MonoplexBipartiteGraph + ?Sized, Distance: Number> From<&'a G>
    for PartialAssignment<'a, G, Distance>
{
    fn from(graph: &'a G) -> Self {
        let predecessors = vec![None; graph.number_of_right_nodes().into_usize()];
        let successors = vec![None; graph.number_of_left_nodes().into_usize()];
        let left_distances = vec![Distance::MAX; graph.number_of_left_nodes().into_usize()];
        PartialAssignment { predecessors, successors, left_distances, graph, null_distance: Distance::MAX }
    }
}

impl<'a, G: MonoplexBipartiteGraph + ?Sized, Distance: Number> PartialAssignment<'a, G, Distance> {
    /// Returns whether the provided left node id has a successor.
    pub(super) fn has_successor(&self, left_node_id: G::LeftNodeId) -> bool {
        self.successors[left_node_id.into_usize()].is_some()
    }

    pub(super) fn bfs(&mut self) -> Result<bool, HopcroftKarpError> {
        let mut frontier = Vec::new();
        for left_node_id in self.graph.left_node_ids() {
            if !self.has_successor(left_node_id) {
                self.left_distances[left_node_id.into_usize()] = Distance::ZERO;
                frontier.push(left_node_id);
            } else {
				self.left_distances[left_node_id.into_usize()] = Distance::MAX;
			}
        }

		// 	if Dist[u] < Dist[NIL] then
		// 		for each v in Adj[u] do
		// 			if Dist[Pair_V[v]] = ∞ then
		// 				Dist[Pair_V[v]] := Dist[u] + 1
		// 				Enqueue(Q, Pair_V[v])
		// return Dist[NIL] ≠ ∞

        self.null_distance = Distance::MAX;

        while !frontier.is_empty() {
            let mut tmp_frontier = Vec::new();
            for left_node_id in frontier {
                if self.left_distance(Some(left_node_id)) < self.null_distance {
                    if self.left_distance(Some(left_node_id)) == Distance::MAX - Distance::ONE {
                        return Err(HopcroftKarpError::InsufficientDistanceType);
                    }
                    for right_node_id in self.graph.successors(left_node_id) {
                        match self.predecessors[right_node_id.into_usize()] {
                            Some(predecessor) => {
                                if self.left_distances[predecessor.into_usize()] == Distance::MAX {
                                    self.left_distances[predecessor.into_usize()] =
                                        left_distance + Distance::ONE;
                                    tmp_frontier.push(predecessor);
                                }
                            }
                            None => {
                                self.null_distance = left_distance + Distance::ONE;
                            }
                        }
                    }
                }
            }
            frontier = tmp_frontier;
        }

        Ok(self.null_distance != Distance::MAX)
    }

	/// Returns the distance of the provided left node id.
	/// 
	/// # Arguments
	/// 
	/// * `left_node_id`: The identifier of the left node.
	/// 
	fn left_distance(&self, left_node_id: Option<G::LeftNodeId>) -> Distance {
		let Some(left_node_id) = left_node_id else {
			return self.null_distance;
		};
		self.left_distances[left_node_id.into_usize()]
	}

	/// Returns a mutable reference to the distance of the provided left node id.
	///
	/// # Arguments
	/// 
	/// * `left_node_id`: The identifier of the left node.
	/// 
	fn left_distance_mut(&mut self, left_node_id: Option<G::LeftNodeId>) -> &mut Distance {
		let Some(left_node_id) = left_node_id else {
			return &mut self.null_distance;
		};
		&mut self.left_distances[left_node_id.into_usize()]
	}

    pub(super) fn dfs(&mut self, left_node_id: Option<G::LeftNodeId>) -> bool {
		let Some(left_node_id) = left_node_id else {
			return true;
		};
        for right_node_id in self.graph.successors(left_node_id) {
            if let Some(predecessor) = self.predecessors[right_node_id.into_usize()] {
                if self.left_distances[predecessor.into_usize()]
                    == self.left_distances[left_node_id.into_usize()] + Distance::ONE
                {
                    if self.dfs(predecessor) {
                        self.successors[left_node_id.into_usize()] = Some(right_node_id);
                        self.predecessors[right_node_id.into_usize()] = Some(left_node_id);
                        return true;
                    }
                }
            }
        }

        self.left_distances[left_node_id.into_usize()] = Distance::MAX;
        false
    }
}
