//! Submodule providing a generic node struct which may be reused across
//! different diagrams.

use std::rc::Rc;

use crate::{
    errors::NodeError,
    shared::{StyleClass, StyleClassError, StyleProperty},
    traits::{Node, NodeBuilder},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Struct representing a generic node in Mermaid diagrams.
pub(crate) struct GenericNode {
    /// Unique identifier for the node.
    id: u64,
    /// Label for the node.
    label: String,
    /// Classes associated with the node, used for styling.
    classes: Vec<Rc<StyleClass>>,
    /// Style properties for the node.
    style: Vec<StyleProperty>,
}

impl Node for GenericNode {
    type Builder = GenericNodeBuilder;

    fn id(&self) -> u64 {
        self.id
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn classes(&self) -> impl Iterator<Item = &StyleClass> {
        self.classes.iter().map(AsRef::as_ref)
    }

    fn styles(&self) -> impl Iterator<Item = &StyleProperty> {
        self.style.iter()
    }

    fn is_compatible_arrow_shape(_shape: super::ArrowShape) -> bool {
        true
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating a `GenericNode`.
pub(crate) struct GenericNodeBuilder {
    /// Unique identifier for the node.
    id: Option<u64>,
    /// Label for the node.
    label: Option<String>,
    /// Classes associated with the node, used for styling.
    classes: Vec<Rc<StyleClass>>,
    /// Style properties for the node.
    style: Vec<StyleProperty>,
}

impl TryFrom<GenericNodeBuilder> for GenericNode {
    type Error = NodeError;

    fn try_from(builder: GenericNodeBuilder) -> Result<Self, Self::Error> {
        let id = builder.id.ok_or(NodeError::MissingId)?;
        let label = builder.label.ok_or(NodeError::MissingLabel)?;

        Ok(GenericNode { id, label, classes: builder.classes, style: builder.style })
    }
}

impl NodeBuilder for GenericNodeBuilder {
    type Node = GenericNode;
    type Error = NodeError;

    fn build(self) -> Result<Self::Node, Self::Error> {
        self.try_into()
    }

    fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    fn get_id(&self) -> Option<u64> {
        self.id
    }

    fn label<S: ToString>(mut self, label: S) -> Result<Self, Self::Error> {
        let label = label.to_string();
        if label.is_empty() {
            return Err(crate::errors::NodeError::EmptyLabel);
        }

        self.label = Some(label);
        Ok(self)
    }

    fn get_label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    fn style_class(mut self, style_class: Rc<StyleClass>) -> Result<Self, StyleClassError> {
        if self.classes.iter().any(|c| c.name() == style_class.name()) {
            return Err(StyleClassError::DuplicateClass(style_class.name().to_owned()));
        }

        self.classes.push(style_class);
        Ok(self)
    }

    fn style_property(mut self, property: StyleProperty) -> Result<Self, StyleClassError> {
        if self.style.iter().any(|p| p.is_same_type(property)) {
            return Err(StyleClassError::DuplicateProperty(property));
        }

        self.style.push(property);
        Ok(self)
    }

    fn style_properties(&self) -> impl Iterator<Item = &StyleProperty> {
        self.style.iter()
    }
}
