//! Submodule providing structs to characterize an ER (Entity-Relationship)
//! Diagram in Mermaid syntax.

mod entity_relationship_edge;
mod entity_relationship_node;
use entity_relationship_edge::EREdge;
pub use entity_relationship_edge::EREdgeBuilder;
use entity_relationship_node::ERNode;
pub use entity_relationship_node::ERNodeAttribute;

use crate::shared::{generic_configuration::GenericConfiguration, generic_diagram::GenericDiagram};

/// Represents an entity-relationship diagram in Mermaid syntax.
pub type EntityRelationshipDiagram = GenericDiagram<ERNode, EREdge, GenericConfiguration>;
