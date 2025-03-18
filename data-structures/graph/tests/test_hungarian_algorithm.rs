//! Test submodule evaluating several corner cases in the Hungarian algorithm.

use ::graph::prelude::*;
use algebra::prelude::ValuedCSR2D;
use sorted_vec::prelude::SortedVec;

#[test]
/// Test checking that the hungarian completes nominally on an empty graph, i.e.
/// a graph with no vertices (or, of course, edges).
pub fn test_hungarian_on_empty_graph() {
    let graph: GenericBiGraph<
        SortedVec<usize>,
        SortedVec<usize>,
        ValuedCSR2D<usize, usize, usize, f64>,
    > = WeightedBiGraph::default();
    let matching = graph.hungarian().unwrap();
    assert_eq!(matching.assignments().len(), 0);
}
