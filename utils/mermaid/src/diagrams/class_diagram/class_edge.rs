//! Submodule defining an edge which may be used in a flowchart diagram
//! in Mermaid syntax.

use std::{fmt::Display, rc::Rc};

use crate::{
    diagrams::class_diagram::{class_edge::multiplicity::Multiplicity, class_node::ClassNode},
    shared::{ArrowShape, GenericEdge, LineStyle, NODE_LETTER},
    traits::{Edge, node::Node},
};

pub mod builder;
pub mod multiplicity;
pub use builder::ClassEdgeBuilder;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// An edge in a Mermaid class diagram, connecting two class nodes with optional
/// multiplicities.
pub struct ClassEdge {
    /// Underlying generic edge.
    edge: GenericEdge<ClassNode>,
    /// Left multiplicity of the edge.
    left_multiplicity: Option<Multiplicity>,
    /// Right multiplicity of the edge.
    right_multiplicity: Option<Multiplicity>,
}

impl Edge for ClassEdge {
    type Builder = ClassEdgeBuilder;
    type Node = ClassNode;

    fn label(&self) -> Option<&str> {
        self.edge.label()
    }

    fn source(&self) -> &Rc<Self::Node> {
        self.edge.source()
    }

    fn destination(&self) -> &Rc<Self::Node> {
        self.edge.destination()
    }

    fn line_style(&self) -> LineStyle {
        self.edge.line_style()
    }

    fn classes(&self) -> impl Iterator<Item = &crate::shared::StyleClass> {
        std::iter::empty()
    }

    fn left_arrow_shape(&self) -> Option<ArrowShape> {
        self.edge.left_arrow_shape()
    }

    fn right_arrow_shape(&self) -> Option<ArrowShape> {
        self.edge.right_arrow_shape()
    }
}

impl Display for ClassEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{NODE_LETTER}{} {left_multiplicity}{left_arrow}{segment}{right_arrow}{right_multiplicity} {NODE_LETTER}{}{}",
            self.source().id(),
            self.destination().id(),
            self.label().map_or_else(String::new, |label| format!(" : \"`{label}`\"")),
            left_multiplicity =
                self.left_multiplicity.as_ref().map_or_else(String::new, |lm| format!("{lm} ")),
            left_arrow = self.left_arrow_shape().as_ref().map_or_else(|| "", |shape| shape.left()),
            segment = match self.line_style() {
                LineStyle::Solid => "--",
                LineStyle::Thick => "==",
                LineStyle::Dashed => "..",
            },
            right_arrow =
                self.right_arrow_shape().as_ref().map_or_else(|| "", |shape| shape.right()),
            right_multiplicity =
                self.right_multiplicity.as_ref().map_or_else(String::new, |rm| format!(" {rm}")),
        )
    }
}
