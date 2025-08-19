//! Submodule defining a builder struct to construct entity-relationship edges
//! in the entity-relationship diagram in Mermaid syntax.

use crate::traits::EdgeBuilder;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EREdgeBuilder {
    // Fields for the builder can be defined here
}

impl EdgeBuilder for EREdgeBuilder {}
