//! Submodule providing the randomized dag trait to generate randomized dag with
//! provided parameters.

use std::collections::HashSet;

use algebra::prelude::{SparseMatrixMut, SquareMatrix};
use common_traits::prelude::Builder;
use crate::prelude::generic_monoplex_monopartite_graph_builder::MonoplexMonopartiteGraphBuilderError;
use crate::{prelude::GenericMonoplexMonopartiteGraphBuilder, traits::{edge, GrowableEdges, MonopartiteEdges, MonopartiteGraphBuilder, MonoplexGraph, MonoplexGraphBuilder, MonoplexMonopartiteGraph}};

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
    G: MonoplexMonopartiteGraph<NodeId = u64, Nodes = u64> + TryFrom<(G::Nodes,G::MonoplexMonopartiteEdges), Error = MonoplexMonopartiteGraphBuilderError>,
    G::MonoplexMonopartiteEdges: GrowableEdges <EdgeId = u64, Edge = (u64,u64),> + MonopartiteEdges<NodeId = u64>,
    <G::MonoplexMonopartiteEdges as GrowableEdges>::GrowableMatrix:SparseMatrixMut<MinimalShape = u64>
{
    fn randomized_dag(seed: u64, nodes: u64) -> Self {
        let mut xorshift  = XorShift64::new(seed);
        let number_of_edges = xorshift.next() % (nodes * (nodes-1)/2);
        let mut edge_tuples = HashSet::with_capacity(number_of_edges as usize);
        while edge_tuples.len() < number_of_edges as usize {
            let seed1 = xorshift.next();
            let seed2 = xorshift.next();
            let mut src = seed1 % nodes;
            let mut dst = seed2 % nodes;
            if src > dst {
                core::mem::swap(&mut src, &mut dst);
            }
            edge_tuples.insert((src,dst));
        }
        let mut sorted_edge_tuples:Vec<(u64,u64)> = edge_tuples.into_iter().collect();
        sorted_edge_tuples.sort_unstable();
        let mut edges = G::MonoplexMonopartiteEdges::with_shaped_capacity(nodes, number_of_edges);
        for (src, dst) in sorted_edge_tuples {
            edges.add((src,dst)).unwrap();
        }
        GenericMonoplexMonopartiteGraphBuilder::default().nodes(nodes).edges(edges).build().unwrap()

    }
}

/// Struct for storing the XorShift state
pub struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    /// Creates a new XorShift64 generator with a non-zero seed
    pub fn new(seed: u64) -> Self {
        assert!(seed != 0, "Seed must be non-zero");
        Self { state: seed }
    }

    /// Generates the next random u64
    pub fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}
