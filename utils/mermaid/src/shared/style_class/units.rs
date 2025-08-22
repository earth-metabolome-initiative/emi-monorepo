//! Submodule providing an enumeration of units which may be used in
//! style class definitions in Mermaid diagrams, including pixel and
//! point units.

use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the unit of measurement used in style class definitions.
pub enum Unit {
    /// Pixel unit, denoted by `px`.
    Pixel(u8),
    /// Point unit, denoted by `pt`.
    Point(u8),
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Pixel(value) => write!(f, "{value}px"),
            Unit::Point(value) => write!(f, "{value}pt"),
        }
    }
}
