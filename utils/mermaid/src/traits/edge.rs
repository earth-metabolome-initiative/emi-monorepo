//! Submodule defining the trait for representing edges in Mermaid diagrams.

use std::rc::Rc;

use crate::{
    shared::{ArrowShape, LineStyle},
    traits::{EdgeBuilder, Node},
};

/// Trait representing an edge in a Mermaid diagram.
pub trait Edge: Ord {
    /// Type of the builder used to construct this edge.
    type Builder: EdgeBuilder<Edge = Self>;
    /// Type of the node this edge connects to.
    type Node: Node;

    /// Returns a reference to the label of the edge.
    fn label(&self) -> Option<&str>;

    /// Returns a reference to the source node of the edge.
    fn source(&self) -> &Rc<Self::Node>;

    /// Returns a reference to the destination node of the edge.
    fn destination(&self) -> &Rc<Self::Node>;

    /// Returns the line style of the edge.
    fn line_style(&self) -> LineStyle;

    /// Returns an iterator over the style classes associated with the edge.
    fn classes(&self) -> impl Iterator<Item = &crate::shared::StyleClass>;

    /// Returns the left arrow shape of the edge, if any.
    fn left_arrow_shape(&self) -> Option<ArrowShape>;

    /// Returns the right arrow shape of the edge, if any.
    fn right_arrow_shape(&self) -> Option<ArrowShape>;
}
