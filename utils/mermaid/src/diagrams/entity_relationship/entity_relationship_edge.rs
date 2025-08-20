//! Submodule defining an edge struct for entity-relationship diagrams in
//! Mermaid syntax.

use std::{fmt::Display, rc::Rc};

use crate::{
    diagrams::entity_relationship::entity_relationship_node::ERNode,
    shared::{ArrowShape, GenericEdge, LineStyle, NODE_LETTER, generic_edge::GenericEdgeBuilder},
    traits::{edge::Edge, edge_builder::EdgeBuilder, node::Node},
};
/// Type alias for an entity-relationship edge builder.
pub type EREdgeBuilder = GenericEdgeBuilder<ERNode>;
/// Type alias for an entity-relationship edge.
pub type EREdge = GenericEdge<ERNode>;

impl EREdgeBuilder {
    /// Creates a new entity-relationship edge builder.
    ///
    /// # Arguments
    ///
    /// * `source` - The source node of the edge.
    /// * `destination` - The destination node of the edge.
    pub fn zero_or_one(source: Rc<ERNode>, destination: Rc<ERNode>) -> Self {
        Self::default()
            .source(source)
            .unwrap()
            .destination(destination)
            .unwrap()
            .left_arrow_shape(ArrowShape::ZeroOrOne)
            .unwrap()
            .right_arrow_shape(ArrowShape::ZeroOrOne)
            .unwrap()
            .line_style(LineStyle::Solid)
    }

    /// Creates a new entity-relationship edge builder with a one-to-one
    /// relationship.
    ///
    /// # Arguments
    ///
    /// * `source` - The source node of the edge.
    /// * `destination` - The destination node of the edge.
    pub fn one_to_one(source: Rc<ERNode>, destination: Rc<ERNode>) -> Self {
        Self::default()
            .source(source)
            .unwrap()
            .destination(destination)
            .unwrap()
            .left_arrow_shape(ArrowShape::ExactlyOne)
            .unwrap()
            .right_arrow_shape(ArrowShape::ExactlyOne)
            .unwrap()
            .line_style(LineStyle::Solid)
    }

    /// Creates a new entity-relationship edge builder with a zero-or-one
    /// relationship.
    ///
    /// # Arguments
    ///
    /// * `source` - The source node of the edge.
    /// * `destination` - The destination node of the edge.
    pub fn zero_or_more(source: Rc<ERNode>, destination: Rc<ERNode>) -> Self {
        Self::default()
            .source(source)
            .unwrap()
            .destination(destination)
            .unwrap()
            .left_arrow_shape(ArrowShape::ZeroOrMore)
            .unwrap()
            .right_arrow_shape(ArrowShape::ZeroOrMore)
            .unwrap()
            .line_style(LineStyle::Solid)
    }

    /// Creates a new entity-relationship edge builder with a one-or-more
    /// relationship.
    ///
    /// # Arguments
    ///
    /// * `source` - The source node of the edge.
    /// * `destination` - The destination node of the edge.
    pub fn one_or_more(source: Rc<ERNode>, destination: Rc<ERNode>) -> Self {
        Self::default()
            .source(source)
            .unwrap()
            .destination(destination)
            .unwrap()
            .left_arrow_shape(ArrowShape::OneOrMore)
            .unwrap()
            .right_arrow_shape(ArrowShape::OneOrMore)
            .unwrap()
            .line_style(LineStyle::Solid)
    }
}

impl Display for EREdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{NODE_LETTER}{} {left_arrow}{segment}{right_arrow} {NODE_LETTER}{}{}",
            self.source().id(),
            self.destination().id(),
            self.label().map_or_else(String::new, |label| format!(" : \"{label}\"")),
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
