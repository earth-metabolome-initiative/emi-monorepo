//! Submodule defining a class diagram in Mermaid syntax.

pub mod class_edge;
pub mod class_node;
mod configuration;
pub mod visibility;
use class_edge::ClassEdge;
pub use class_edge::ClassEdgeAttribute;
use class_node::ClassNode;
pub use class_node::ClassNodeAttribute;
use configuration::ClassDiagramConfiguration;
pub use configuration::ClassDiagramConfigurationAttribute;

use crate::shared::generic_diagram::GenericDiagram;

/// Represents a class diagram in Mermaid syntax.
pub type ClassDiagram = GenericDiagram<ClassNode, ClassEdge, ClassDiagramConfiguration>;
