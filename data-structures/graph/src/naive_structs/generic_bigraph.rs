//! Submodule providing a naively implemented `GenericBiGraph`.

use super::generic_monoplex_bipartite_graph_builder::MonoplexBipartiteGraphBuilderError;

/// Struct representing a generic bigraph.
pub struct GenericBiGraph<LeftNodes, RightNodes, Edges> {
    /// The left nodes of the graph.
    left_nodes: LeftNodes,
    /// The right nodes of the graph.
    right_nodes: RightNodes,
    /// The edges of the graph.
    edges: Edges,
}

impl<LeftNodes, RightNodes, Edges> TryFrom<(LeftNodes, RightNodes, Edges)>
    for GenericBiGraph<LeftNodes, RightNodes, Edges>
{
    type Error = MonoplexBipartiteGraphBuilderError;

    fn try_from(
        (left_nodes, right_nodes, edges): (LeftNodes, RightNodes, Edges),
    ) -> Result<Self, Self::Error> {
        Ok(Self { left_nodes, right_nodes, edges })
    }
}
