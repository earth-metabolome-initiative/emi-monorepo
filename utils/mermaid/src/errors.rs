//! Submodule defining the possible errors that can occur in the Mermaid
//! library.

use std::fmt::{Debug, Display};

mod config_error;
pub use config_error::ConfigError;
mod edge_error;
pub use edge_error::EdgeError;
mod node_error;
pub use node_error::NodeError;

use crate::shared::style_class::StyleClassError;

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
