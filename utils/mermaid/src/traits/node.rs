//! Submodule defining the node trait used in Mermaid diagrams.

use crate::{
    shared::{ArrowShape, StyleClass, StyleProperty},
    traits::NodeBuilder,
};

/// Trait representing a node in a Mermaid diagram.
pub trait Node: PartialOrd + Ord + Eq + PartialEq {
    /// Type of the builder used to construct this node.
    type Builder: NodeBuilder<Node = Self>;

    /// Returns the identifier of the node.
    fn id(&self) -> u64;

    /// Returns the label of the node.
    fn label(&self) -> &str;

    /// Returns an iterator over the CSS classes associated with the node.
    fn classes(&self) -> impl Iterator<Item = &StyleClass>;

    /// Returns an iterator over the style properties associated with the node.
    fn styles(&self) -> impl Iterator<Item = &StyleProperty>;

    /// Returns whether the node is stylized.
    fn has_styles(&self) -> bool {
        self.styles().next().is_some()
    }

    /// Returns whether the provided arrow shape is compatible with the node.
    fn is_compatible_arrow_shape(shape: ArrowShape) -> bool;
}
