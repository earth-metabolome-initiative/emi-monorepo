//! Submodule defining the trait for structs constructing edges in Mermaid
//! diagrams.

use std::rc::Rc;

use crate::shared::{ArrowShape, LineStyle};

/// Trait representing an edge builder in a Mermaid diagram.
pub trait EdgeBuilder: Sized {
    /// The type of edge this builder constructs.
    type Edge;
    /// Type of the node this edge connects to.
    type Node;

    /// The error type returned when building the edge fails.
    type Error;

    /// Builds the edge.
    fn build(self) -> Result<Self::Edge, Self::Error>;

    /// Set the label for this edge.
    ///
    /// # Arguments
    ///
    /// * `label` - The label to set for this edge.
    ///
    /// # Errors
    ///
    /// * If the label is empty.
    fn label<S: ToString>(self, label: S) -> Result<Self, Self::Error>;

    /// Set the source node for this edge.
    ///
    /// # Arguments
    ///
    /// * `node` - The source node for this edge.
    ///
    /// # Errors
    ///
    /// * If the node is not compatible with any of the other set parameters.
    fn source(self, node: Rc<Self::Node>) -> Result<Self, Self::Error>;

    /// Set the destination node for this edge.
    ///
    /// # Arguments
    ///
    /// * `node` - The destination node for this edge.
    ///
    /// # Errors
    ///
    /// * If the node is not compatible with any of the other set parameters.
    fn destination(self, node: Rc<Self::Node>) -> Result<Self, Self::Error>;

    #[must_use]
    /// Set the style class for this edge.
    ///
    /// # Arguments
    ///
    /// * `class` - The style class to set for this edge.
    fn line_style(self, style: LineStyle) -> Self;

    /// Set the left arrow shape for this edge.
    ///
    /// # Arguments
    ///
    /// * `shape` - The shape of the left arrow.
    ///
    /// # Errors
    ///
    /// * If the shape is not compatible with the associated node type.
    fn left_arrow_shape(self, shape: ArrowShape) -> Result<Self, Self::Error>;

    /// Set the right arrow shape for this edge.
    ///
    /// # Arguments
    ///
    /// * `shape` - The shape of the right arrow.
    ///
    /// # Errors
    ///
    /// * If the shape is not compatible with the associated node type.
    fn right_arrow_shape(self, shape: ArrowShape) -> Result<Self, Self::Error>;
}
