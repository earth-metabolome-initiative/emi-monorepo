//! The looks enumeration to use for rendering a Mermaid diagram.

use std::fmt::Display;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The looks enumeration to use for rendering a Mermaid diagram.
pub enum Look {
    /// The Neo look, a modern style for diagrams.
    Neo,
    /// The hand-drawn look, a sketch-like style for diagrams.
    HandDrawn,
    #[default]
    /// The Classic look, the traditional Mermaid style.
    Classic,
}

impl Display for Look {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Look::Neo => "neo",
                Look::HandDrawn => "handDrawn",
                Look::Classic => "classic",
            }
        )
    }
}
