//! Submodule providing a generic node struct which may be reused across
//! different diagrams.

use std::{iter::empty, rc::Rc};

use crate::{
    errors::EdgeError,
    shared::{ArrowShape, LineStyle, StyleClass},
    traits::{Edge, EdgeBuilder, Node},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Struct representing a generic node in Mermaid diagrams.
pub struct GenericEdge<Node> {
    /// Label for the node.
    label: Option<String>,
    /// The source node of the edge.
    source: Rc<Node>,
    /// The destination node of the edge.
    destination: Rc<Node>,
    /// The line style of the link.
    line_style: LineStyle,
    /// The left arrow shape of the link, if any.
    left_arrow_shape: Option<ArrowShape>,
    /// The right arrow shape of the link, if any.
    right_arrow_shape: Option<ArrowShape>,
}

impl<N: Node> Edge for GenericEdge<N> {
    type Builder = GenericEdgeBuilder<N>;
    type Node = N;

    fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn source(&self) -> &Rc<Self::Node> {
        &self.source
    }

    fn destination(&self) -> &Rc<Self::Node> {
        &self.destination
    }

    fn line_style(&self) -> LineStyle {
        self.line_style
    }

    fn classes(&self) -> impl Iterator<Item = &StyleClass> {
        empty()
    }

    fn left_arrow_shape(&self) -> Option<ArrowShape> {
        self.left_arrow_shape
    }

    fn right_arrow_shape(&self) -> Option<ArrowShape> {
        self.right_arrow_shape
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating a `GenericEdge`.
pub struct GenericEdgeBuilder<Node> {
    /// Label for the edge.
    label: Option<String>,
    /// Source node for the edge.
    source: Option<Rc<Node>>,
    /// Destination node for the edge.
    destination: Option<Rc<Node>>,
    /// Line style of the edge.
    line_style: LineStyle,
    /// Left arrow shape of the edge, if any.
    left_arrow_shape: Option<ArrowShape>,
    /// Right arrow shape of the edge, if any.
    right_arrow_shape: Option<ArrowShape>,
}

impl<Node> Default for GenericEdgeBuilder<Node> {
    fn default() -> Self {
        Self {
            label: None,
            source: None,
            destination: None,
            line_style: LineStyle::default(),
            left_arrow_shape: None,
            right_arrow_shape: None,
        }
    }
}

impl<N> TryFrom<GenericEdgeBuilder<N>> for GenericEdge<N> {
    type Error = EdgeError;

    fn try_from(builder: GenericEdgeBuilder<N>) -> Result<Self, Self::Error> {
        Ok(GenericEdge {
            label: builder.label,
            source: builder.source.ok_or(EdgeError::MissingSource)?,
            destination: builder.destination.ok_or(EdgeError::MissingDestination)?,
            line_style: builder.line_style,
            left_arrow_shape: builder.left_arrow_shape,
            right_arrow_shape: builder.right_arrow_shape,
        })
    }
}

impl<N: Node> EdgeBuilder for GenericEdgeBuilder<N> {
    type Node = N;
    type Edge = GenericEdge<N>;
    type Error = EdgeError;

    fn build(self) -> Result<Self::Edge, Self::Error> {
        self.try_into()
    }

    fn label<S: ToString>(mut self, label: S) -> Result<Self, Self::Error> {
        let label = label.to_string();
        if label.is_empty() {
            return Err(EdgeError::EmptyLabel);
        }
        self.label = Some(label);
        Ok(self)
    }

    fn source(mut self, source: Rc<Self::Node>) -> Result<Self, Self::Error> {
        self.source = Some(source);
        Ok(self)
    }

    fn destination(mut self, destination: Rc<Self::Node>) -> Result<Self, Self::Error> {
        self.destination = Some(destination);
        Ok(self)
    }

    fn line_style(mut self, style: LineStyle) -> Self {
        self.line_style = style;
        self
    }
    fn left_arrow_shape(mut self, shape: ArrowShape) -> Result<Self, Self::Error> {
        if !Self::Node::is_compatible_arrow_shape(shape) {
            return Err(EdgeError::IncompatibleLeftArrowShape(shape));
        }
        self.left_arrow_shape = Some(shape);
        Ok(self)
    }
    fn right_arrow_shape(mut self, shape: ArrowShape) -> Result<Self, Self::Error> {
        if !Self::Node::is_compatible_arrow_shape(shape) {
            return Err(EdgeError::IncompatibleRightArrowShape(shape));
        }
        self.right_arrow_shape = Some(shape);
        Ok(self)
    }
}
