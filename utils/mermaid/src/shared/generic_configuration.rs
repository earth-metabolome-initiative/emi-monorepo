//! Submodule defining the configuration options which are applied at the top
//! level of a Mermaid diagram.

mod renderers;
use std::fmt::Display;

use common_traits::prelude::Builder;
pub use renderers::Renderer;
mod direction;
pub use direction::Direction;

use crate::{
    errors::ConfigError,
    traits::{Configuration, ConfigurationBuilder},
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the configuration options for a Mermaid diagram.
pub struct GenericConfiguration {
    /// The title of the diagram.
    title: Option<String>,
    /// The renderer to use for the diagram.
    renderer: Renderer,
    /// The direction of the flowchart.
    direction: Direction,
}

impl Configuration for GenericConfiguration {
    type Builder = GenericConfigurationBuilder;

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    fn direction(&self) -> &Direction {
        &self.direction
    }
}

impl Display for GenericConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "---")?;
        if let Some(title) = &self.title {
            writeln!(f, "title: {}", title)?;
        }
        writeln!(f, "config:")?;
        writeln!(f, "  layout: {}", self.renderer)?;
        writeln!(f, "---")?;

        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating a `GenericConfiguration`.
pub struct GenericConfigurationBuilder {
    /// The title of the diagram.
    title: Option<String>,
    /// The renderer to use for the diagram.
    renderer: Renderer,
    /// The direction of the flowchart.
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GenericConfigurationAttribute {
    /// Title of the diagram.
    Title,
    /// Renderer used for the diagram.
    Renderer,
    /// Direction of the flowchart.
    Direction,
}

impl Display for GenericConfigurationAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericConfigurationAttribute::Title => write!(f, "title"),
            GenericConfigurationAttribute::Renderer => write!(f, "renderer"),
            GenericConfigurationAttribute::Direction => write!(f, "direction"),
        }
    }
}

impl Builder for GenericConfigurationBuilder {
    type Error = ConfigError<GenericConfigurationAttribute>;
    type Object = GenericConfiguration;
    type Attribute = GenericConfigurationAttribute;

    fn is_complete(&self) -> bool {
        true
    }

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(GenericConfiguration {
            title: self.title,
            renderer: self.renderer,
            direction: self.direction,
        })
    }
}

impl ConfigurationBuilder for GenericConfigurationBuilder {
    type Configuration = GenericConfiguration;

    fn title<S: ToString>(mut self, title: S) -> Result<Self, Self::Error> {
        let title = title.to_string();
        if title.is_empty() {
            return Err(ConfigError::EmptyTitle);
        }
        self.title = Some(title);
        Ok(self)
    }

    fn renderer(mut self, renderer: Renderer) -> Self {
        self.renderer = renderer;
        self
    }

    fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
}
