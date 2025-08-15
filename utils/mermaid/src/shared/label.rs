//! Submodule providing an enumeration for different types of Mermaid labels.

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents different types of labels that can be used in Mermaid diagrams.
pub enum Label {
    /// A markdown formatted label.
    Markdown(String),
    /// Unicode label.
    Unicode(String),
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Label::Markdown(text) => write!(f, "\"`{text}`\""),
            Label::Unicode(text) => write!(f, "\"{text}\""),
        }
    }
}
