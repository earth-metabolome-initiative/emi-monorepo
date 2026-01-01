//! Submodule defining a class diagram in Mermaid syntax.

pub mod class_edge;
pub mod class_node;
mod configuration;
pub mod visibility;
use std::fmt::Display;

use class_edge::ClassEdge;
use class_node::ClassNode;
pub use configuration::ClassDiagramConfiguration;

use crate::{
    shared::generic_diagram::{GenericDiagram, GenericDiagramBuilder},
    traits::{configuration::Configuration, diagram::Diagram},
};

/// Represents a class diagram in Mermaid syntax.
pub type ClassDiagram = GenericDiagram<ClassNode, ClassEdge, ClassDiagramConfiguration>;
/// Represents a builder for a class diagram in Mermaid syntax.
pub type ClassDiagramBuilder =
    GenericDiagramBuilder<ClassNode, ClassEdge, ClassDiagramConfiguration>;

impl Display for ClassDiagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.configuration())?;
        writeln!(f, "classDiagram")?;
        writeln!(f, "  direction {}", self.configuration().direction())?;
        for style_class in self.style_classes() {
            write!(f, "  {style_class}")?;
        }
        for node in self.nodes() {
            write!(f, "  {node}")?;
        }
        for edge in self.edges() {
            write!(f, "  {edge}")?;
        }
        Ok(())
    }
}
