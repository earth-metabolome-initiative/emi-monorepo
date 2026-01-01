//! Submodule providing an enumeration of possible errors that can occur in the
//! edges of diagrams in Mermaid syntax.

use thiserror::Error;

use crate::shared::ArrowShape;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Enum representing errors related to edges in Mermaid diagrams.
pub enum EdgeError {
    /// The provided edge label is empty.
    #[error("Edge label cannot be empty.")]
    EmptyLabel,
    /// The provided left arrow shape is not compatible with the diagram.
    #[error("Incompatible left arrow shape: `{}`", .0.left())]
    IncompatibleLeftArrowShape(ArrowShape),
    /// The provided right arrow shape is not compatible with the diagram.
    #[error("Incompatible right arrow shape: `{}`", .0.right())]
    IncompatibleRightArrowShape(ArrowShape),
    /// The provided source node does not exist in the diagram.
    #[error("Source node not found: `{0}`")]
    SourceNodeNotFound(String),
    /// The provided destination node does not exist in the diagram.
    #[error("Destination node not found: `{0}`")]
    DestinationNodeNotFound(String),
    /// The source node is missing.
    #[error("Source node is missing.")]
    MissingSource,
    /// The destination node is missing.
    #[error("Destination node is missing.")]
    MissingDestination,
    /// The edge ID is missing.
    #[error("Edge ID is missing.")]
    MissingId,
    /// The edge length is invalid (must be > 0).
    #[error("Edge length must be greater than 0.")]
    InvalidLength,
}
