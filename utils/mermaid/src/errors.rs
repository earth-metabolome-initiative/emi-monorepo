//! Submodule defining the possible errors that can occur in the Mermaid
//! library.

use std::fmt::{Debug, Display};

mod config_error;
pub use config_error::ConfigError;
mod edge_error;
pub use edge_error::EdgeError;
mod node_error;
pub use node_error::NodeError;

use crate::{
    diagrams::{
        class_diagram::{
            ClassDiagramConfigurationAttribute, ClassEdgeAttribute, ClassNodeAttribute,
        },
        entity_relationship::ERNodeAttribute,
        flowchart::{
            FlowchartConfigurationAttribute, FlowchartEdgeAttribute, FlowchartNodeAttribute,
        },
    },
    shared::{
        generic_configuration::GenericConfigurationAttribute, generic_edge::GenericEdgeAttribute,
        style_class::StyleClassError,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Enum representing the different types of errors that can occur in the
/// Mermaid library.
pub enum Error<NodeAttr, EdgeAttr, ConfigAttr> {
    /// An error regarding nodes.
    Node(NodeError<NodeAttr>),
    /// An error regarding edges.
    Edge(EdgeError<EdgeAttr>),
    /// An error regarding diagram configuration.
    Config(ConfigError<ConfigAttr>),
    /// An error regarding style classes.
    StyleClass(StyleClassError),
}

/// Type alias for the `NodeError` type specialized for class diagrams.
pub type ClassDiagramNodeError = NodeError<ClassNodeAttribute>;
/// Type alias for the `NodeError` type specialized for entity-relationship
/// diagrams.
pub type ERDiagramNodeError = NodeError<ERNodeAttribute>;
/// Type alias for the `NodeError` type specialized for flowcharts.
pub type FlowchartNodeError = NodeError<FlowchartNodeAttribute>;

/// Type alias for the `EdgeError` type specialized for class diagrams.
pub type ClassDiagramEdgeError = EdgeError<ClassEdgeAttribute>;
/// Type alias for the `EdgeError` type specialized for entity-relationship
/// diagrams.
pub type ERDiagramEdgeError = EdgeError<GenericEdgeAttribute>;
/// Type alias for the `EdgeError` type specialized for flowcharts.
pub type FlowchartEdgeError = EdgeError<FlowchartEdgeAttribute>;

/// Type alias for the `ConfigError` type specialized for class diagrams.
pub type ClassDiagramConfigError = ConfigError<ClassDiagramConfigurationAttribute>;
/// Type alias for the `ConfigError` type specialized for entity-relationship
/// diagrams.
pub type ERDiagramConfigError = ConfigError<GenericConfigurationAttribute>;
/// Type alias for the `ConfigError` type specialized for flowcharts.
pub type FlowchartConfigError = ConfigError<FlowchartConfigurationAttribute>;

/// Type alias for the `Error` type specialized for class diagrams.
pub type ClassDiagramError =
    Error<ClassNodeAttribute, ClassEdgeAttribute, ClassDiagramConfigurationAttribute>;
/// Type alias for the `Error` type specialized for entity-relationship
/// diagrams.
pub type ERDiagramError =
    Error<ERNodeAttribute, GenericEdgeAttribute, GenericConfigurationAttribute>;
/// Type alias for the `Error` type specialized for flowcharts.
pub type FlowchartError =
    Error<FlowchartNodeAttribute, FlowchartEdgeAttribute, FlowchartConfigurationAttribute>;

impl<NodeAttr, EdgeAttr, ConfigAttr> From<StyleClassError>
    for Error<NodeAttr, EdgeAttr, ConfigAttr>
{
    fn from(error: StyleClassError) -> Self {
        Error::StyleClass(error)
    }
}

impl<NodeAttr, EdgeAttr, ConfigAttr> From<NodeError<NodeAttr>>
    for Error<NodeAttr, EdgeAttr, ConfigAttr>
{
    fn from(error: NodeError<NodeAttr>) -> Self {
        Error::Node(error)
    }
}

impl<NodeAttr, EdgeAttr, ConfigAttr> From<EdgeError<EdgeAttr>>
    for Error<NodeAttr, EdgeAttr, ConfigAttr>
{
    fn from(error: EdgeError<EdgeAttr>) -> Self {
        Error::Edge(error)
    }
}

impl<NodeAttr, EdgeAttr, ConfigAttr> From<ConfigError<ConfigAttr>>
    for Error<NodeAttr, EdgeAttr, ConfigAttr>
{
    fn from(error: ConfigError<ConfigAttr>) -> Self {
        Error::Config(error)
    }
}

impl<NodeAttr: Display, EdgeAttr: Display, ConfigAttr: Display> Display
    for Error<NodeAttr, EdgeAttr, ConfigAttr>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Node(error) => write!(f, "Node error: `{error}`"),
            Error::Edge(error) => write!(f, "Edge error: `{error}`"),
            Error::Config(error) => write!(f, "Configuration error: `{error}`"),
            Error::StyleClass(error) => write!(f, "Style class error: `{error}`"),
        }
    }
}

impl<
    NodeAttr: Debug + Display + 'static,
    EdgeAttr: Debug + Display + 'static,
    ConfigAttr: Debug + Display + 'static,
> core::error::Error for Error<NodeAttr, EdgeAttr, ConfigAttr>
{
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Error::Node(error) => Some(error),
            Error::Edge(error) => Some(error),
            Error::Config(error) => Some(error),
            Error::StyleClass(error) => Some(error),
        }
    }
}
