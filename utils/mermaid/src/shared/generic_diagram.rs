//! Submodule defining a generic diagram struct which can be used as a base
//! for various types of diagrams in Mermaid syntax.

use std::{fmt::Display, rc::Rc};

use crate::{
    shared::{StyleClass, StyleClassError},
    traits::{
        Configuration, ConfigurationBuilder, Diagram, DiagramBuilder, Edge, EdgeBuilder, Node,
        NodeBuilder,
    },
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A generic diagram struct that can be extended for specific diagram types.
pub struct GenericDiagram<Node, Edge, Config> {
    /// Style classes associated with this diagram.
    style_classes: Vec<Rc<StyleClass>>,
    /// Nodes in the diagram.
    nodes: Vec<Rc<Node>>,
    /// Edges in the diagram.
    edges: Vec<Rc<Edge>>,
    /// Configuration options for the diagram.
    configuration: Config,
}

impl<N: Node + Display, E: Edge<Node = N> + Display, C: Configuration> Diagram
    for GenericDiagram<N, E, C>
where
    crate::errors::Error: From<<N::Builder as NodeBuilder>::Error>
        + From<<E::Builder as EdgeBuilder>::Error>
        + From<<C::Builder as ConfigurationBuilder>::Error>,
{
    type Builder = GenericDiagramBuilder<N, E, C>;
    type Node = N;
    type Edge = E;
    type Configuration = C;

    fn configuration(&self) -> &Self::Configuration {
        &self.configuration
    }

    fn nodes(&self) -> impl Iterator<Item = &Self::Node> {
        self.nodes.iter().map(AsRef::as_ref)
    }

    fn edges(&self) -> impl Iterator<Item = &Self::Edge> {
        self.edges.iter().map(AsRef::as_ref)
    }

    fn style_classes(&self) -> impl Iterator<Item = &StyleClass> {
        self.style_classes.iter().map(AsRef::as_ref)
    }

    fn get_node_by_id(&self, id: u64) -> Option<Rc<Self::Node>> {
        self.nodes.iter().find(|node| node.id() == id).cloned()
    }

    fn get_style_class_by_name(&self, name: &str) -> Option<Rc<StyleClass>> {
        self.style_classes.iter().find(|sc| sc.name() == name).cloned()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A builder for creating a generic diagram.
pub struct GenericDiagramBuilder<Node, Edge, Config> {
    /// Underlying generic diagram.
    generic_diagram: GenericDiagram<Node, Edge, Config>,
}

impl<Node, Edge, Config: Default> Default for GenericDiagramBuilder<Node, Edge, Config> {
    fn default() -> Self {
        Self {
            generic_diagram: GenericDiagram {
                style_classes: Vec::new(),
                nodes: Vec::new(),
                edges: Vec::new(),
                configuration: Config::default(),
            },
        }
    }
}

impl<N: Node + Display, E: Edge<Node = N> + Display, C: Configuration>
    From<GenericDiagramBuilder<N, E, C>> for GenericDiagram<N, E, C>
{
    fn from(builder: GenericDiagramBuilder<N, E, C>) -> Self {
        builder.generic_diagram
    }
}

impl<N: Node + Display, E: Edge<Node = N> + Display, C: Configuration> DiagramBuilder
    for GenericDiagramBuilder<N, E, C>
where
    crate::errors::Error: From<<N::Builder as NodeBuilder>::Error>
        + From<<E::Builder as EdgeBuilder>::Error>
        + From<<C::Builder as ConfigurationBuilder>::Error>,
    GenericDiagram<N, E, C>: Diagram<Node = N, Edge = E, Configuration = C, Builder = Self>,
{
    type Diagram = GenericDiagram<N, E, C>;
    type Node = N;
    type NodeBuilder = N::Builder;
    type Edge = E;
    type EdgeBuilder = E::Builder;
    type Configuration = C;
    type ConfigurationBuilder = C::Builder;
    type Error = crate::errors::Error;

    fn configuration(
        mut self,
        configuration: Self::ConfigurationBuilder,
    ) -> Result<Self, Self::Error> {
        self.generic_diagram.configuration =
            configuration.build().map_err(crate::errors::Error::from)?;
        Ok(self)
    }

    fn edge(&mut self, edge: Self::EdgeBuilder) -> Result<Rc<Self::Edge>, Self::Error> {
        let edge = edge.build().map_err(crate::errors::Error::from)?;

        if !self.generic_diagram.nodes.contains(edge.source()) {
            return Err(crate::errors::EdgeError::SourceNodeNotFound(
                edge.source().label().to_owned(),
            )
            .into());
        }

        if !self.generic_diagram.nodes.contains(edge.destination()) {
            return Err(crate::errors::EdgeError::DestinationNodeNotFound(
                edge.destination().label().to_owned(),
            )
            .into());
        }

        let rc = Rc::new(edge);
        self.generic_diagram.edges.push(rc.clone());
        Ok(rc)
    }

    fn node(&mut self, mut node: Self::NodeBuilder) -> Result<Rc<Self::Node>, Self::Error> {
        let number_of_nodes = self.generic_diagram.nodes.len();
        {
            use crate::traits::node_builder::NodeBuilder;
            if node.get_id().is_none() {
                node = node.id(number_of_nodes as u64);
            }
        }
        let node = node.build().map_err(crate::errors::Error::from)?;

        for class in node.classes() {
            if !self.generic_diagram.style_classes.iter().any(|sc| sc.name() == class.name()) {
                return Err(StyleClassError::UnknownClass(class.clone()).into());
            }
        }

        let rc = Rc::new(node);
        self.generic_diagram.nodes.push(rc.clone());
        Ok(rc)
    }

    fn nodes(&self) -> impl Iterator<Item = &Rc<Self::Node>> + '_ {
        self.generic_diagram.nodes.iter()
    }

    fn get_node_by_id(&self, id: u64) -> Option<Rc<Self::Node>> {
        self.generic_diagram.get_node_by_id(id)
    }

    fn get_style_class_by_name(&self, name: &str) -> Option<Rc<StyleClass>> {
        self.generic_diagram.get_style_class_by_name(name)
    }

    fn number_of_nodes(&self) -> usize {
        self.generic_diagram.nodes.len()
    }

    fn number_of_edges(&self) -> usize {
        self.generic_diagram.edges.len()
    }

    fn style_class(
        &mut self,
        style_class: super::StyleClassBuilder,
    ) -> Result<Rc<StyleClass>, Self::Error> {
        let style_class = style_class.build().map_err(crate::errors::Error::from)?;

        if self.generic_diagram.style_classes.iter().any(|sc| sc.name() == style_class.name()) {
            return Err(StyleClassError::DuplicateClass(style_class.name().to_owned()).into());
        }

        let rc = Rc::new(style_class);
        self.generic_diagram.style_classes.push(rc.clone());
        Ok(rc)
    }
}
