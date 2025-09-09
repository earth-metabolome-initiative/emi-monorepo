//! Submodule defining the possible renderers which may be used in a flowchart
//! configuration in Mermaid.

use std::fmt::Display;

#[derive(Default, Copy, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the renderer used for Mermaid diagrams.
pub enum Renderer {
    /// The dagre renderer, which is the default renderer for flowcharts.
    #[default]
    Dagre,
    /// The newer Eclipse Layout Kernel (ELK) renderer, which is an alternative
    /// to the dagre renderer.
    EclipseLayoutKernel,
}

impl Display for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Renderer::Dagre => write!(f, "dagre"),
            Renderer::EclipseLayoutKernel => write!(f, "elk"),
        }
    }
}
