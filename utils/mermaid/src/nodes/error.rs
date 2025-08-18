//! Submodule defining errors pertaining to nodes in Mermaid diagrams.

use crate::shared::StyleClass;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NodeError<Node> {
    DuplicateNode(Node),
}
