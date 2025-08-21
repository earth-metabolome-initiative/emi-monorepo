//! Submodule providing an enumeration of possible errors that can occur in the
//! edges of diagrams in Mermaid syntax.

use std::fmt::{Debug, Display};

use common_traits::prelude::BuilderError;

use crate::{
    diagrams::{class_diagram::ClassEdgeAttribute, flowchart::FlowchartEdgeAttribute},
    shared::{ArrowShape, generic_edge::GenericEdgeAttribute},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Enum representing errors related to edges in Mermaid diagrams.
pub enum EdgeError<EdgeAttr> {
    /// The provided edge label is empty.
    EmptyLabel,
    /// The provided left arrow shape is not compatible with the diagram.
    IncompatibleLeftArrowShape(ArrowShape),
    /// The provided right arrow shape is not compatible with the diagram.
    IncompatibleRightArrowShape(ArrowShape),
    /// The provided source node does not exist in the diagram.
    SourceNodeNotFound(String),
    /// The provided destination node does not exist in the diagram.
    DestinationNodeNotFound(String),
    /// An error occurred while building the edge.
    Builder(BuilderError<EdgeAttr>),
}

impl<EdgeAttr> From<BuilderError<EdgeAttr>> for EdgeError<EdgeAttr> {
    fn from(error: BuilderError<EdgeAttr>) -> Self {
        EdgeError::Builder(error)
    }
}

impl From<EdgeError<GenericEdgeAttribute>> for EdgeError<FlowchartEdgeAttribute> {
    fn from(error: EdgeError<GenericEdgeAttribute>) -> Self {
        match error {
            EdgeError::EmptyLabel => EdgeError::EmptyLabel,
            EdgeError::IncompatibleLeftArrowShape(shape) => {
                EdgeError::IncompatibleLeftArrowShape(shape)
            }
            EdgeError::IncompatibleRightArrowShape(shape) => {
                EdgeError::IncompatibleRightArrowShape(shape)
            }
            EdgeError::SourceNodeNotFound(node) => EdgeError::SourceNodeNotFound(node),
            EdgeError::DestinationNodeNotFound(node) => EdgeError::DestinationNodeNotFound(node),
            EdgeError::Builder(builder_error) => {
                EdgeError::Builder(builder_error.into_field_name(From::from))
            }
        }
    }
}

impl From<EdgeError<GenericEdgeAttribute>> for EdgeError<ClassEdgeAttribute> {
    fn from(error: EdgeError<GenericEdgeAttribute>) -> Self {
        match error {
            EdgeError::EmptyLabel => EdgeError::EmptyLabel,
            EdgeError::IncompatibleLeftArrowShape(shape) => {
                EdgeError::IncompatibleLeftArrowShape(shape)
            }
            EdgeError::IncompatibleRightArrowShape(shape) => {
                EdgeError::IncompatibleRightArrowShape(shape)
            }
            EdgeError::SourceNodeNotFound(node) => EdgeError::SourceNodeNotFound(node),
            EdgeError::DestinationNodeNotFound(node) => EdgeError::DestinationNodeNotFound(node),
            EdgeError::Builder(builder_error) => {
                EdgeError::Builder(builder_error.into_field_name(From::from))
            }
        }
    }
}

impl<EdgeAttr: Display> std::fmt::Display for EdgeError<EdgeAttr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgeError::EmptyLabel => write!(f, "Edge label cannot be empty."),
            EdgeError::IncompatibleLeftArrowShape(shape) => {
                write!(f, "Incompatible left arrow shape: `{}`", shape.left())
            }
            EdgeError::IncompatibleRightArrowShape(shape) => {
                write!(f, "Incompatible right arrow shape: `{}`", shape.right())
            }
            EdgeError::SourceNodeNotFound(node) => {
                write!(f, "Source node not found: `{node}`",)
            }
            EdgeError::DestinationNodeNotFound(node) => {
                write!(f, "Destination node not found: `{node}`",)
            }
            EdgeError::Builder(error) => write!(f, "Builder error: `{error}`"),
        }
    }
}

impl<EdgeAttr: Debug + Display + 'static> core::error::Error for EdgeError<EdgeAttr> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            EdgeError::Builder(error) => Some(error),
            _ => None,
        }
    }
}
