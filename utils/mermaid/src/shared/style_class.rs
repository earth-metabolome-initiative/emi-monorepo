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
pub use color::Color;
pub use error::StyleClassError;
pub use font_weight::FontWeight;
pub use style_properties::StyleProperty;
pub use units::Unit;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a style class in Mermaid diagrams, which can be used to define
/// styles for nodes, edges, and other elements. It includes a name and a set of
/// properties that define the style.
pub struct StyleClass {
    /// The name of the style class.
    name: String,
    /// The properties associated with the style class.
    properties: Vec<StyleProperty>,
}

impl StyleClass {
    #[must_use]
    /// Returns the name of the style class.
    pub fn name(&self) -> &str {
        &self.name
    }
    #[must_use]
    /// Returns the properties of the style class.
    pub fn properties(&self) -> &[StyleProperty] {
        &self.properties
    }
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
        writeln!(f)
    }
}
