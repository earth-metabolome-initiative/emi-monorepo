//! Submodule providing a generic node struct which may be reused across
//! different diagrams.

use std::{fmt::Display, rc::Rc};

use common_traits::prelude::{Builder, BuilderError};

use crate::{
    errors::EdgeError,
    shared::{ArrowShape, LineStyle},
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enumeration of attributes that can be set on a `GenericEdge`.
pub enum GenericEdgeAttribute {
    /// Label of the edge.
    Label,
    /// Source node of the edge.
    Source,
    /// Destination node of the edge.
    Destination,
    /// Line style of the edge.
    LineStyle,
    /// Left arrow shape of the edge, if any.
    LeftArrowShape,
    /// Right arrow shape of the edge, if any.
    RightArrowShape,
}

impl Display for GenericEdgeAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericEdgeAttribute::Label => write!(f, "label"),
            GenericEdgeAttribute::Source => write!(f, "source"),
            GenericEdgeAttribute::Destination => write!(f, "destination"),
            GenericEdgeAttribute::LineStyle => write!(f, "line_style"),
            GenericEdgeAttribute::LeftArrowShape => write!(f, "left_arrow_shape"),
            GenericEdgeAttribute::RightArrowShape => write!(f, "right_arrow_shape"),
        }
    }
}

impl<N> Builder for GenericEdgeBuilder<N> {
    type Attribute = GenericEdgeAttribute;
    type Error = EdgeError<Self::Attribute>;
    type Object = GenericEdge<N>;

    fn is_complete(&self) -> bool {
        self.source.is_some() && self.destination.is_some()
    }

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(GenericEdge {
            label: self.label,
            source: self
                .source
                .ok_or(BuilderError::IncompleteBuild(GenericEdgeAttribute::Source))?,
            destination: self
                .destination
                .ok_or(BuilderError::IncompleteBuild(GenericEdgeAttribute::Destination))?,
            line_style: self.line_style,
            left_arrow_shape: self.left_arrow_shape,
            right_arrow_shape: self.right_arrow_shape,
        })
    }
}

impl<N: Node> EdgeBuilder for GenericEdgeBuilder<N> {
    type Node = N;
    type Edge = GenericEdge<N>;

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
