//! Submodule defining the trait for Options for building a edges.

use algebra::prelude::SparseMatrixMut;
use common_traits::prelude::Builder;

use crate::traits::{Edges, GrowableEdges};

#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Options for building a edges.
pub enum EdgesBuilderOptions {
    /// The source of the edges.
    Edges,
}

impl core::fmt::Display for EdgesBuilderOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            EdgesBuilderOptions::Edges => write!(f, "edges"),
        }
    }
}

/// Trait for Options for building edges.
pub trait EdgesBuilder:
    Builder<
        Object = <Self as EdgesBuilder>::Edges,
        Error = crate::errors::builder::edges::EdgesBuilderError<<Self as EdgesBuilder>::Edges>,
        Attribute = EdgesBuilderOptions,
    >
{
    /// The type of the edges being built.
    type Edges: Edges;
    /// The intermediate type of the edges, if any.
    type IntermediateEdges: GrowableEdges;
    /// The iterator of edges.
    type EdgeIterator: IntoIterator<Item = <Self::Edges as Edges>::Edge>;

    #[must_use]
    /// Set whether to ignore duplicated edges.
    fn ignore_duplicates(self) -> Self;

    /// Returns whether to ignore duplicated edges.
    fn should_ignore_duplicates(&self) -> bool;

    #[must_use]
    /// Set the expected number of edges.
    ///
    /// # Arguments
    ///
    /// * `number_of_edges` - The expected number of edges.
    fn expected_number_of_edges(self, number_of_edges: <Self::Edges as Edges>::EdgeId) -> Self;

    /// Returns the expected number of edges.
    fn get_expected_number_of_edges(&self) -> Option<<Self::Edges as Edges>::EdgeId>;

    #[must_use]
    /// Set the expected shape of the graph.
    ///
    /// # Arguments
    ///
    /// * `shape` - The expected shape of the graph.
    fn expected_shape(
        self,
        shape: <<Self::IntermediateEdges as GrowableEdges>::GrowableMatrix as SparseMatrixMut>::MinimalShape,
    ) -> Self;

    /// Returns the expected shape of the graph.
    fn get_expected_shape(&self) -> Option<<<Self::IntermediateEdges as GrowableEdges>::GrowableMatrix as SparseMatrixMut>::MinimalShape>;

    #[must_use]
    /// Set the iterator of edges.
    ///
    /// # Arguments
    ///
    /// * `edges` - The iterator of edges.
    fn edges(self, edges: Self::EdgeIterator) -> Self;
}
