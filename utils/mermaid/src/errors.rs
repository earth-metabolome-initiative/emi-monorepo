//! Submodule defining the possible errors that can occur in the Mermaid
//! library.

use crate::{nodes::NodeError, shared::style_class::StyleClassError};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing the different types of errors that can occur in the
/// Mermaid library.
pub enum Error<Node> {
    /// An error regarding nodes.
    Node(NodeError<Node>),
    /// An error regarding style classes.
    StyleClass(StyleClassError),
}
