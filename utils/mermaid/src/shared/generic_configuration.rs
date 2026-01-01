//! Submodule defining the configuration options which are applied at the top
//! level of a Mermaid diagram.

mod renderers;
use std::fmt::Display;

pub use renderers::Renderer;
mod direction;
pub use direction::Direction;
mod theme;
pub use theme::Theme;
mod look;
pub use look::Look;

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
    /// The theme to use for the diagram.
    theme: Theme,
    /// The look to use for the diagram.
    look: Look,
}

impl Configuration for GenericConfiguration {
    type Builder = GenericConfigurationBuilder;

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn renderer(&self) -> Renderer {
        self.renderer
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn theme(&self) -> Theme {
        self.theme
    }

    fn look(&self) -> Look {
        self.look
    }
}

impl Display for GenericConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "---")?;
        writeln!(f, "config:")?;
        writeln!(f, "  layout: {}", self.renderer)?;
        writeln!(f, "  theme: {}", self.theme)?;
        writeln!(f, "  look: {}", self.look)?;
        if let Some(title) = &self.title {
            writeln!(f, "title: {title}")?;
        }
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
    /// The theme to use for the diagram.
    theme: Theme,
    /// The look to use for the diagram.
    look: Look,
}

impl TryFrom<GenericConfigurationBuilder> for GenericConfiguration {
    type Error = ConfigError;

    fn try_from(builder: GenericConfigurationBuilder) -> Result<Self, Self::Error> {
        Ok(GenericConfiguration {
            title: builder.title,
            renderer: builder.renderer,
            direction: builder.direction,
            theme: builder.theme,
            look: builder.look,
        })
    }
}

impl ConfigurationBuilder for GenericConfigurationBuilder {
    type Configuration = GenericConfiguration;
    type Error = ConfigError;

    fn build(self) -> Result<Self::Configuration, Self::Error> {
        self.try_into()
    }

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
