//! Submodule defining the `Configuration` trait for Mermaid diagrams.

use std::fmt::Display;

use crate::{
    shared::generic_configuration::{Direction, Renderer},
    traits::ConfigurationBuilder,
};

/// Trait defining the configuration for Mermaid diagrams.
pub trait Configuration: Default + Display {
    /// The builder type for this configuration.
    type Builder: ConfigurationBuilder<Configuration = Self>;

    /// Returns the title of the diagram, if any.
    fn title(&self) -> Option<&str>;

    /// Returns the renderer to use for the diagram.
    fn renderer(&self) -> &Renderer;

    /// Returns the direction of the flowchart.
    fn direction(&self) -> &Direction;
}
