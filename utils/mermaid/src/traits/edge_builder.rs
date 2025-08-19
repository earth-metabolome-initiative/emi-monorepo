//! Submodule defining the trait for structs constructing edges in Mermaid
//! diagrams.

use std::rc::Rc;

use common_traits::prelude::Builder;

use crate::traits::{Edge, Node};

/// Trait representing an edge builder in a Mermaid diagram.
pub trait EdgeBuilder: Builder {
    /// The type of edge this builder constructs.
    type Edge: Edge<Builder = Self, Node = Self::Node>;
    /// Type of the node this edge connects to.
    type Node: Node;

    /// Set the label for this edge.
    fn label<S: ToString>(&mut self, label: S) -> &mut Self;

    /// Set the source node for this edge.
    fn source(&mut self, node: Rc<Self::Node>) -> &mut Self;

    /// Set the target node for this edge.
    fn destination(&mut self, node: Rc<Self::Node>) -> &mut Self;

    /// Build the edge from the current state of the builder.
    fn build(self) -> Self::Edge;
}
