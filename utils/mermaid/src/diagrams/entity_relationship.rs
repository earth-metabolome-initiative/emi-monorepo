//! Submodule providing structs to characterize an ER (Entity-Relationship)
//! Diagram in Mermaid syntax.

mod entity_relationship_edge;
mod entity_relationship_node;
use std::fmt::Display;

use entity_relationship_edge::EREdge;
pub use entity_relationship_edge::EREdgeBuilder;
use entity_relationship_node::ERNode;
pub use entity_relationship_node::ERNodeBuilder;

use crate::{
    shared::{
        generic_configuration::{GenericConfiguration, GenericConfigurationBuilder},
        generic_diagram::{GenericDiagram, GenericDiagramBuilder},
    },
    traits::{configuration::Configuration, diagram::Diagram},
};

/// Represents the configuration for an entity-relationship diagram.
pub type ERDiagramConfiguration = GenericConfiguration;
/// Represents the configuration builder for an entity-relationship diagram.
pub type ERDiagramConfigurationBuilder = GenericConfigurationBuilder;
/// Represents an entity-relationship diagram in Mermaid syntax.
pub type ERDiagram = GenericDiagram<ERNode, EREdge, GenericConfiguration>;
/// Represents a builder for an entity-relationship diagram in Mermaid syntax.
pub type ERDiagramBuilder = GenericDiagramBuilder<ERNode, EREdge, GenericConfiguration>;

impl Display for ERDiagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.configuration())?;
        writeln!(f, "erDiagram")?;
        writeln!(f, "direction {}", self.configuration().direction())?;
        for style_class in self.style_classes() {
            write!(f, "{style_class}")?;
        }
        for node in self.nodes() {
            write!(f, "{node}")?;
        }
        for edge in self.edges() {
            write!(f, "{edge}")?;
        }
        Ok(())
    }
}
