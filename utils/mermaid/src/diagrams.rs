//! Submodule defining properties and structures shared across different types
//! of Mermaid diagrams.

pub mod class_diagram;
pub mod entity_relationship;
pub mod flowchart;
pub use class_diagram::ClassDiagram;
pub use entity_relationship::EntityRelationshipDiagram;
pub use flowchart::Flowchart;
