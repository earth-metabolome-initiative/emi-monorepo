//! Submodule defining the possible errors that can occur in the Mermaid
//! library.

use thiserror::Error;

mod config_error;
pub use config_error::ConfigError;
mod edge_error;
pub use edge_error::EdgeError;
mod node_error;
pub use node_error::NodeError;

pub use crate::shared::style_class::StyleClassError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Enum representing the different types of errors that can occur in the
/// Mermaid library.
pub enum Error {
    /// An error regarding nodes.
    #[error("Node error: {0}")]
    Node(#[from] NodeError),
    /// An error regarding edges.
    #[error("Edge error: {0}")]
    Edge(#[from] EdgeError),
    /// An error regarding diagram configuration.
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    /// An error regarding style classes.
    #[error("Style class error: {0}")]
    StyleClass(#[from] StyleClassError),
}
