//! Submodule defining the `Diagram` trait for Mermaid diagrams.

use std::fmt::Display;

use crate::{
    shared::StyleClass,
    traits::{DiagramBuilder, Edge, Node},
};

/// Trait representing a Mermaid diagram.
pub trait Diagram: Display {
    /// Type of the associated builder for this diagram.
    type Builder: DiagramBuilder<Diagram = Self, Node = Self::Node, Edge = Self::Edge>;
    /// Type of the node used in the diagram.
    type Node: Node;
    /// Type of edge used in the diagram.
    type Edge: Edge<Node = Self::Node>;

    /// Iterates across the style classes associated with this diagram.
    fn style_classes(&self) -> impl Iterator<Item = &StyleClass>;

    /// Returns whether the diagram contains a certain node.
    fn contains_node<N>(&self, node: N) -> bool
    where
        N: AsRef<Self::Node>;

    /// Iterates across the nodes in this diagram.
    fn nodes(&self) -> impl Iterator<Item = &Self::Node>;

    /// Iterates across the edges in this diagram.
    fn edges(&self) -> impl Iterator<Item = &Self::Edge>;
}
