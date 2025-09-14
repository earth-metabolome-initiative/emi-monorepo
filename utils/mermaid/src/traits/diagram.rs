//! Submodule defining the `Diagram` trait for Mermaid diagrams.

use std::{fmt::Display, rc::Rc};

use crate::{
    shared::StyleClass,
    traits::{Configuration, DiagramBuilder, Edge, Node},
};

/// Trait representing a Mermaid diagram.
pub trait Diagram {
    /// Type of the associated builder for this diagram.
    type Builder: DiagramBuilder<Diagram = Self, Node = Self::Node, Edge = Self::Edge>;
    /// Type of the node used in the diagram.
    type Node: Node + Display;
    /// Type of edge used in the diagram.
    type Edge: Edge<Node = Self::Node> + Display;
    /// The configuration options for the diagram.
    type Configuration: Configuration + Display;

    /// Returns the configuration of the diagram.
    fn configuration(&self) -> &Self::Configuration;

    /// Returns an iterator over the nodes in the diagram.
    fn nodes(&self) -> impl Iterator<Item = &Self::Node>;

    /// Returns an iterator over the edges in the diagram.
    fn edges(&self) -> impl Iterator<Item = &Self::Edge>;

    /// Returns an iterator over the style classes associated with the diagram.
    fn style_classes(&self) -> impl Iterator<Item = &StyleClass>;

    /// Returns the reference to the requested node by ID if it exists.
    fn get_node_by_id(&self, id: u64) -> Option<Rc<Self::Node>>;

    /// Returns the reference to the requested style class by name if it exists.
    fn get_style_class_by_name(&self, name: &str) -> Option<Rc<StyleClass>>;
}
