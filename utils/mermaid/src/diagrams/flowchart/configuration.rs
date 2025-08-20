//! Submodule defining configuration specifically for flowchart diagrams in
//! Mermaid.

mod builder;

pub use builder::FlowchartConfigurationAttribute;

use crate::{
    diagrams::flowchart::{
        configuration::builder::FlowchartConfigurationBuilder, curve_styles::CurveStyle,
    },
    shared::generic_configuration::GenericConfiguration,
    traits::Configuration,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlowchartConfiguration {
    /// Generic configuration options which apply to all Mermaid diagrams.
    generic: GenericConfiguration,
    /// Whether to enable html labels in the flowchart.
    html_labels: bool,
    /// The curve style used for edges in the flowchart.
    curve_style: CurveStyle,
}

impl Configuration for FlowchartConfiguration {
    type Builder = FlowchartConfigurationBuilder;

    fn title(&self) -> Option<&str> {
        self.generic.title()
    }

    fn direction(&self) -> &crate::shared::generic_configuration::Direction {
        self.generic.direction()
    }

    fn markdown_auto_wrap(&self) -> bool {
        self.generic.markdown_auto_wrap()
    }

    fn renderer(&self) -> &crate::shared::generic_configuration::Renderer {
        self.generic.renderer()
    }
}
