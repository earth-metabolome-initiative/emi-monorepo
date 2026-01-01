//! Submodule defining configuration specifically for flowchart diagrams in
//! Mermaid.

mod builder;

use std::fmt::Display;

pub use builder::FlowchartConfigurationBuilder;

use crate::{
    diagrams::flowchart::curve_styles::CurveStyle,
    shared::{
        Direction, Renderer,
        generic_configuration::{GenericConfiguration, Look, Theme},
    },
    traits::Configuration,
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the configuration for a flowchart diagram in Mermaid syntax.
pub struct FlowchartConfiguration {
    /// Generic configuration options which apply to all Mermaid diagrams.
    generic: GenericConfiguration,
    /// Whether to automatically wrap markdown labels.
    markdown_auto_wrap: bool,
    /// Whether to enable html labels in the flowchart.
    html_labels: bool,
    /// The curve style used for edges in the flowchart.
    curve_style: CurveStyle,
}

impl Display for FlowchartConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.generic.title().is_none() && self.renderer() == Renderer::default() {
            return Ok(());
        }
        writeln!(f, "---")?;
        writeln!(f, "config:")?;
        writeln!(f, "  theme: {}", self.theme())?;
        writeln!(f, "  look: {}", self.look())?;
        writeln!(f, "  flowchart:")?;
        writeln!(f, "    defaultRenderer: \"{}\"", self.renderer())?;
        if let Some(title) = &self.generic.title() {
            writeln!(f, "title: {title}")?;
        }
        writeln!(f, "---")?;

        Ok(())
    }
}

impl Configuration for FlowchartConfiguration {
    type Builder = FlowchartConfigurationBuilder;

    fn title(&self) -> Option<&str> {
        self.generic.title()
    }

    fn direction(&self) -> Direction {
        self.generic.direction()
    }

    fn renderer(&self) -> Renderer {
        self.generic.renderer()
    }

    fn theme(&self) -> Theme {
        self.generic.theme()
    }

    fn look(&self) -> Look {
        self.generic.look()
    }
}
