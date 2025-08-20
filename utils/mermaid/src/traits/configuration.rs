//! Submodule defining the `Configuration` trait for Mermaid diagrams.

use crate::{
    shared::generic_configuration::{Direction, Renderer},
    traits::ConfigurationBuilder,
};

/// Trait defining the configuration for Mermaid diagrams.
pub trait Configuration {
    /// The builder type for this configuration.
    type Builder: ConfigurationBuilder<Configuration = Self>;

    /// Returns the title of the diagram, if any.
    fn title(&self) -> Option<&str>;

    /// Returns whether markdown labels should be automatically wrapped.
    fn markdown_auto_wrap(&self) -> bool;

    /// Returns the renderer to use for the diagram.
    fn renderer(&self) -> &Renderer;

    /// Returns the direction of the flowchart.
    fn direction(&self) -> &Direction;
}
