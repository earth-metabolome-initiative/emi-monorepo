//! Submodule defining the `DiagramBuilder` trait for Mermaid diagrams.

use common_traits::prelude::Builder;

use crate::{
    shared::{StyleClassBuilder, StyleClassError},
    traits::{Diagram, Edge, Node, NodeBuilder},
};

/// Trait defining the builder for Mermaid diagrams.
pub trait DiagramBuilder: Builder {
    /// Type of the diagram that this builder constructs.
    type Diagram: Diagram<Builder = Self, Node = Self::Node, Edge = Self::Edge>;
    /// Type of the node used in the diagram.
    type Node: Node;
    /// Type of edge used in the diagram.
    type Edge: Edge<Node = Self::Node>;
    /// The type of node builder used to create nodes in the diagram.
    type NodeBuilder: NodeBuilder<Node = Self::Node>;

    /// Adds a style class to the diagram being built.
    fn style_class(&mut self, style_class: StyleClassBuilder)
    -> Result<&mut Self, StyleClassError>;

    /// Returns the number of nodes currently in the diagram.
    fn number_of_nodes(&self) -> usize;

    /// Adds a node to the diagram being built.
    fn node(
        &mut self,
        node: Self::NodeBuilder,
    ) -> Result<&mut Self, <Self::NodeBuilder as Builder>::Error>;
}
