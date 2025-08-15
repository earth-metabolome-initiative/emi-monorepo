//! Submodule defining configuration specifically for flowchart diagrams in
//! Mermaid.

use crate::shared::configuration::Configuration;

pub struct FlowchartConfiguration {
    /// Shared configuration options which apply to all Mermaid diagrams.
    shared: Configuration,
    /// Whether to enable html labels in the flowchart.
    html_labels: bool,
}
