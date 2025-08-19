//! Submodule defining the struct to represent an entity-relationship node
//! for the entity-relationship diagram in Mermaid syntax.

use std::fmt::Display;

mod attribute;
mod builder;
use attribute::EntityRelationshipAttribute;
pub use builder::ERNodeAttribute;

use crate::{
    diagrams::entity_relationship::entity_relationship_node::builder::ERNodeBuilder,
    shared::{GenericNode, NODE_LETTER, StyleClass, StyleProperty},
    traits::Node,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ERNode {
    /// Underlying node structure.
    node: GenericNode,
    /// Attributes of the entity-relationship node.
    attributes: Vec<EntityRelationshipAttribute>,
}

impl Node for ERNode {
    type Builder = ERNodeBuilder;

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

impl Display for ERNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{NODE_LETTER}{}[\"`{}`\"] {{\n", self.id(), self.label())?;
        for attr in &self.attributes {
            writeln!(f, "    {attr}")?;
        }
        write!(f, "}}")?;

        for class in self.classes() {
            write!(f, "class {NODE_LETTER}{} {};", self.id(), class)?;
        }

        Ok(())
    }
}
