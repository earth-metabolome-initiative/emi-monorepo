//! Submodule defining the `DiagramBuilder` trait for Mermaid diagrams.

use crate::{shared::StyleClass, traits::Diagram};

/// Trait defining the builder for Mermaid diagrams.
pub trait DiagramBuilder {
    /// Type of the diagram that this builder constructs.
    type Diagram: Diagram<Builder = Self>;
    /// Type

    /// Adds a style class to the diagram being built.
    fn style_class(&mut self, style_class: StyleClass) -> &mut Self;
}
