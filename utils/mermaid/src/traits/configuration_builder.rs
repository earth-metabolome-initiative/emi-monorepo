//! Submodule defining the `ConfigurationBuilder` trait for Mermaid diagrams.

use crate::shared::generic_configuration::{Direction, Renderer};

/// Trait defining a configuration builder for Mermaid diagrams.
pub trait ConfigurationBuilder: Sized {
    /// Type of the configuration that this builder constructs.
    type Configuration;

    /// The error type returned when building the configuration fails.
    type Error;

    /// Builds the configuration.
    fn build(self) -> Result<Self::Configuration, Self::Error>;

    /// Returns the current title of the configuration, if any.
    ///
    /// # Arguments
    ///
    /// * `title` - The title to set for the configuration.
    ///
    /// # Errors
    ///
    /// * If the provided title is empty.
    fn title<S: ToString>(self, title: S) -> Result<Self, Self::Error>;

    #[must_use]
    /// Sets the renderer to use for the diagram.
    fn renderer(self, renderer: Renderer) -> Self;

    #[must_use]
    /// Sets the direction of the flowchart.
    fn direction(self, direction: Direction) -> Self;
}
