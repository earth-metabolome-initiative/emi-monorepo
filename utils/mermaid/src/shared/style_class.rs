//! Submodule providing structs for style class definitions in Mermaid diagrams.

mod builder;
mod color;
mod error;
mod font_style;
mod font_weight;
mod style_properties;
mod units;
use std::fmt::Display;

pub use builder::StyleClassBuilder;
pub use error::StyleClassError;
pub use style_properties::StyleProperty;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StyleClass {
    /// The name of the style class.
    name: String,
    /// The properties associated with the style class.
    properties: Vec<StyleProperty>,
}

impl Display for StyleClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "classDef {} ", self.name)?;
        for (property_number, property) in self.properties.iter().enumerate() {
            if property_number > 0 {
                write!(f, ",")?;
            }
            write!(f, "{property}")?;
        }
        write!(f, ";")
    }
}
