//! Submodule defining the error enumeration which describes errors
//! which may happen while creating style classes in Mermaid diagrams.

use thiserror::Error;

use crate::shared::{StyleClass, style_class::StyleProperty};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Error)]
/// Enum representing the different types of errors that can occur when
/// creating or using style classes in Mermaid diagrams.
pub enum StyleClassError {
    /// The name of the style class is empty.
    #[error("Style class name cannot be empty.")]
    EmptyName,
    /// The style class was duplicated.
    #[error("Duplicate style class: `{0}`")]
    DuplicateClass(String),
    /// The property was duplicated.
    #[error("Duplicate property found: `{0}`")]
    DuplicateProperty(StyleProperty),
    /// The style class is unknown in the context of the diagram.
    #[error("Unknown style class: `{}`", .0.name())]
    UnknownClass(StyleClass),
    /// The name of the style class is missing.
    #[error("Style class name is missing.")]
    MissingName,
    /// The properties of the style class are missing.
    #[error("Style class properties are missing.")]
    MissingProperties,
}
