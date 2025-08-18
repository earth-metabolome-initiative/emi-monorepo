//! Submodule providing a builder struct for style classes in Mermaid diagrams.

use crate::shared::style_class::{StyleClassError, StyleProperty};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Builder struct for creating style classes in Mermaid diagrams.
pub struct StyleClassBuilder {
    /// The name of the style class.
    name: Option<String>,
    /// The properties associated with the style class.
    properties: Vec<StyleProperty>,
}

impl StyleClassBuilder {
    /// Sets the name of the style class.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the style class.
    pub fn name(mut self, name: impl Into<String>) -> Result<Self, StyleClassError> {
        self.name = Some(name.into());
        Ok(self)
    }

    /// Adds a property to the style class.
    ///
    /// # Arguments
    ///
    /// * `property` - A `StyleProperty` that will be added to the style class.
    pub fn property(mut self, property: StyleProperty) -> Result<Self, StyleClassError> {
        if self.properties.contains(&property) {
            return Err(StyleClassError::DuplicateProperty(property));
        }

        self.properties.push(property);
        Ok(self)
    }
}
