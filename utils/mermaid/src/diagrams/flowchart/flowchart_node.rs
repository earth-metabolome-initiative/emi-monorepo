//! Submodule defining a node which may be used in a flowchart diagram
//! in Mermaid syntax.

mod builder;
mod shape;
use std::{fmt::Display, rc::Rc};

pub use builder::FlowchartNodeBuilder;
pub use shape::FlowchartNodeShape;

use crate::{
    shared::{
        ClickEvent, GenericNode, NODE_LETTER, StyleClass, generic_configuration::Direction,
        style_class::StyleProperty,
    },
    traits::Node,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a node in a flowchart diagram, which can have various
/// properties and may include click events.
pub struct FlowchartNode {
    /// Underlying node structure.
    node: GenericNode,
    /// The click event associated with the node, if any.
    click_event: Option<ClickEvent>,
    /// The shape of the node, which can be customized.
    shape: FlowchartNodeShape,
    /// The sub-nodes, when the node is a subgraph.
    subnodes: Vec<Rc<FlowchartNode>>,
    /// The direction of the subgraph, if applicable.
    direction: Option<Direction>,
}

impl FlowchartNode {
    /// Returns an iterator over the subnodes of the flowchart node.
    pub fn subnodes(&self) -> impl Iterator<Item = &FlowchartNode> {
        self.subnodes.iter().map(AsRef::as_ref)
    }
}

impl Node for FlowchartNode {
    type Builder = FlowchartNodeBuilder;

    fn label(&self) -> &str {
        self.node.label()
    }

    fn id(&self) -> u64 {
        self.node.id()
    }

    fn styles(&self) -> impl Iterator<Item = &StyleProperty> {
        self.node.styles()
    }

    fn classes(&self) -> impl Iterator<Item = &StyleClass> {
        self.node.classes()
    }

    fn is_compatible_arrow_shape(shape: crate::shared::ArrowShape) -> bool {
        matches!(
            shape,
            crate::shared::ArrowShape::Normal
                | crate::shared::ArrowShape::Circle
                | crate::shared::ArrowShape::X
        )
    }
}

impl Display for FlowchartNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.subnodes.is_empty() {
            writeln!(
                f,
                "{NODE_LETTER}{}@{{shape: {}, label: \"{}\"}}",
                self.id(),
                self.shape,
                self.label()
            )?;

            if let Some(click_event) = &self.click_event {
                writeln!(f, " click {NODE_LETTER}{} {click_event}", self.id(),)?;
            }

            for class in self.classes() {
                writeln!(f, "class {NODE_LETTER}{} {}", self.id(), class.name())?;
            }
        } else {
            writeln!(f, "subgraph {NODE_LETTER}{} [\"`{}`\"]", self.id(), self.label())?;
            if let Some(direction) = &self.direction {
                writeln!(f, "    direction {direction}")?;
            }

            for node in &self.subnodes {
                write!(f, "    {node}")?;
            }
            writeln!(f, "end")?;
        }
        if self.has_styles() {
            write!(f, "style {NODE_LETTER}{} ", self.id())?;
            for (style_number, style) in self.styles().enumerate() {
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
