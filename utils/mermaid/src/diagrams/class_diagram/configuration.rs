//! Submodule defining configuration specifically for class diagrams in
//! Mermaid.

use crate::shared::configuration::Configuration;

pub struct ClassDiagramConfiguration {
    /// Shared configuration options which apply to all Mermaid diagrams.
    shared: Configuration,
    /// Whether to hide empty members in the class diagram.
    hide_empty_members_box: bool,
}
