//! Submodule providing structs for creating flowcharts in Mermaid syntax.

mod builder;
mod configuration;
mod curve_styles;
mod flowchart_edge;
mod flowchart_node;
use std::fmt::Display;

pub use builder::FlowchartBuilder;
pub use configuration::{FlowchartConfiguration, FlowchartConfigurationBuilder};
pub use curve_styles::CurveStyle;
pub use flowchart_edge::{FlowchartEdge, FlowchartEdgeBuilder};
pub use flowchart_node::{FlowchartNode, FlowchartNodeBuilder, FlowchartNodeShape};

use crate::{
    shared::generic_diagram::GenericDiagram,
    traits::{Configuration, Diagram, Node, edge::Edge},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a flowchart diagram in Mermaid syntax.
pub struct Flowchart {
    generic: GenericDiagram<FlowchartNode, FlowchartEdge, FlowchartConfiguration>,
}

impl Diagram for Flowchart {
    type Builder = FlowchartBuilder;
    type Configuration = FlowchartConfiguration;
    type Edge = FlowchartEdge;
    type Node = FlowchartNode;

    fn configuration(&self) -> &Self::Configuration {
        self.generic.configuration()
    }

    fn edges(&self) -> impl Iterator<Item = &Self::Edge> {
        self.generic.edges()
    }

    fn get_node_by_id(&self, id: u64) -> Option<std::rc::Rc<Self::Node>> {
        self.generic.get_node_by_id(id)
    }

    fn get_style_class_by_name(
        &self,
        name: &str,
    ) -> Option<std::rc::Rc<crate::shared::StyleClass>> {
        self.generic.get_style_class_by_name(name)
    }

    fn nodes(&self) -> impl Iterator<Item = &Self::Node> {
        self.generic.nodes()
    }

    fn style_classes(&self) -> impl Iterator<Item = &crate::shared::StyleClass> {
        self.generic.style_classes()
    }
}

impl Display for Flowchart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.configuration())?;
        writeln!(f, "flowchart {}", self.configuration().direction())?;
        for style_class in self.style_classes() {
            if !self.nodes().any(|n| n.classes().any(|sc| sc == style_class))
                && !self.edges().any(|e| e.classes().any(|sc| sc == style_class))
            {
                continue;
            }

            write!(f, "{style_class}")?;
        }

        let mut subgraph_nodes = Vec::new();
        for node in self.nodes() {
            if subgraph_nodes.contains(&node) {
                continue;
            }
            subgraph_nodes.extend(node.subnodes());
        }
        subgraph_nodes.sort_unstable();

        for node in self.nodes() {
            if subgraph_nodes.contains(&node) {
                continue;
            }
            write!(f, "{node}")?;
        }
        for edge in self.edges() {
            write!(f, "{edge}")?;
        }
        Ok(())
    }
}
