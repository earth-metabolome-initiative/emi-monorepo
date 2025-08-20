//! Submodule defining configuration specifically for flowchart diagrams in
//! Mermaid.

mod builder;

use std::fmt::Display;

pub use builder::FlowchartConfigurationAttribute;

use crate::{
    diagrams::flowchart::{
        configuration::builder::FlowchartConfigurationBuilder, curve_styles::CurveStyle,
    },
    shared::generic_configuration::GenericConfiguration,
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
        writeln!(f, "---")?;
        if let Some(title) = &self.generic.title() {
            writeln!(f, "title: {}", title)?;
        }
        writeln!(f, "config:")?;
        writeln!(f, "  flowchart:")?;
        writeln!(f, "    defaultRenderer: \"{}\"", self.renderer())?;
        writeln!(f, "---")?;

        Ok(())
    }
}

impl Configuration for FlowchartConfiguration {
    type Builder = FlowchartConfigurationBuilder;

    fn title(&self) -> Option<&str> {
        self.generic.title()
    }

    fn direction(&self) -> &crate::shared::generic_configuration::Direction {
        self.generic.direction()
    }

    fn renderer(&self) -> &crate::shared::generic_configuration::Renderer {
        self.generic.renderer()
    }
}
