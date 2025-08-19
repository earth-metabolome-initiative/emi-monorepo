//! Submodule defining the traits for building a node in Mermaid diagrams.

use std::rc::Rc;

use common_traits::prelude::Builder;

use crate::{
    shared::{StyleClass, StyleClassError, StyleProperty},
    traits::Node,
};

/// Trait for building nodes in Mermaid diagrams.
pub trait NodeBuilder: Builder {
    /// Type of the node that this builder constructs.
    type Node: Node<Builder = Self>;

    /// Adds the provided style class to the node being built.
    fn style_class(&mut self, style_class: Rc<StyleClass>) -> Result<&mut Self, StyleClassError>;

    /// Adds a style property to the node being built.
    fn style_property(&mut self, property: StyleProperty) -> Result<&mut Self, StyleClassError>;

    /// Sets the label for the node being built.
    fn label<S: ToString>(&mut self, label: S) -> Result<&mut Self, Self::Error>;

    /// Sets the ID for the node being built.
    fn id(&mut self, id: u32) -> &mut Self;
}
