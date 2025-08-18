//! Submodule defining a class attribute struct for the class diagram in
//! Mermaid syntax, including its visibility and type.

use std::fmt::Display;

use crate::diagrams::class_diagram::visibility::Visibility;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Struct representing a class attribute in a class diagram.
pub struct ClassAttribute {
    /// The name of the class attribute.
    name: String,
    /// The type of the class attribute.
    attribute_type: String,
    /// The visibility of the class attribute (e.g., public, private).
    visibility: Visibility,
}

impl Display for ClassAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}: {}", self.visibility, self.name, self.attribute_type)
    }
}
