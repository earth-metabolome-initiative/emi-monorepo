//! Submodule defining a generic diagram struct which can be used as a base
//! for various types of diagrams in Mermaid syntax.

use crate::shared::StyleClass;

/// A generic diagram struct that can be extended for specific diagram types.
pub(crate) struct GenericDiagram {
    /// Style classes associated with this diagram.
    style_classes: Vec<StyleClass>,
}
