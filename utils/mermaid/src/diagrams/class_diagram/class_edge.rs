//! Submodule providing a struct representing an edge for class diagrams in
//! Mermaid.

pub mod builder;
use std::{fmt::Display, rc::Rc};

pub use builder::ClassEdgeBuilder;

use crate::{
    diagrams::class_diagram::class_node::ClassNode,
    shared::{ArrowShape, LineStyle, NODE_LETTER},
    traits::{Edge, node::Node},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents an edge in a class diagram, connecting two class nodes.
pub struct ClassEdge {
    /// The label of the edge, typically representing the relationship type.
    label: String,
    /// The source class node of the edge.
    source: Rc<ClassNode>,
    /// The destination class node of the edge.
    destination: Rc<ClassNode>,
    /// The line style of the link.
    line_style: LineStyle,
    /// The left arrow shape of the link, if any.
    left_arrow_shape: Option<ArrowShape>,
    /// The right arrow shape of the link, if any.
    right_arrow_shape: Option<ArrowShape>,
}

impl Edge for ClassEdge {
    type Builder = ClassEdgeBuilder;
    type Node = ClassNode;

    fn label(&self) -> &str {
        &self.label
    }

    fn source(&self) -> &Self::Node {
        &self.source
    }

    fn destination(&self) -> &Self::Node {
        &self.destination
    }
}

impl Display for ClassEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{NODE_LETTER}{} {left_arrow}{segment}{right_arrow} {NODE_LETTER}{} : {}",
            self.source.id(),
            self.destination.id(),
            self.label,
            left_arrow = self.left_arrow_shape.as_ref().map_or_else(|| "", |shape| shape.left()),
            right_arrow = self.right_arrow_shape.as_ref().map_or_else(|| "", |shape| shape.right()),
            segment = match self.line_style {
                LineStyle::Solid => "--",
                LineStyle::Thick => "==",
                LineStyle::Dashed => "..",
            }
        )
    }
}
