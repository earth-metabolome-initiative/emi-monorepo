//! Submodule providing the randomized dag trait to generate randomized dag with
//! provided parameters.

use std::collections::HashSet;

use algebra::prelude::SparseMatrixMut;

use crate::traits::{GrowableEdges, MonoplexGraph, MonoplexMonopartiteGraph};

/// Trait providing randomized dag method
pub trait RandomizedDAG: MonoplexGraph {
    /// returns a randomized dag within given parameters
    ///
    /// # Arguments
    /// - `seed`: the random seed the dag is generated from
    /// - `nodes`: number of the nodes the dag will have
    fn randomized_dag(seed: u64, nodes: u64) -> Self;
}

impl<G> RandomizedDAG for G
where
    G: MonoplexMonopartiteGraph<Nodes = u64> + From<(G::Nodes, G::MonoplexMonopartiteEdges)>,
    G::MonoplexMonopartiteEdges: GrowableEdges<EdgeId = u64, Edge = (u64, u64)>,
    <G::MonoplexMonopartiteEdges as GrowableEdges>::GrowableMatrix:
        SparseMatrixMut<MinimalShape = u64>,
{
    fn randomized_dag(seed: u64, nodes: u64) -> Self {
        let mut xorshift = XorShift64::from(seed);
        let number_of_edges = xorshift.next().unwrap() % (nodes * (nodes - 1) / 2);
        let mut edge_tuples = HashSet::with_capacity(usize::try_from(number_of_edges).unwrap());
        while edge_tuples.len() < usize::try_from(number_of_edges).unwrap() {
            let seed1 = xorshift.next().unwrap();
            let seed2 = xorshift.next().unwrap();
            let mut src = seed1 % nodes;
            let mut dst = seed2 % nodes;
            if src == dst {
                continue;
            }
            if src > dst {
                core::mem::swap(&mut src, &mut dst);
            }
            edge_tuples.insert((src, dst));
        }
        let mut sorted_edge_tuples: Vec<(u64, u64)> = edge_tuples.into_iter().collect();
        sorted_edge_tuples.sort_unstable();
        let mut edges = G::MonoplexMonopartiteEdges::with_shaped_capacity(nodes, number_of_edges);
        for (src, dst) in sorted_edge_tuples {
            edges.add((src, dst)).unwrap();
        }
        G::from((nodes, edges))
    }
}

/// Struct for storing the `XorShift64` state
pub struct XorShift64(u64);

impl From<u64> for XorShift64 {
    fn from(state: u64) -> Self {
        Self(state)
    }
}

impl Iterator for XorShift64 {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        Some(x)
    }
}
