//! Submodule providing a builder for flowchart diagrams in Mermaid syntax.

use crate::{
    prelude::{
        Flowchart, FlowchartConfiguration, FlowchartConfigurationBuilder, FlowchartEdge,
        FlowchartEdgeBuilder, FlowchartNode, FlowchartNodeBuilder,
    },
    shared::{StyleClass, StyleClassBuilder, generic_diagram::GenericDiagramBuilder},
    traits::DiagramBuilder,
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a builder for a flowchart diagram in Mermaid syntax.
pub struct FlowchartBuilder {
    /// The configuration of the flowchart.
    generic: GenericDiagramBuilder<FlowchartNode, FlowchartEdge, FlowchartConfiguration>,
}

impl From<FlowchartBuilder> for Flowchart {
    fn from(builder: FlowchartBuilder) -> Self {
        let generic = builder.generic.into();
        Flowchart { generic }
    }
}

impl DiagramBuilder for FlowchartBuilder {
    type Configuration = FlowchartConfiguration;
    type ConfigurationBuilder = FlowchartConfigurationBuilder;
    type Diagram = Flowchart;
    type Edge = FlowchartEdge;
    type EdgeBuilder = FlowchartEdgeBuilder;
    type Node = FlowchartNode;
    type NodeBuilder = FlowchartNodeBuilder;
    type Error = crate::errors::Error;

    fn configuration(
        mut self,
        configuration: Self::ConfigurationBuilder,
    ) -> Result<Self, Self::Error> {
        self.generic = self.generic.configuration(configuration)?;
        Ok(self)
    }

    fn edge(
        &mut self,
        mut edge: Self::EdgeBuilder,
    ) -> Result<std::rc::Rc<Self::Edge>, Self::Error> {
        edge = edge.id(self.number_of_edges());
        self.generic.edge(edge)
    }

    fn get_node_by_id(&self, id: u64) -> Option<std::rc::Rc<Self::Node>> {
        self.generic.get_node_by_id(id)
    }

    fn node(&mut self, node: Self::NodeBuilder) -> Result<std::rc::Rc<Self::Node>, Self::Error> {
        self.generic.node(node)
    }

    fn nodes(&self) -> impl Iterator<Item = &std::rc::Rc<Self::Node>> + '_ {
        self.generic.nodes()
    }

    fn number_of_edges(&self) -> usize {
        self.generic.number_of_edges()
    }

    fn number_of_nodes(&self) -> usize {
        self.generic.number_of_nodes()
    }

    fn style_class(
        &mut self,
        style_class: StyleClassBuilder,
    ) -> Result<std::rc::Rc<StyleClass>, Self::Error> {
        self.generic.style_class(style_class)
    }

    fn get_style_class_by_name(&self, name: &str) -> Option<std::rc::Rc<StyleClass>> {
        self.generic.get_style_class_by_name(name)
    }
}
