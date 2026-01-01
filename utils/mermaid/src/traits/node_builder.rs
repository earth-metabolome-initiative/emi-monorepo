//! Submodule defining the traits for building a node in Mermaid diagrams.

use std::rc::Rc;

use crate::shared::{StyleClass, StyleClassError, StyleProperty};

/// Trait for building nodes in Mermaid diagrams.
pub trait NodeBuilder: Sized {
    /// Type of the node that this builder constructs.
    type Node;

    /// The error type returned when building the node fails.
    type Error;

    /// Builds the node.
    fn build(self) -> Result<Self::Node, Self::Error>;

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

    /// Iterates across the style properties of the node being built.
    fn style_properties(&self) -> impl Iterator<Item = &StyleProperty>;

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

    /// Returns a reference to the label of the node being built, if set.
    fn get_label(&self) -> Option<&String>;

    #[must_use]
    /// Sets the ID for the node being built.
    fn id(self, id: u64) -> Self;

    /// Returns the ID of the node being built, if set.
    fn get_id(&self) -> Option<u64>;
}
