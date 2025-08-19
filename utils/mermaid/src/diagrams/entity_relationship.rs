//! Submodule providing structs to characterize an ER (Entity-Relationship)
//! Diagram in Mermaid syntax.

mod builder;
mod configuration;
mod entity_relationship_edge;
mod entity_relationship_node;
use configuration::EntityRelationshipConfiguration;
use entity_relationship_edge::EREdge;
use entity_relationship_node::ERNode;
pub use entity_relationship_node::ERNodeAttribute;

use crate::shared::generic_diagram::GenericDiagram;

/// Represents an entity-relationship diagram in Mermaid syntax.
pub struct EntityRelationshipDiagram {
    /// Configuration options for the entity-relationship diagram.
    configuration: EntityRelationshipConfiguration,
    /// Underlying generic diagram structure.
    diagram: GenericDiagram<ERNode, EREdge>,
}
