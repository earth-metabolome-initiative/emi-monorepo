//! Submodule defining the `Diagram` trait for Mermaid diagrams.

use std::fmt::Display;

use crate::{shared::StyleClass, traits::DiagramBuilder};

/// Trait representing a Mermaid diagram.
pub trait Diagram: Display {
    /// Type of the associated builder for this diagram.
    type Builder: DiagramBuilder<Diagram = Self>;
    /// Type of the node used in the diagram.
    type Node: Display;

    /// Iterates across the style classes associated with this diagram.
    fn style_classes(&self) -> impl Iterator<Item = &StyleClass>;

    /// Iterates across the nodes in this diagram.
    fn nodes(&self) -> impl Iterator<Item = &Self::Node>;
}
