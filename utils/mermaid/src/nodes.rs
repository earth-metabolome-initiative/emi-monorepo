mod builder;
mod shape;
use std::fmt::Display;

pub(crate) use shape::NodeShape;

use crate::colors::Colors;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a node in a Mermaid diagram.
pub struct Node {
    /// Unique identifier for the node.
    id: u32,
    /// The visual shape of the node (e.g., rectangle, circle, etc.).
    shape: NodeShape,
    /// The text label displayed inside the node.
    label: String,
    /// The color of the node's border.
    stroke_color: Colors,
    /// The color of the node's label text.
    text_color: Colors,
    /// The background color of the node.
    fill_color: Colors,
    /// The thickness of the node's border.
    stroke_width: u8,
}

impl Node {
    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // First line is the node id with its label and shape e.g. 2@{label: "stop",
        // shape: circle}
        writeln!(f, "{}@{{label: \"{}\", shape: {}}}", self.id, self.label, self.shape)?;
        // Second line is the node styling : style 2
        // fill:#bbf,stroke:#ff0000,stroke-width:2px,color:#ff0000
        write!(
            f,
            "style {} fill:{},stroke:{},stroke-width:{}px,color:{}",
            self.id, self.fill_color, self.stroke_color, self.stroke_width, self.text_color
        )
    }
}

/// Unit tests for the Node struct
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_display_default() {
        let node = Node::default();
        let expected = "0@{label: \"hello\", shape: rect}\nstyle 0 fill:#ffffff,stroke:#000000,stroke-width:2px,color:#000000";
        assert_eq!(format!("{}", node), expected);
    }
}
