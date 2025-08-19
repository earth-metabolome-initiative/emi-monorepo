//! Submodule defining an edge struct for entity-relationship diagrams in
//! Mermaid syntax.

use crate::traits::Edge;

mod builder;
pub struct EREdge {}

impl Edge for EREdge {}
