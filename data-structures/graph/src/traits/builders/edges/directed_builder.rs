//! Submodule defining the trait for a directed edges builder.

use crate::traits::DirectedEdges;

use super::EdgesBuilder;

/// Trait for Options for building edges.
pub trait DirectedEdgesBuilder:
    EdgesBuilder<Edges = <Self as DirectedEdgesBuilder>::DirectedEdges>
{
    /// The type of the edges being built.
    type DirectedEdges: DirectedEdges;

    /// Set whether to ignore self-loops.
    fn ignore_self_loops(&mut self) -> &mut Self;

    /// Returns whether to ignore self-loops.
    fn should_ignore_self_loops(&self) -> bool;
}
