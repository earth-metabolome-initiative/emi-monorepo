//! Submodule defining the `DiagramBuilder` trait for Mermaid diagrams.

use std::{fmt::Display, rc::Rc};

use common_traits::prelude::Builder;

use crate::{
    shared::{StyleClass, StyleClassBuilder},
    traits::{Configuration, ConfigurationBuilder, Diagram, Edge, Node, NodeBuilder},
};

/// Trait defining the builder for Mermaid diagrams.
pub trait DiagramBuilder: Builder<Object = <Self as DiagramBuilder>::Diagram> {
    /// Type of the diagram that this builder constructs.
    type Diagram: Diagram<
            Builder = Self,
            Node = Self::Node,
            Edge = Self::Edge,
            Configuration = Self::Configuration,
        >;
    /// Type of the node used in the diagram.
    type Node: Node<Builder = Self::NodeBuilder> + Display;
    /// The type of node builder used to create nodes in the diagram.
    type NodeBuilder: NodeBuilder<Node = Self::Node>;
    /// Type of edge used in the diagram.
    type Edge: Edge<Node = Self::Node> + Display;
    /// The type of edge builder used to create edges in the diagram.
    type EdgeBuilder: Builder<Object = Self::Edge>;
    /// The configuration type for the diagram.
    type Configuration: Configuration + Display;
    /// The configuration builder type for the diagram.
    type ConfigurationBuilder: ConfigurationBuilder<Configuration = Self::Configuration>;

    /// Sets the configuration for the diagram being built.
    ///
    /// # Arguments
    ///
    /// * `configuration` - The configuration builder to use for constructing
    ///   the diagram's configuration.
    ///
    /// # Errors
    ///
    /// * If the configuration builder is incomplete or invalid.
    fn configuration(
        &mut self,
        configuration: Self::ConfigurationBuilder,
    ) -> Result<&mut Self, Self::Error>;

    /// Adds a style class to the diagram being built.
    ///
    /// # Arguments
    ///
    /// * `style_class` - The style class to add to the diagram.
    ///
    /// # Errors
    ///
    /// * If there are conflicting style class names.
    /// * If the style class builder is incomplete or invalid.
    fn style_class(
        &mut self,
        style_class: StyleClassBuilder,
    ) -> Result<Rc<StyleClass>, Self::Error>;

    #[must_use]
    /// Returns the number of nodes currently in the diagram.
    fn number_of_nodes(&self) -> usize;

    /// Builds and adds a node to the diagram being built.
    ///
    /// # Arguments
    ///
    /// * `node` - The node builder to use for constructing the node.
    ///
    /// # Errors
    ///
    /// * If the node cannot be built due to missing attributes or other issues.
    fn node(&mut self, node: Self::NodeBuilder) -> Result<Rc<Self::Node>, Self::Error>;

    /// Builds and adds an edge to the diagram being built.
    ///
    /// # Arguments
    ///
    /// * `edge` - The edge builder to use for constructing the edge.
    ///
    /// # Errors
    ///
    /// * If the source or destination nodes cannot be found in the diagram.
    /// * If the edge cannot be built due to missing attributes or other issues.
    fn edge(&mut self, edge: Self::EdgeBuilder) -> Result<Rc<Self::Edge>, Self::Error>;
}
