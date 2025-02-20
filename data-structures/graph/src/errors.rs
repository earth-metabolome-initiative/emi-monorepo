//! Submodule defining common errors for the graph crate.

pub mod nodes;

use nodes::NodeError;

use crate::traits::Graph;

/// Errors that may occur when working with graphs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error<G: Graph> {
    /// Error relative to nodes.
    NodeError(NodeError<G>),
}
