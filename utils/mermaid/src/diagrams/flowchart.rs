//! Submodule providing structs for creating flowcharts in Mermaid syntax.

mod configuration;
mod curve_styles;
mod flowchart_edge;
mod flowchart_node;
use std::fmt::Display;

pub use configuration::FlowchartConfiguration;
pub use configuration::FlowchartConfigurationAttribute;
pub use flowchart_edge::FlowchartEdgeAttribute;
pub use flowchart_node::FlowchartNodeAttribute;

use crate::{
    diagrams::flowchart::{flowchart_edge::FlowchartEdge, flowchart_node::FlowchartNode},
    shared::generic_diagram::{GenericDiagram, GenericDiagramBuilder},
    traits::{Configuration, Diagram},
};

/// Represents a flowchart diagram in Mermaid syntax.
pub type Flowchart = GenericDiagram<FlowchartNode, FlowchartEdge, FlowchartConfiguration>;
/// Represents a builder for a flowchart diagram in Mermaid syntax
pub type FlowchartBuilder =
    GenericDiagramBuilder<FlowchartNode, FlowchartEdge, FlowchartConfiguration>;

impl Display for Flowchart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.configuration())?;
        writeln!(f, "flowchart {}", self.configuration().direction())?;
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
