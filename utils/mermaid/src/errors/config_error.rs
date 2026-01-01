//! Submodule providing an enumeration of possible errors that can occur in the
//! configuration of diagrams in Mermaid syntax.

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Enum representing errors related to configuration in Mermaid diagrams.
pub enum ConfigError {
    /// The provided diagram title is empty.
    #[error("Configuration title cannot be empty.")]
    EmptyTitle,
}
