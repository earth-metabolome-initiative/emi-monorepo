mod arrow_shape;
mod builder;
mod line_style;
mod link_style;

use crate::{links::link_style::LinkStyle, shared::style_class::StyleClass};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a connection between two nodes in a Mermaid diagram.
pub struct Link {
    /// Unique identifier for the link.
    id: u32,
    /// The visual style of the link.
    style: LinkStyle,
    /// Optional text label displayed on the link.
    label: Option<String>,
    /// Identifier of the node where the link starts.
    origin_node_id: u32,
    /// Identifier of the node where the link ends.
    destination_node_id: u32,
    /// Classes applied to the link.
    style_classes: Vec<StyleClass>,
}
