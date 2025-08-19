//! Submodule defining configuration specifically for flowchart diagrams in
//! Mermaid.

use crate::{diagrams::flowchart::curve_styles::CurveStyle, shared::configuration::Configuration};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlowchartConfiguration {
    /// Shared configuration options which apply to all Mermaid diagrams.
    shared: Configuration,
    /// Whether to enable html labels in the flowchart.
    html_labels: bool,
    /// The curve style used for edges in the flowchart.
    curve_style: CurveStyle,
}
