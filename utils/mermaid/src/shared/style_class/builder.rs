//! Submodule providing a builder struct for style classes in Mermaid diagrams.

use std::fmt::Display;

use common_traits::prelude::{Builder, BuilderError};

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StyleClassAttribute {
    Name,
    Properties,
}

impl Display for StyleClassAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StyleClassAttribute::Name => write!(f, "name"),
            StyleClassAttribute::Properties => write!(f, "properties"),
        }
    }
}

impl Builder for StyleClassBuilder {
    type Attribute = StyleClassAttribute;
    type Error = StyleClassError;
    type Object = StyleClass;

    fn is_complete(&self) -> bool {
        self.name.is_some() && !self.properties.is_empty()
    }

    fn build(self) -> Result<Self::Object, Self::Error> {
        if self.properties.is_empty() {
            return Err(BuilderError::IncompleteBuild(StyleClassAttribute::Properties).into());
        }

        Ok(StyleClass {
            name: self.name.ok_or(BuilderError::IncompleteBuild(StyleClassAttribute::Name))?,
            properties: self.properties,
        })
    }
}

impl StyleClassBuilder {
    /// Sets the name of the style class.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the style class.
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
    pub fn property(mut self, property: StyleProperty) -> Result<Self, StyleClassError> {
        if self.properties.contains(&property) {
            return Err(StyleClassError::DuplicateProperty(property));
        }

        self.properties.push(property);
        Ok(self)
    }
}
