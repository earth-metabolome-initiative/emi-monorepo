mod shape;
use std::fmt::Display;

pub(crate) use shape::NodeShape;

use crate::colors::Colors;

#[derive(Debug, Clone)]
pub(crate) struct Node {
    node_id: u32,
    shape: NodeShape,
    label: String,
    stroke_color: Colors,
    text_color: Colors,
    fill_color: Colors,
    stroke_width: u8,
}

impl Node {
    pub fn new(
        node_id: u32,
        shape: NodeShape,
        label: String,
        stroke_color: Colors,
        text_color: Colors,
        fill_color: Colors,
        stroke_width: u8,
    ) -> Self {
        Node { node_id, shape, label, stroke_color, text_color, fill_color, stroke_width }
    }

    pub fn get_id(&self) -> u32 {
        self.node_id
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            node_id: 0,
            shape: NodeShape::Rectangle,
            label: "hello".to_string(),
            stroke_color: Colors::Black,
            text_color: Colors::Black,
            fill_color: Colors::White,
            stroke_width: 2,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // First line is the node id with its label and shape e.g. 2@{label: "stop",
        // shape: circle}
        writeln!(
            f,
            "{}@{{label: \"{}\", shape: {}}}",
            self.node_id,
            self.label.to_string(),
            self.shape
        )?;
        // Second line is the node styling : style 2
        // fill:#bbf,stroke:#ff0000,stroke-width:2px,color:#ff0000
        write!(
            f,
            "style {} fill:{},stroke:{},stroke-width:{}px,color:{}",
            self.node_id, self.fill_color, self.stroke_color, self.stroke_width, self.text_color
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
