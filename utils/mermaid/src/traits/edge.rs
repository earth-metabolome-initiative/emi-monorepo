//! Submodule defining the trait for representing edges in Mermaid diagrams.

use std::fmt::Display;

use crate::traits::{EdgeBuilder, Node};

/// Trait representing an edge in a Mermaid diagram.
pub trait Edge: Display {
    /// Type of the builder used to construct this edge.
    type Builder: EdgeBuilder<Edge = Self>;
    /// Type of the node this edge connects to.
    type Node: Node;

    /// Returns a reference to the label of the edge.
    fn label(&self) -> &str;

    /// Returns a reference to the source node of the edge.
    fn source(&self) -> &Self::Node;

    /// Returns a reference to the destination node of the edge.
    fn destination(&self) -> &Self::Node;
}
