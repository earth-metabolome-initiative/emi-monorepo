//! Submodule defining a node struct for the class diagram in
//! Mermaid syntax.

mod class_attribute;
mod class_method;
pub use class_attribute::ClassAttribute;
pub use class_method::ClassMethod;

use crate::nodes::Node;

pub struct ClassNode {
    /// Node properties such as ID, label, and styles.
    node: Node,
    /// Attributes of the class node.
    attributes: Vec<ClassAttribute>,
    /// Methods of the class node.
    methods: Vec<ClassMethod>,
}
