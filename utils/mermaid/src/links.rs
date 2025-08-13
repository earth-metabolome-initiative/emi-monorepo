mod builder;
mod shape;
use std::fmt::Display;

pub(crate) use shape::LinkShape;

use super::colors::Colors;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a connection between two nodes in a Mermaid diagram.
pub struct Link {
    /// Unique identifier for the link.
    id: u32,
    /// The visual shape or style of the link.
    shape: LinkShape,
    /// Optional text label displayed on the link.
    label: Option<String>,
    /// Configuration options for the link's appearance and behavior.
    config: LinkConfig,
    /// Identifier of the node where the link starts.
    origin_node_id: u32,
    /// Identifier of the node where the link ends.
    destination_node_id: u32,
}

/// Represents the color and the line width of a link in the Mermaid diagram.
#[derive(Clone, Debug)]
pub(crate) struct LinkConfig {
    id: u32,
    stroke_color: Colors,
    stroke_width: u8,
}

impl Link {
    pub fn get_config(&self) -> &LinkConfig {
        &self.config
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.label {
            Some(label) => write!(f, "{}|{}|", self.shape, label),
            None => write!(f, "{}", self.shape),
        }
    }
}

impl Display for LinkConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "linkStyle {} stroke:{},stroke-width:{}px",
            self.id, self.stroke_color, self.stroke_width
        )
    }
}
