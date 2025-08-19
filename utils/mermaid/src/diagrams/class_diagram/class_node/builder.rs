//! Submodule defining a builder struct for the class node in class diagrams.

use std::{fmt::Display, rc::Rc};

use common_traits::prelude::Builder;

use crate::{
    diagrams::class_diagram::class_node::{ClassAttribute, ClassMethod, ClassNode},
    errors::NodeError,
    shared::{
        StyleClass, StyleClassError,
        generic_node::{GenericNodeAttribute, GenericNodeBuilder},
    },
    traits::NodeBuilder,
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassNodeBuilder {
    /// Underlying generic node builder.
    builder: GenericNodeBuilder,
    /// The annotation of the class node, which usually
    /// contains functional information such as `trait`, `interface`, etc.
    annotation: Option<String>,
    /// Attributes of the class node.
    attributes: Vec<ClassAttribute>,
    /// Methods of the class node.
    methods: Vec<ClassMethod>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enumeration of possible attributes for flowchart nodes.
pub enum ClassNodeAttribute {
    /// Attribute from the underlying generic node.
    Generic(GenericNodeAttribute),
    /// Annotation for the class node, such as `trait`, `interface`, etc.
    Annotation,
    /// Class attributes for the entity-relationship node.
    Attributes,
    /// Class methods for the entity-relationship node.
    Methods,
}

impl From<GenericNodeAttribute> for ClassNodeAttribute {
    fn from(attr: GenericNodeAttribute) -> Self {
        ClassNodeAttribute::Generic(attr)
    }
}

impl Display for ClassNodeAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassNodeAttribute::Generic(attr) => write!(f, "{attr}"),
            ClassNodeAttribute::Attributes => write!(f, "attributes"),
            ClassNodeAttribute::Methods => write!(f, "methods"),
            ClassNodeAttribute::Annotation => write!(f, "annotation"),
        }
    }
}

impl Builder for ClassNodeBuilder {
    type Attribute = ClassNodeAttribute;
    type Object = ClassNode;
    type Error = NodeError<Self::Attribute>;

    fn is_complete(&self) -> bool {
        self.builder.is_complete()
    }

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(ClassNode {
            node: self.builder.build()?,
            annotation: self.annotation,
            attributes: self.attributes,
            methods: self.methods,
        })
    }
}

impl NodeBuilder for ClassNodeBuilder {
    type Node = ClassNode;

    fn id(&mut self, id: u32) -> &mut Self {
        self.builder.id(id);
        self
    }

    fn label<S: ToString>(&mut self, label: S) -> Result<&mut Self, Self::Error> {
        self.builder.label(label)?;
        Ok(self)
    }

    fn style_class(&mut self, style_class: Rc<StyleClass>) -> Result<&mut Self, StyleClassError> {
        self.builder.style_class(style_class)?;
        Ok(self)
    }

    fn style_property(
        &mut self,
        property: crate::shared::StyleProperty,
    ) -> Result<&mut Self, StyleClassError> {
        self.builder.style_property(property)?;
        Ok(self)
    }
}
