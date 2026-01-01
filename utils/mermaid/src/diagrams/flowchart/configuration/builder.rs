//! Submodule providing a builder struct for the configuration of flowchart
//! diagrams in Mermaid syntax.

use crate::{
    diagrams::flowchart::{configuration::FlowchartConfiguration, curve_styles::CurveStyle},
    errors::ConfigError,
    shared::generic_configuration::GenericConfigurationBuilder,
    traits::ConfigurationBuilder,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating flowchart configurations with various properties.
pub struct FlowchartConfigurationBuilder {
    /// Generic configuration options which apply to all Mermaid diagrams.
    generic: GenericConfigurationBuilder,
    /// Whether to enable html labels in the flowchart.
    html_labels: bool,
    /// Whether to automatically wrap markdown labels.
    markdown_auto_wrap: bool,
    /// The curve style used for edges in the flowchart.
    curve_style: CurveStyle,
}

impl FlowchartConfigurationBuilder {
    #[must_use]
    /// Sets whether to enable html labels in the flowchart.
    pub fn html_labels(mut self, enable: bool) -> Self {
        self.html_labels = enable;
        self
    }

    #[must_use]
    /// Sets whether to automatically wrap markdown labels.
    pub fn markdown_auto_wrap(mut self, auto_wrap: bool) -> Self {
        self.markdown_auto_wrap = auto_wrap;
        self
    }

    #[must_use]
    /// Sets the curve style for edges in the flowchart.
    pub fn curve_style(mut self, style: CurveStyle) -> Self {
        self.curve_style = style;
        self
    }
}

impl TryFrom<FlowchartConfigurationBuilder> for FlowchartConfiguration {
    type Error = ConfigError;

    fn try_from(builder: FlowchartConfigurationBuilder) -> Result<Self, Self::Error> {
        Ok(FlowchartConfiguration {
            generic: builder.generic.try_into()?,
            markdown_auto_wrap: builder.markdown_auto_wrap,
            html_labels: builder.html_labels,
            curve_style: builder.curve_style,
        })
    }
}

impl ConfigurationBuilder for FlowchartConfigurationBuilder {
    type Configuration = FlowchartConfiguration;
    type Error = ConfigError;

    fn build(self) -> Result<Self::Configuration, Self::Error> {
        self.try_into()
    }

    fn title<S: ToString>(mut self, title: S) -> Result<Self, Self::Error> {
        self.generic = self.generic.title(title)?;
        Ok(self)
    }

    fn direction(mut self, direction: crate::shared::generic_configuration::Direction) -> Self {
        self.generic = self.generic.direction(direction);
        self
    }

    fn renderer(mut self, renderer: crate::shared::generic_configuration::Renderer) -> Self {
        self.generic = self.generic.renderer(renderer);
        self
    }
}
