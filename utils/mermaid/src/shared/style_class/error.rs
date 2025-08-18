//! Submodule defining the error enumeration which describes errors
//! which may happen while creating style classes in Mermaid diagrams.

use crate::shared::style_class::StyleProperty;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing the different types of errors that can occur when
/// creating or using style classes in Mermaid diagrams.
pub enum StyleClassError {
    /// The name of the style class is empty.
    EmptyName,
    /// The property was duplicated.
    DuplicateProperty(StyleProperty),
}
