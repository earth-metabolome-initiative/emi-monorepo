//! Submodule defining configuration specifically for entity-relationship
//! diagrams in Mermaid.
use crate::shared::configuration::Configuration;

/// Configuration for entity-relationship diagrams in Mermaid.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntityRelationshipConfiguration {
    /// Shared configuration options which apply to all Mermaid diagrams.
    shared: Configuration,
}
