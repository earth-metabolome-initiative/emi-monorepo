//! Submodule providing an enumeration for visibility modifiers employable
//! in class entries of Mermaid class diagrams.
//!
//! These include: Public (`+`), Private (`-`), Protected (`#`), and
//! Package/Internal (`~`).

use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// An enumeration representing the visibility of class members in Mermaid class
/// diagrams.
pub enum Visibility {
    /// Public visibility, denoted by `+`.
    Public,
    /// Private visibility, denoted by `-`.
    Private,
    /// Protected visibility, denoted by `#`.
    Protected,
    /// Package/Internal visibility, denoted by `~`.
    Package,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Public => write!(f, "+"),
            Visibility::Private => write!(f, "-"),
            Visibility::Protected => write!(f, "#"),
            Visibility::Package => write!(f, "~"),
        }
    }
}
