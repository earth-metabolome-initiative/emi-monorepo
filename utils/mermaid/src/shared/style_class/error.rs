//! Submodule defining the error enumeration which describes errors
//! which may happen while creating style classes in Mermaid diagrams.

use common_traits::prelude::BuilderError;

use crate::shared::{
    StyleClass,
    style_class::{StyleProperty, builder::StyleClassAttribute},
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Enum representing the different types of errors that can occur when
/// creating or using style classes in Mermaid diagrams.
pub enum StyleClassError {
    /// The name of the style class is empty.
    EmptyName,
    /// The style class was duplicated.
    DuplicateClass(String),
    /// The property was duplicated.
    DuplicateProperty(StyleProperty),
    /// The style class is unknown in the context of the diagram.
    UnknownClass(StyleClass),
    /// Builder errors.
    Builder(BuilderError<StyleClassAttribute>),
}

impl From<BuilderError<StyleClassAttribute>> for StyleClassError {
    fn from(error: BuilderError<StyleClassAttribute>) -> Self {
        StyleClassError::Builder(error)
    }
}

impl std::fmt::Display for StyleClassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StyleClassError::EmptyName => write!(f, "Style class name cannot be empty."),
            StyleClassError::DuplicateProperty(property) => {
                write!(f, "Duplicate property found: `{property}`")
            }
            StyleClassError::UnknownClass(class) => {
                write!(f, "Unknown style class: `{}`", class.name())
            }
            StyleClassError::DuplicateClass(name) => write!(f, "Duplicate style class: `{name}`"),
            StyleClassError::Builder(error) => write!(f, "Builder error: {error}"),
        }
    }
}

impl core::error::Error for StyleClassError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            StyleClassError::Builder(error) => Some(error),
            _ => None,
        }
    }
}
