use std::{fmt::Display, io::Write};

use crate::pairs::Pair;

enum GraphDirection {
    TopToBottom,
    LeftToRight,
}

struct MermaidGraph {
    pairs: Vec<Pair>,
    direction: GraphDirection,
}

impl MermaidGraph {
    pub fn new(direction: GraphDirection) -> Self {
        MermaidGraph { pairs: Vec::new(), direction }
    }

    pub fn add_pair(&mut self, pair: Pair) {
        self.pairs.push(pair);
    }

    pub fn to_file(&self, file_path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(file_path)?;
        // we add the quotes for the markdown file
        writeln!(file, "```mermaid")?;
        write!(file, "{self}")?;
        write!(file, "```")?;
        Ok(())
    }
}

impl Display for MermaidGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.direction {
            GraphDirection::TopToBottom => writeln!(f, "flowchart TD"),
            GraphDirection::LeftToRight => writeln!(f, "flowchart LR"),
        }?;
        for pair in &self.pairs {
            writeln!(f, "{pair}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        colors::Colors,
        links::{Link, LinkShape},
        nodes::{Node, NodeShape},
    };

    #[test]
    fn test_add_two_nodes_with_two_edges() {
        let mut graph = MermaidGraph::new(GraphDirection::TopToBottom);
        let origin_node = Node::new(
            0,
            NodeShape::Rectangle,
            "Start".to_string(),
            Colors::Blue,
            Colors::White,
            Colors::Black,
            2,
        );
        let destination_node = Node::new(
            1,
            NodeShape::Circle,
            "End".to_string(),
            Colors::Green,
            Colors::White,
            Colors::Black,
            2,
        );
        let link_0 = Link::new(0, LinkShape::Dotted, Some("hola".to_string()), Colors::Red, 2);
        let link_1 = Link::new(1, LinkShape::ArrowHead, Some("adios".to_string()), Colors::Blue, 2);
        let pair_0 = Pair::new(origin_node.clone(), destination_node.clone(), link_0);
        let pair_1 = Pair::new(origin_node.clone(), destination_node.clone(), link_1);

        graph.add_pair(pair_0);
        graph.add_pair(pair_1);

        assert_eq!(
            format!("{}", graph),
            "flowchart TD\n0 -->|hola| 1\n0@{label: \"Start\", shape: rect}\n1@{label: \"End\", shape: circle}\nlinkStyle 0 stroke:red,stroke-width:2px\n"
        );
    }
}
