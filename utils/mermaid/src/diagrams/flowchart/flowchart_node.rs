//! Submodule defining a node which may be used in a flowchart diagram
//! in Mermaid syntax.

mod builder;
mod click_event;
pub use click_event::ClickEvent;

use crate::nodes::Node;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a node in a flowchart diagram, which can have various
/// properties and may include click events.
pub struct FlowchartNode {
    /// Underlying properties of a diagram node.
    node: Node,
    /// The click event associated with the node, if any.
    click_event: Option<ClickEvent>,
}
