//! Submodule defining a builder struct for the class node in class diagrams.

use std::{fmt::Display, rc::Rc};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::Builder,
};

use crate::{
    diagrams::class_diagram::class_node::{ClassAttribute, ClassMethod, ClassNode},
    errors::NodeError,
    shared::{
        ClickEvent, StyleClass, StyleClassError,
        generic_node::{GenericNodeAttribute, GenericNodeBuilder},
    },
    traits::NodeBuilder,
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassNodeBuilder {
    /// Underlying generic node builder.
    builder: GenericNodeBuilder,
    /// The click event associated with the node, if any.
    click_event: Option<ClickEvent>,
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
    /// Click event associated with the class node.
    ClickEvent,
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
            ClassNodeAttribute::ClickEvent => write!(f, "click_event"),
            ClassNodeAttribute::Attributes => write!(f, "attributes"),
            ClassNodeAttribute::Methods => write!(f, "methods"),
            ClassNodeAttribute::Annotation => write!(f, "annotation"),
        }
    }
}

impl ClassNodeBuilder {
    /// Sets the click event for the class node.
    pub fn click_event(&mut self, click_event: ClickEvent) -> &mut Self {
        self.click_event = Some(click_event);
        self
    }

    /// Sets the annotation for the class node.
    pub fn annotation<S: ToString>(&mut self, annotation: &S) -> &mut Self {
        self.annotation = Some(annotation.to_string());
        self
    }

    /// Adds an attribute to the class node.
    pub fn attribute(&mut self, attribute: ClassAttribute) -> &mut Self {
        self.attributes.push(attribute);
        self
    }

    /// Adds a method to the class node.
    pub fn method(&mut self, method: ClassMethod) -> &mut Self {
        self.methods.push(method);
        self
    }
}

impl IsCompleteBuilder for ClassNodeBuilder {
    fn is_complete(&self) -> bool {
        self.builder.is_complete()
    }
}

impl Attributed for ClassNodeBuilder {
    type Attribute = ClassNodeAttribute;
}

impl Builder for ClassNodeBuilder {
    type Object = ClassNode;
    type Error = NodeError<Self::Attribute>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(ClassNode {
            node: self.builder.build()?,
            click_event: self.click_event,
            annotation: self.annotation,
            attributes: self.attributes,
            methods: self.methods,
        })
    }
}

impl NodeBuilder for ClassNodeBuilder {
    type Node = ClassNode;

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
