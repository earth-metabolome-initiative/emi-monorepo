//! Submodule providing an enumeration of possible errors that can occur in the
//! nodes of diagrams in Mermaid syntax.

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Enum representing errors related to nodes in Mermaid diagrams.
pub enum NodeError {
    /// The provided node label is empty.
    #[error("Node label cannot be empty.")]
    EmptyLabel,
    /// The provided node ID is empty.
    #[error("Node ID cannot be empty.")]
    EmptyId,
    /// The provided node ID contains invalid characters.
    #[error("Node ID `{0}` contains invalid characters.")]
    InvalidId(String),
    /// The provided node already exists in the diagram.
    #[error("Node `{0}` already exists.")]
    DuplicateNode(String),
    /// The node ID is missing.
    #[error("Node ID is missing.")]
    MissingId,
    /// The node label is missing.
    #[error("Node label is missing.")]
    MissingLabel,
    /// The subnodes are missing (required for subgraph with direction).
    #[error("Subnodes are missing.")]
    MissingSubnodes,
}
