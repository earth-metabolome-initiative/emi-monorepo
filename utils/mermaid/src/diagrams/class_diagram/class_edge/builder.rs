//! Submodule providing a builder for class edges in Mermaid class diagrams.

use crate::{
    diagrams::class_diagram::{class_edge::ClassEdge, class_node::ClassNode},
    traits::EdgeBuilder,
};

pub struct ClassEdgeBuilder {}

impl EdgeBuilder for ClassEdgeBuilder {
    type Edge = ClassEdge;
    type Node = ClassNode;
}
