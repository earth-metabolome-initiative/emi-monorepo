//! Submodule defining the struct to represent an entity-relationship node
//! for the entity-relationship diagram in Mermaid syntax.

use std::fmt::Display;

mod attribute;
mod builder;
use attribute::EntityRelationshipAttribute;

pub struct EntityRelationshipNode {
    /// Label for the entity-relationship node.
    label: String,
    /// Attributes of the entity-relationship node.
    attributes: Vec<EntityRelationshipAttribute>,
}

impl Display for EntityRelationshipNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{\n", self.label)?;
        for attr in &self.attributes {
            writeln!(f, "    {attr}")?;
        }
        write!(f, "}}")
    }
}
