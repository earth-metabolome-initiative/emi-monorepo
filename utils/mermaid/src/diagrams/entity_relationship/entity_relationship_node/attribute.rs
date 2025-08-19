//! Submodule defining an attribute of an Entity-Relationship (ER) node
//! for the entity-relationship diagram in Mermaid syntax.

use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntityRelationshipAttribute {
    /// The name of the class attribute.
    name: String,
    /// The type of the class attribute.
    attribute_type: String,
}

impl Display for EntityRelationshipAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.attribute_type)
    }
}
