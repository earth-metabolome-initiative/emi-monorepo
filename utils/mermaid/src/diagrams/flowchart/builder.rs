//! Submodule providing a builder for creating flowcharts in Mermaid syntax.

use crate::{
    diagrams::flowchart::{flowchart_edge::FlowchartEdge, flowchart_node::FlowchartNode},
    traits::DiagramBuilder,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlowchartBuilder {}

impl DiagramBuilder for FlowchartBuilder {
    type Node = FlowchartNode;
    type Edge = FlowchartEdge;
}
