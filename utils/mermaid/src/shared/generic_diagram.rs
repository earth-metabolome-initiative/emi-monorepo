//! Submodule defining a generic diagram struct which can be used as a base
//! for various types of diagrams in Mermaid syntax.

use std::rc::Rc;

use crate::shared::StyleClass;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A generic diagram struct that can be extended for specific diagram types.
pub struct GenericDiagram<Node, Edge, Config> {
    /// Style classes associated with this diagram.
    style_classes: Vec<Rc<StyleClass>>,
    /// Nodes in the diagram.
    nodes: Vec<Node>,
    /// Edges in the diagram.
    edges: Vec<Edge>,
    /// Configuration options for the diagram.
    configuration: Config,
}
