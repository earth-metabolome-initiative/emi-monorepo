//! Submodule defining a node which may be used in a flowchart diagram
//! in Mermaid syntax.

use crate::nodes::Node;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a node in a flowchart diagram, which can have various
/// properties and may include click events.
pub struct FlowchartNode {
    /// Underlying properties of a diagram node.
    node: Node,
}
