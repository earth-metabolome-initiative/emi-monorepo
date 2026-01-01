//! Submodule providing a builder struct for style classes in Mermaid diagrams.

use crate::shared::{
    StyleClass,
    style_class::{StyleClassError, StyleProperty},
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder struct for creating style classes in Mermaid diagrams.
pub struct StyleClassBuilder {
    /// The name of the style class.
    name: Option<String>,
    /// The properties associated with the style class.
    properties: Vec<StyleProperty>,
}

impl TryFrom<StyleClassBuilder> for StyleClass {
    type Error = StyleClassError;

    fn try_from(builder: StyleClassBuilder) -> Result<Self, Self::Error> {
        if builder.properties.is_empty() {
            return Err(StyleClassError::MissingProperties);
        }

        Ok(StyleClass {
            name: builder.name.ok_or(StyleClassError::MissingName)?,
            properties: builder.properties,
        })
    }
}

impl StyleClassBuilder {
    /// Sets the name of the style class.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the style class.
    ///
    /// # Errors
    ///
    /// * Returns `StyleClassError::EmptyName` if the provided name is empty.
    pub fn name(mut self, name: impl Into<String>) -> Result<Self, StyleClassError> {
        let name = name.into();

        if name.is_empty() {
            return Err(StyleClassError::EmptyName);
        }

        self.name = Some(name);

        Ok(self)
    }

    /// Adds a property to the style class.
    ///
    /// # Arguments
    ///
    /// * `property` - A `StyleProperty` that will be added to the style class.
    ///
    /// # Errors
    ///
    /// * Returns `StyleClassError::DuplicateProperty` if the property is
    ///   already present.
    pub fn property(mut self, property: StyleProperty) -> Result<Self, StyleClassError> {
        if self.properties.contains(&property) {
            return Err(StyleClassError::DuplicateProperty(property));
        }

        self.properties.push(property);
        Ok(self)
    }

    /// Builds the style class.
    pub fn build(self) -> Result<StyleClass, StyleClassError> {
        self.try_into()
    }
}
