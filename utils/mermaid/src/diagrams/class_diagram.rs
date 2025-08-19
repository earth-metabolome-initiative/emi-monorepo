//! Submodule defining a class diagram in Mermaid syntax.

pub mod class_edge;
pub mod class_node;
pub mod configuration;
pub mod visibility;
use class_edge::ClassEdge;
use class_node::ClassNode;
pub use class_node::ClassNodeAttribute;
use configuration::ClassDiagramConfiguration;

use crate::shared::generic_diagram::GenericDiagram;

/// Represents a class diagram in Mermaid syntax.
pub struct ClassDiagram {
    /// Configuration options for the class diagram.
    configuration: ClassDiagramConfiguration,
    /// Underlying generic diagram structure.
    diagram: GenericDiagram<ClassNode, ClassEdge>,
}
