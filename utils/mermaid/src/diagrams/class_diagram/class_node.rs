//! Submodule defining a node struct for the class diagram in
//! Mermaid syntax.

mod builder;
mod class_attribute;
mod class_method;
use std::fmt::Display;

pub use class_attribute::ClassAttribute;
pub use class_method::ClassMethod;

use crate::nodes::Node;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Struct representing a class node in a class diagram.
pub struct ClassNode {
    /// Node properties such as ID, label, and styles.
    node: Node,
    /// Attributes of the class node.
    attributes: Vec<ClassAttribute>,
    /// Methods of the class node.
    methods: Vec<ClassMethod>,
}

impl Display for ClassNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class {} {{\n", self.node.label)?;
        for attr in &self.attributes {
            writeln!(f, "    {attr}")?;
        }
        for method in &self.methods {
            writeln!(f, "    {method}")?;
        }
        write!(f, "}}")
    }
}
