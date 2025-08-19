//! Submodule providing structs for creating flowcharts in Mermaid syntax.

mod builder;
pub use builder::FlowchartBuilder;
mod configuration;
mod curve_styles;
mod flowchart_direction;
mod flowchart_edge;
mod flowchart_node;
use configuration::FlowchartConfiguration;
pub use flowchart_node::FlowchartNodeAttribute;

use crate::{
    diagrams::flowchart::{flowchart_edge::FlowchartEdge, flowchart_node::FlowchartNode},
    shared::generic_diagram::GenericDiagram,
    traits::Diagram,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a flowchart diagram in Mermaid syntax.
pub struct Flowchart {
    /// Configuration options for the flowchart.
    configuration: FlowchartConfiguration,
    /// Underlying generic diagram structure.
    diagram: GenericDiagram<FlowchartNode, FlowchartEdge>,
}

impl Diagram for Flowchart {
    type Builder = FlowchartBuilder;
    type Node = FlowchartNode;
    type Edge = FlowchartEdge;
}
