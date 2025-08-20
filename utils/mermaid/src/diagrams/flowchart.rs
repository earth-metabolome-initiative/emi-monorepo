//! Submodule providing structs for creating flowcharts in Mermaid syntax.

mod configuration;
mod curve_styles;
mod flowchart_edge;
mod flowchart_node;
use configuration::FlowchartConfiguration;
pub use configuration::FlowchartConfigurationAttribute;
pub use flowchart_edge::FlowchartEdgeAttribute;
pub use flowchart_node::FlowchartNodeAttribute;

use crate::{
    diagrams::flowchart::{flowchart_edge::FlowchartEdge, flowchart_node::FlowchartNode},
    shared::generic_diagram::GenericDiagram,
};

/// Represents a flowchart diagram in Mermaid syntax.
pub type Flowchart = GenericDiagram<FlowchartNode, FlowchartEdge, FlowchartConfiguration>;
