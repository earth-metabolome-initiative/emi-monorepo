use std::fmt::Display;

use crate::{links::Link, nodes::Node};

#[derive(Debug, Clone, Default)]
pub(crate) struct Pair {
    origin_node: Node,
    destination_node: Node,
    link: Link,
}

impl Pair {
    pub fn new(origin_node: Node, destination_node: Node, link: Link) -> Self {
        Pair { origin_node, destination_node, link }
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {} {}",
            self.origin_node.get_id(),
            self.link,
            self.destination_node.get_id(),
        )?;
        writeln!(f, "{}", self.origin_node)?;
        writeln!(f, "{}", self.destination_node)?;
        write!(f, "{}", self.link.get_config())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{colors::Colors, links::LinkShape, nodes::NodeShape};

    #[test]
    fn test_pair_display() {
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
        let link = Link::new(0, LinkShape::Dotted, Some("hola".to_string()), Colors::Red, 2);

        let pair = Pair::new(origin_node, destination_node, link);
        println!("{}", pair);
        assert_eq!(
            format!("{}", pair),
            "0 -.->|hola| 1\n0@{label: \"Start\", shape: rect}\nstyle 0 fill:#000000,stroke:#0000ff,stroke-width:2px,color:#ffffff\n1@{label: \"End\", shape: circle}\nstyle 1 fill:#000000,stroke:#008000,stroke-width:2px,color:#ffffff\nlinkStyle 0 stroke:#ff0000,stroke-width:2px\n"
        );
    }
}
