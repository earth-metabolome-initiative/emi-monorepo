//! Submodule providing an enumeration for the possible link line styles in
//! Mermaid diagrams.

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the line style of links in Mermaid diagrams.
pub enum LineStyle {
    /// A normal line style.
    #[default]
    Solid,
    /// A Thick line style.
    Thick,
    /// A Dashed line style.
    Dashed,
}
