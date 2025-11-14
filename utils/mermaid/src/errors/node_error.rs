//! Submodule providing an enumeration of possible errors that can occur in the
//! nodes of diagrams in Mermaid syntax.

use std::fmt::{Debug, Display};

use common_traits::prelude::BuilderError;

use crate::{
    diagrams::{
        class_diagram::ClassNodeAttribute, entity_relationship::ERNodeAttribute,
        flowchart::FlowchartNodeAttribute,
    },
    shared::generic_node::GenericNodeAttribute,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Enum representing errors related to nodes in Mermaid diagrams.
pub enum NodeError<NodeAttr> {
    /// The provided node label is empty.
    EmptyLabel,
    /// The provided node ID is empty.
    EmptyId,
    /// The provided node ID contains invalid characters.
    InvalidId(String),
    /// The provided node already exists in the diagram.
    DuplicateNode(String),
    /// An error occurred while building the node.
    Builder(BuilderError<NodeAttr>),
}

impl From<NodeError<GenericNodeAttribute>> for NodeError<FlowchartNodeAttribute> {
    fn from(error: NodeError<GenericNodeAttribute>) -> Self {
        match error {
            NodeError::EmptyLabel => NodeError::EmptyLabel,
            NodeError::EmptyId => NodeError::EmptyId,
            NodeError::InvalidId(id) => NodeError::InvalidId(id),
            NodeError::DuplicateNode(node) => NodeError::DuplicateNode(node),
            NodeError::Builder(builder_error) => {
                NodeError::Builder(builder_error.replace_field_name(From::from))
            }
        }
    }
}

impl From<NodeError<GenericNodeAttribute>> for NodeError<ERNodeAttribute> {
    fn from(error: NodeError<GenericNodeAttribute>) -> Self {
        match error {
            NodeError::EmptyLabel => NodeError::EmptyLabel,
            NodeError::EmptyId => NodeError::EmptyId,
            NodeError::InvalidId(id) => NodeError::InvalidId(id),
            NodeError::DuplicateNode(node) => NodeError::DuplicateNode(node),
            NodeError::Builder(builder_error) => {
                NodeError::Builder(builder_error.replace_field_name(From::from))
            }
        }
    }
}

impl From<NodeError<GenericNodeAttribute>> for NodeError<ClassNodeAttribute> {
    fn from(error: NodeError<GenericNodeAttribute>) -> Self {
        match error {
            NodeError::EmptyLabel => NodeError::EmptyLabel,
            NodeError::EmptyId => NodeError::EmptyId,
            NodeError::InvalidId(id) => NodeError::InvalidId(id),
            NodeError::DuplicateNode(node) => NodeError::DuplicateNode(node),
            NodeError::Builder(builder_error) => {
                NodeError::Builder(builder_error.replace_field_name(From::from))
            }
        }
    }
}

impl<NodeAttr> From<BuilderError<NodeAttr>> for NodeError<NodeAttr> {
    fn from(error: BuilderError<NodeAttr>) -> Self {
        NodeError::Builder(error)
    }
}

impl<NodeAttr: Display> std::fmt::Display for NodeError<NodeAttr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeError::EmptyLabel => write!(f, "Node label cannot be empty."),
            NodeError::EmptyId => write!(f, "Node ID cannot be empty."),
            NodeError::InvalidId(id) => write!(f, "Node ID `{id}` contains invalid characters."),
            NodeError::DuplicateNode(node) => {
                write!(f, "Node `{node}` already exists in the diagram.")
            }
            NodeError::Builder(error) => write!(f, "Builder error: `{error}`"),
        }
    }
}

impl<NodeAttr: Debug + Display + 'static> core::error::Error for NodeError<NodeAttr> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            NodeError::Builder(error) => Some(error),
            _ => None,
        }
    }
}
