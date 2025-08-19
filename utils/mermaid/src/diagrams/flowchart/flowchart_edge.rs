//! Submodule defining an edge which may be used in a flowchart diagram
//! in Mermaid syntax.

use std::{fmt::Display, rc::Rc};

use crate::{
    diagrams::flowchart::{curve_styles::CurveStyle, flowchart_node::FlowchartNode},
    shared::{ArrowShape, EDGE_LETTER, LineStyle, NODE_LETTER, StyleClass, StyleProperty},
    traits::{Edge, node::Node},
};

pub mod builder;
pub use builder::FlowchartEdgeBuilder;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct FlowchartEdge {
    /// Unique identifier for the edge.
    id: u32,
    /// The source node ID of the edge.
    source: Rc<FlowchartNode>,
    /// The destination node ID of the edge.
    destination: Rc<FlowchartNode>,
    /// The label of the edge, which is displayed in the diagram.
    label: String,
    /// Classes associated with the edge, used for styling.
    classes: Vec<Rc<StyleClass>>,
    /// Styling properties for the edge, such as color and font.
    styles: Vec<StyleProperty>,
    /// The curve style of the edge.
    curve_style: CurveStyle,
    /// The number of segments composing the link style.
    length: u8,
    /// The line style of the link.
    line_style: LineStyle,
    /// The left arrow shape of the link, if any.
    left_arrow_shape: Option<ArrowShape>,
    /// The right arrow shape of the link, if any.
    right_arrow_shape: Option<ArrowShape>,
}

impl Edge for FlowchartEdge {
    type Builder = FlowchartEdgeBuilder;
    type Node = FlowchartNode;

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

impl Display for FlowchartEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let segment = match self.line_style {
            LineStyle::Solid => "-".repeat(2 + self.length as usize),
            LineStyle::Thick => "=".repeat(2 + self.length as usize),
            LineStyle::Dashed => format!("-{}-", ".".repeat(self.length as usize)),
        };

        write!(
            f,
            "{NODE_LETTER}{} {EDGE_LETTER}{}{left_arrow}{segment}{right_arrow}|\"`{}`\"| {NODE_LETTER}{}",
            self.source.id(),
            self.id,
            self.label,
            self.destination.id(),
            left_arrow = self.left_arrow_shape.as_ref().map_or_else(|| "", |shape| shape.left()),
            right_arrow = self.right_arrow_shape.as_ref().map_or_else(|| "", |shape| shape.right()),
        )?;

        write!(f, "{EDGE_LETTER}{}@{{curve:{}}}", self.id, self.curve_style)?;

        for class in &self.classes {
            write!(f, "class {} {};", self.id, class)?;
        }

        if !self.styles.is_empty() {
            write!(f, "linkStyle {} ", self.id)?;
            for (style_number, style) in self.styles.iter().enumerate() {
                if style_number > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ", style)?;
            }
        }
        Ok(())
    }
}
