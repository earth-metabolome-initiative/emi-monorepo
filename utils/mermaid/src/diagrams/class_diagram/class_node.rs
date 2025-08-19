//! Submodule defining a node struct for the class diagram in
//! Mermaid syntax.

mod builder;
mod class_attribute;
mod class_method;
use std::fmt::Display;

pub use builder::ClassNodeAttribute;
pub use class_attribute::ClassAttribute;
pub use class_method::ClassMethod;

use crate::{
    diagrams::class_diagram::class_node::builder::ClassNodeBuilder,
    shared::{GenericNode, StyleClass, StyleProperty},
    traits::Node,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Struct representing a class node in a class diagram.
pub struct ClassNode {
    /// Underlying generic node.
    node: GenericNode,
    /// The annotation of the class node, which usually
    /// contains functional information such as `trait`, `interface`, etc.
    annotation: Option<String>,
    /// Attributes of the class node.
    attributes: Vec<ClassAttribute>,
    /// Methods of the class node.
    methods: Vec<ClassMethod>,
}

impl Node for ClassNode {
    type Builder = ClassNodeBuilder;

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

impl Display for ClassNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class v{}[{}] {{\n", self.id(), self.label())?;
        if let Some(annotation) = &self.annotation {
            write!(f, "    <<{}>>", annotation)?;
        }

        for attr in &self.attributes {
            writeln!(f, "    {attr}")?;
        }
        for method in &self.methods {
            writeln!(f, "    {method}")?;
        }
        write!(f, "}}")?;

        for class in self.classes() {
            write!(f, "cssClass {} {};", self.id(), class)?;
        }

        Ok(())
    }
}
