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
    ///
    /// # Arguments
    ///
    /// * `style_class` - The style class to add to the node.
    ///
    /// # Errors
    ///
    /// * If the style class name clashes with a previously applied class.
    fn style_class(self, style_class: Rc<StyleClass>) -> Result<Self, StyleClassError>;

    /// Adds a style property to the node being built.
    ///
    /// # Arguments
    ///
    /// * `property` - The style property to add to the node.
    ///
    /// # Errors
    ///
    /// * If the style property has already been set for the node.
    fn style_property(self, property: StyleProperty) -> Result<Self, StyleClassError>;

    /// Sets the label for the node being built.
    ///
    /// # Arguments
    ///
    /// * `label` - The label to set for the node.
    ///
    /// # Errors
    ///
    /// * If the label is empty.
    fn label<S: ToString>(self, label: S) -> Result<Self, Self::Error>;

    #[must_use]
    /// Sets the ID for the node being built.
    fn id(self, id: u32) -> Self;
}
