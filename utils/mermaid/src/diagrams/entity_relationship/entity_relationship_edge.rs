//! Submodule defining an edge struct for entity-relationship diagrams in
//! Mermaid syntax.

use std::fmt::Display;

use crate::{
    diagrams::entity_relationship::entity_relationship_node::ERNode,
    shared::{GenericEdge, LineStyle, NODE_LETTER},
    traits::{edge::Edge, node::Node},
};

/// Type alias for an entity-relationship edge builder.
pub type EREdgeBuilder = GenericEdge<ERNode>;
/// Type alias for an entity-relationship edge.
pub type EREdge = GenericEdge<ERNode>;

impl Display for EREdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{NODE_LETTER}{} {left_arrow}{segment}{right_arrow} {NODE_LETTER}{}{}",
            self.source().id(),
            self.destination().id(),
            self.label().map_or_else(String::new, |label| format!(" : \"`{label}`\"")),
            left_arrow = self.left_arrow_shape().as_ref().map_or_else(|| "", |shape| shape.left()),
            segment = match self.line_style() {
                LineStyle::Solid => "--",
                LineStyle::Thick => "==",
                LineStyle::Dashed => "..",
            },
            right_arrow =
                self.right_arrow_shape().as_ref().map_or_else(|| "", |shape| shape.right()),
        )
    }
}
