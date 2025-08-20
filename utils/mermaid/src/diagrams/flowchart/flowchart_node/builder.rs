//! Submodule defining a builder struct to construct flowchart nodes in the
//! flowchart Mermaid diagrams.

use std::{fmt::Display, rc::Rc};

use common_traits::prelude::{Builder, BuilderError};

use crate::{
    diagrams::flowchart::flowchart_node::{ClickEvent, FlowchartNode, shape::FlowchartNodeShape},
    errors::NodeError,
    shared::{
        StyleClass, StyleClassError,
        generic_configuration::Direction,
        generic_node::{GenericNodeAttribute, GenericNodeBuilder},
    },
    traits::NodeBuilder,
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating flowchart nodes with various properties.
pub struct FlowchartNodeBuilder {
    /// Shared attributes builder for the node.
    builder: GenericNodeBuilder,
    /// The click event associated with the node, if any.
    click_event: Option<ClickEvent>,
    /// The shape of the flowchart node.
    shape: Option<FlowchartNodeShape>,
    /// Possible subnodes of the flowchart node.
    subnodes: Vec<Rc<FlowchartNode>>,
    /// The direction of the subgraph, if applicable.
    direction: Option<Direction>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enumeration of possible attributes for flowchart nodes.
pub enum FlowchartNodeAttribute {
    /// Attribute from the underlying generic node.
    Generic(GenericNodeAttribute),
    /// Click event attribute.
    ClickEvent,
    /// Shape attribute for the flowchart node.
    Shape,
    /// Subnodes attribute, representing child nodes.
    Subnodes,
    /// Direction attribute, representing the flow direction of the subgraph.
    Direction,
}

impl From<GenericNodeAttribute> for FlowchartNodeAttribute {
    fn from(attr: GenericNodeAttribute) -> Self {
        FlowchartNodeAttribute::Generic(attr)
    }
}

impl Display for FlowchartNodeAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlowchartNodeAttribute::Generic(attr) => write!(f, "{attr}"),
            FlowchartNodeAttribute::ClickEvent => write!(f, "clickEvent"),
            FlowchartNodeAttribute::Shape => write!(f, "shape"),
            FlowchartNodeAttribute::Subnodes => write!(f, "subnodes"),
            FlowchartNodeAttribute::Direction => write!(f, "direction"),
        }
    }
}

impl Builder for FlowchartNodeBuilder {
    type Attribute = FlowchartNodeAttribute;
    type Object = FlowchartNode;
    type Error = NodeError<Self::Attribute>;

    fn is_complete(&self) -> bool {
        self.builder.is_complete() && self.shape.is_some()
    }

    fn build(self) -> Result<Self::Object, Self::Error> {
        if self.direction.is_some() && self.subnodes.is_empty() {
            return Err(BuilderError::IncompleteBuild(FlowchartNodeAttribute::Subnodes).into());
        }

        Ok(FlowchartNode {
            node: self.builder.build()?,
            click_event: self.click_event,
            shape: self
                .shape
                .ok_or(BuilderError::IncompleteBuild(FlowchartNodeAttribute::Shape))?,
            subnodes: self.subnodes,
            direction: self.direction,
        })
    }
}

impl NodeBuilder for FlowchartNodeBuilder {
    type Node = FlowchartNode;

    fn id(mut self, id: u32) -> Self {
        self.builder = self.builder.id(id);
        self
    }

    fn label<S: ToString>(mut self, label: S) -> Result<Self, Self::Error> {
        self.builder = self.builder.label(label)?;
        Ok(self)
    }

    fn style_class(mut self, style_class: Rc<StyleClass>) -> Result<Self, StyleClassError> {
        self.builder = self.builder.style_class(style_class)?;
        Ok(self)
    }

    fn style_property(
        mut self,
        property: crate::shared::StyleProperty,
    ) -> Result<Self, StyleClassError> {
        self.builder = self.builder.style_property(property)?;
        Ok(self)
    }
}
