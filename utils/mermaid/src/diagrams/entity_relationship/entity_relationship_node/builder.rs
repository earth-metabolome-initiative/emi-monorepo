//! Submodule defining a builder struct for the entity-relationship node in
//! entity-relationship diagrams.

use std::{fmt::Display, rc::Rc};

use common_traits::prelude::Builder;

use crate::{
    diagrams::entity_relationship::entity_relationship_node::{
        ERNode, attribute::EntityRelationshipAttribute,
    },
    errors::NodeError,
    shared::{
        StyleClass, StyleClassError,
        generic_node::{GenericNodeAttribute, GenericNodeBuilder},
    },
    traits::NodeBuilder,
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for the entity-relationship node in Mermaid syntax.
pub struct ERNodeBuilder {
    /// Shared attributes builder for the node.
    builder: GenericNodeBuilder,
    /// The attributes of the entity-relationship node.
    class_attributes: Vec<EntityRelationshipAttribute>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enumeration of possible attributes for flowchart nodes.
pub enum ERNodeAttribute {
    /// Attribute from the underlying generic node.
    Generic(GenericNodeAttribute),
    /// Class attribute for the entity-relationship node.
    Attributes,
}

impl From<GenericNodeAttribute> for ERNodeAttribute {
    fn from(attr: GenericNodeAttribute) -> Self {
        ERNodeAttribute::Generic(attr)
    }
}

impl Display for ERNodeAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ERNodeAttribute::Generic(attr) => write!(f, "{attr}"),
            ERNodeAttribute::Attributes => write!(f, "attributes"),
        }
    }
}

impl Builder for ERNodeBuilder {
    type Attribute = ERNodeAttribute;
    type Object = ERNode;
    type Error = NodeError<Self::Attribute>;

    fn is_complete(&self) -> bool {
        self.builder.is_complete()
    }

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(ERNode { node: self.builder.build()?, attributes: self.class_attributes })
    }
}

impl NodeBuilder for ERNodeBuilder {
    type Node = ERNode;

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
