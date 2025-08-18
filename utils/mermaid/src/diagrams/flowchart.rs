//! Submodule providing structs for creating flowcharts in Mermaid syntax.

mod builder;
mod configuration;
mod flowchart_direction;
mod flowchart_node;
use configuration::FlowchartConfiguration;

use crate::shared::generic_diagram::GenericDiagram;

/// Represents a flowchart diagram in Mermaid syntax.
pub struct Flowchart {
    /// Configuration options for the flowchart.
    configuration: FlowchartConfiguration,
    /// Underlying generic diagram structure.
    diagram: GenericDiagram,
}
