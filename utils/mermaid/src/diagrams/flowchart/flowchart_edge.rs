//! Submodule defining an edge which may be used in a flowchart diagram
//! in Mermaid syntax.

use std::{fmt::Display, rc::Rc};

use crate::{
    diagrams::flowchart::{curve_styles::CurveStyle, flowchart_node::FlowchartNode},
    shared::{
        ArrowShape, EDGE_LETTER, GenericEdge, LineStyle, NODE_LETTER, StyleClass, StyleProperty,
    },
    traits::{Edge, node::Node},
};

pub mod builder;
pub use builder::{FlowchartEdgeAttribute, FlowchartEdgeBuilder};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlowchartEdge {
    /// Unique identifier for the edge.
    id: u32,
    /// Underlying generic edge.
    edge: GenericEdge<FlowchartNode>,
    /// Classes associated with the edge, used for styling.
    style_classes: Vec<Rc<StyleClass>>,
    /// Styling properties for the edge, such as color and font.
    style_properties: Vec<StyleProperty>,
    /// The curve style of the edge.
    curve_style: CurveStyle,
    /// The number of segments composing the link style.
    length: u8,
}

impl Edge for FlowchartEdge {
    type Builder = FlowchartEdgeBuilder;
    type Node = FlowchartNode;

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

    fn left_arrow_shape(&self) -> Option<ArrowShape> {
        self.edge.left_arrow_shape()
    }

    fn right_arrow_shape(&self) -> Option<ArrowShape> {
        self.edge.right_arrow_shape()
    }
}

impl Display for FlowchartEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let segment = match self.line_style() {
            LineStyle::Solid => "-".repeat(2 + self.length as usize),
            LineStyle::Thick => "=".repeat(2 + self.length as usize),
            LineStyle::Dashed => format!("-{}-", ".".repeat(self.length as usize)),
        };

        writeln!(
            f,
            "{NODE_LETTER}{} {EDGE_LETTER}{}@{left_arrow}{segment}{right_arrow}{} {NODE_LETTER}{}",
            self.source().id(),
            self.id,
            self.label().map_or_else(String::new, |label| format!("|\"`{label}`\"|")),
            self.destination().id(),
            left_arrow = self.left_arrow_shape().as_ref().map_or_else(|| "", |shape| shape.left()),
            right_arrow =
                self.right_arrow_shape().as_ref().map_or_else(|| "", |shape| shape.right()),
        )?;

        writeln!(f, "{EDGE_LETTER}{}@{{curve:{}}}", self.id, self.curve_style)?;

        for class in &self.style_classes {
            writeln!(f, "class {EDGE_LETTER}{} {class};", self.id)?;
        }

        if !self.style_properties.is_empty() {
            write!(f, "linkStyle {EDGE_LETTER}{} ", self.id)?;
            for (style_number, style) in self.style_properties.iter().enumerate() {
                if style_number > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{style} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
