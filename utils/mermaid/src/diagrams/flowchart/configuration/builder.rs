//! Submodule providing a builder struct for the configuration of flowchart
//! diagrams in Mermaid syntax.

use std::fmt::Display;

use common_traits::prelude::Builder;

use crate::{
    diagrams::flowchart::{configuration::FlowchartConfiguration, curve_styles::CurveStyle},
    errors::ConfigError,
    shared::generic_configuration::{GenericConfigurationAttribute, GenericConfigurationBuilder},
    traits::ConfigurationBuilder,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlowchartConfigurationBuilder {
    /// Generic configuration options which apply to all Mermaid diagrams.
    generic: GenericConfigurationBuilder,
    /// Whether to enable html labels in the flowchart.
    html_labels: bool,
    /// The curve style used for edges in the flowchart.
    curve_style: CurveStyle,
}

impl FlowchartConfigurationBuilder {
    /// Sets whether to enable html labels in the flowchart.
    pub fn html_labels(mut self, enable: bool) -> Self {
        self.html_labels = enable;
        self
    }

    /// Sets the curve style for edges in the flowchart.
    pub fn curve_style(mut self, style: CurveStyle) -> Self {
        self.curve_style = style;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the configuration attributes specific to flowchart diagrams.
pub enum FlowchartConfigurationAttribute {
    /// Generic configuration attribute.
    Generic(GenericConfigurationAttribute),
    /// HTML labels attribute.
    HtmlLabels,
    /// Curve style attribute.
    CurveStyle,
}

impl From<GenericConfigurationAttribute> for FlowchartConfigurationAttribute {
    fn from(attr: GenericConfigurationAttribute) -> Self {
        FlowchartConfigurationAttribute::Generic(attr)
    }
}

impl Display for FlowchartConfigurationAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlowchartConfigurationAttribute::Generic(attr) => write!(f, "{attr}"),
            FlowchartConfigurationAttribute::HtmlLabels => write!(f, "html_labels"),
            FlowchartConfigurationAttribute::CurveStyle => write!(f, "curve_style"),
        }
    }
}

impl Builder for FlowchartConfigurationBuilder {
    type Object = FlowchartConfiguration;
    type Attribute = FlowchartConfigurationAttribute;
    type Error = ConfigError<Self::Attribute>;

    fn is_complete(&self) -> bool {
        self.generic.is_complete()
    }

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(FlowchartConfiguration {
            generic: self.generic.build()?,
            html_labels: self.html_labels,
            curve_style: self.curve_style,
        })
    }
}

impl ConfigurationBuilder for FlowchartConfigurationBuilder {
    type Configuration = FlowchartConfiguration;

    fn title<S: ToString>(mut self, title: S) -> Result<Self, Self::Error> {
        self.generic = self.generic.title(title)?;
        Ok(self)
    }

    fn direction(mut self, direction: crate::shared::generic_configuration::Direction) -> Self {
        self.generic = self.generic.direction(direction);
        self
    }

    fn markdown_auto_wrap(mut self, auto_wrap: bool) -> Self {
        self.generic = self.generic.markdown_auto_wrap(auto_wrap);
        self
    }

    fn renderer(mut self, renderer: crate::shared::generic_configuration::Renderer) -> Self {
        self.generic = self.generic.renderer(renderer);
        self
    }
}
