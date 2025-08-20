//! Submodule providing a generic node struct which may be reused across
//! different diagrams.

use std::{fmt::Display, rc::Rc};

use common_traits::prelude::{Builder, BuilderError};

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
    id: u32,
    /// Label for the node.
    label: String,
    /// Classes associated with the node, used for styling.
    classes: Vec<Rc<StyleClass>>,
    /// Style properties for the node.
    style: Vec<StyleProperty>,
}

impl Node for GenericNode {
    type Builder = GenericNodeBuilder;

    fn id(&self) -> u32 {
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
    id: Option<u32>,
    /// Label for the node.
    label: Option<String>,
    /// Classes associated with the node, used for styling.
    classes: Vec<Rc<StyleClass>>,
    /// Style properties for the node.
    style: Vec<StyleProperty>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enumeration of attributes that can be set on a `GenericNode`.
pub enum GenericNodeAttribute {
    /// Unique identifier for the node.
    Id,
    /// Label of the node.
    Label,
    /// Classes associated with the node.
    Classes,
    /// Style properties for the node.
    Style,
}

impl Display for GenericNodeAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericNodeAttribute::Id => write!(f, "id"),
            GenericNodeAttribute::Label => write!(f, "label"),
            GenericNodeAttribute::Classes => write!(f, "classes"),
            GenericNodeAttribute::Style => write!(f, "style"),
        }
    }
}

impl Builder for GenericNodeBuilder {
    type Attribute = GenericNodeAttribute;
    type Error = NodeError<Self::Attribute>;
    type Object = GenericNode;

    fn is_complete(&self) -> bool {
        self.id.is_some() && self.label.is_some()
    }

    fn build(self) -> Result<Self::Object, Self::Error> {
        let id = self.id.ok_or(BuilderError::IncompleteBuild(GenericNodeAttribute::Id))?;
        let label = self.label.ok_or(BuilderError::IncompleteBuild(GenericNodeAttribute::Label))?;

        Ok(GenericNode { id, label, classes: self.classes, style: self.style })
    }
}

impl NodeBuilder for GenericNodeBuilder {
    type Node = GenericNode;

    fn id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }

    fn label<S: ToString>(mut self, label: S) -> Result<Self, Self::Error> {
        let label = label.to_string();
        if label.is_empty() {
            return Err(crate::errors::NodeError::EmptyLabel);
        }

        self.label = Some(label);
        Ok(self)
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
}
