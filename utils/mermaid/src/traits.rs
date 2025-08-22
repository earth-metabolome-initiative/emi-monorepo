//! Submodule defining traits for Mermaid diagrams and annexed objects.

pub mod diagram;
pub mod diagram_builder;
pub use diagram::Diagram;
pub use diagram_builder::DiagramBuilder;
pub mod node;
pub use node::Node;
pub mod node_builder;
pub use node_builder::NodeBuilder;
pub mod edge;
pub use edge::Edge;
pub mod edge_builder;
pub use edge_builder::EdgeBuilder;
pub mod configuration;
pub use configuration::Configuration;
pub mod configuration_builder;
pub use configuration_builder::ConfigurationBuilder;
