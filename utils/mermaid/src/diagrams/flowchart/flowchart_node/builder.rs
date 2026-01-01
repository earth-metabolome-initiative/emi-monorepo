//! Submodule defining a builder struct to construct flowchart nodes in the
//! flowchart Mermaid diagrams.

use std::rc::Rc;

use crate::{
    diagrams::flowchart::flowchart_node::{ClickEvent, FlowchartNode, shape::FlowchartNodeShape},
    errors::NodeError,
    shared::{
        StyleClass, StyleClassError, generic_configuration::Direction,
        generic_node::GenericNodeBuilder,
    },
    traits::{Node, NodeBuilder},
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating a `FlowchartNode`.
pub struct FlowchartNodeBuilder {
    /// Shared attributes builder for the node.
    builder: GenericNodeBuilder,
    /// The click event associated with the node, if any.
    click_event: Option<ClickEvent>,
    /// The shape of the flowchart node.
    shape: FlowchartNodeShape,
    /// Possible subnodes of the flowchart node.
    subnodes: Vec<Rc<FlowchartNode>>,
    /// The direction of the subgraph, if applicable.
    direction: Option<Direction>,
}

impl FlowchartNodeBuilder {
    #[must_use]
    /// Sets the click event for the flowchart node.
    pub fn click_event(mut self, click_event: ClickEvent) -> Self {
        self.click_event = Some(click_event);
        self
    }

    #[must_use]
    /// Sets the shape of the flowchart node.
    pub fn shape(mut self, shape: FlowchartNodeShape) -> Self {
        self.shape = shape;
        self
    }

    /// Adds a subnode to the flowchart node.
    ///
    /// # Arguments
    ///
    /// * `subnode`: The subnode to be added, wrapped in a `Rc` for shared
    ///   ownership.
    ///
    /// # Errors
    ///
    /// * If the subnode is already present in the list, an error is returned.
    pub fn subnode(mut self, subnode: Rc<FlowchartNode>) -> Result<Self, NodeError> {
        if self.subnodes.contains(&subnode) {
            return Err(NodeError::DuplicateNode(subnode.label().to_owned()));
        }

        self.subnodes.push(subnode);
        Ok(self)
    }

    #[must_use]
    /// Returns whether the current node builder is a subgraph node.
    pub fn is_subgraph(&self) -> bool {
        !self.subnodes.is_empty()
    }

    #[must_use]
    /// Sets the direction of the subgraph for the flowchart node.
    ///
    /// # Arguments
    ///
    /// * `direction`: The direction of the subgraph.
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }

    #[must_use]
    /// Returns the direction of the subgraph, if set.
    pub fn get_direction(&self) -> Option<Direction> {
        self.direction
    }

    #[must_use]
    /// Resets the direction of the subgraph for the flowchart node.
    pub fn reset_direction(mut self) -> Self {
        self.direction = None;
        self
    }
}

impl TryFrom<FlowchartNodeBuilder> for FlowchartNode {
    type Error = NodeError;

    fn try_from(mut builder: FlowchartNodeBuilder) -> Result<Self, Self::Error> {
        if builder.direction.is_some() && builder.subnodes.is_empty() {
            return Err(NodeError::MissingSubnodes);
        }

        builder.subnodes.sort_unstable();

        Ok(FlowchartNode {
            node: builder.builder.try_into()?,
            click_event: builder.click_event,
            shape: builder.shape,
            subnodes: builder.subnodes,
            direction: builder.direction,
        })
    }
}

impl NodeBuilder for FlowchartNodeBuilder {
    type Node = FlowchartNode;
    type Error = NodeError;

    fn build(self) -> Result<Self::Node, Self::Error> {
        self.try_into()
    }

    fn id(mut self, id: u64) -> Self {
        self.builder = self.builder.id(id);
        self
    }

    fn get_id(&self) -> Option<u64> {
        self.builder.get_id()
    }

    fn label<S: ToString>(mut self, label: S) -> Result<Self, Self::Error> {
        self.builder = self.builder.label(label)?;
        Ok(self)
    }

    fn get_label(&self) -> Option<&String> {
        self.builder.get_label()
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

    fn style_properties(&self) -> impl Iterator<Item = &crate::prelude::StyleProperty> {
        self.builder.style_properties()
    }
}
