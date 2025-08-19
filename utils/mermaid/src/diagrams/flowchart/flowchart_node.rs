//! Submodule defining a node which may be used in a flowchart diagram
//! in Mermaid syntax.

mod builder;
mod click_event;
mod shape;
use std::{fmt::Display, rc::Rc};

pub use builder::{FlowchartNodeAttribute, FlowchartNodeBuilder};
pub use click_event::ClickEvent;
use shape::FlowchartNodeShape;

use crate::{
    shared::{GenericNode, NODE_LETTER, StyleClass, style_class::StyleProperty},
    traits::Node,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a node in a flowchart diagram, which can have various
/// properties and may include click events.
pub(crate) struct FlowchartNode {
    /// Underlying node structure.
    node: GenericNode,
    /// The click event associated with the node, if any.
    click_event: Option<ClickEvent>,
    /// The shape of the node, which can be customized.
    shape: FlowchartNodeShape,
    /// The sub-nodes, when the node is a subgraph.
    subnodes: Vec<Rc<FlowchartNode>>,
}

impl Node for FlowchartNode {
    type Builder = FlowchartNodeBuilder;

    fn label(&self) -> &str {
        self.node.label()
    }

    fn id(&self) -> u32 {
        self.node.id()
    }

    fn styles(&self) -> impl Iterator<Item = &StyleProperty> {
        self.node.styles()
    }

    fn classes(&self) -> impl Iterator<Item = &StyleClass> {
        self.node.classes()
    }
}

impl Display for FlowchartNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.subnodes.is_empty() {
            write!(
                f,
                "{NODE_LETTER}{}@{{shape:{}, label:\"`{}`\"}}",
                self.id(),
                self.shape,
                self.label()
            )?;

            for class in self.classes() {
                write!(f, "class {NODE_LETTER}{} {};", self.id(), class)?;
            }
        } else {
            write!(f, "subgraph {NODE_LETTER}{} [{}] {{\n", self.id(), self.label())?;
            for node in &self.subnodes {
                writeln!(f, "    {node}")?;
            }
            write!(f, "}}")?;
        }
        if self.has_styles() {
            write!(f, "style {NODE_LETTER}{} ", self.id())?;
            for (style_number, style) in self.styles().enumerate() {
                if style_number > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ", style)?;
            }
        }
        Ok(())
    }
}
