mod builder;
mod error;
mod shape;
use std::{fmt::Display, rc::Rc};

pub use error::NodeError;
pub use shape::NodeShape;

use crate::shared::{label::Label, style_class::StyleClass};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a node in a Mermaid diagram.
pub(crate) struct Node {
    /// Unique identifier for the node.
    id: u32,
    /// The visual shape of the node (e.g., rectangle, circle, etc.).
    shape: NodeShape,
    /// The text label displayed inside the node.
    pub(crate) label: Label,
    /// The style classes applied to the node.
    style_classes: Vec<Rc<StyleClass>>,
}
