//! A generic edges builder that can be used to build a edges for any type of
//! graph.

use std::marker::PhantomData;

use algebra::prelude::{SparseMatrixMut, Symmetrize};

use super::GenericEdgesBuilder;
use crate::{
    errors::builder::edges::EdgesBuilderError,
    traits::{
        DirectedEdges, Edges, EdgesBuilder, FromDirectedMonopartiteEdges, GrowableEdges,
        MonopartiteEdges, UndirectedMonopartiteEdges,
    },
};

/// A generic edges builder that can be used to build a edges for any type of
/// graph.
pub struct GenericUndirectedMonopartiteEdgesBuilder<EdgeIterator, GE: GrowableEdges, UE> {
    /// The inner builder for the edges.
    builder: GenericEdgesBuilder<EdgeIterator, GE>,
    /// The actual undirected graph that will be built.
    _undirected_edges: core::marker::PhantomData<UE>,
}

impl<EdgeIterator, GE: GrowableEdges, UE> Default
    for GenericUndirectedMonopartiteEdgesBuilder<EdgeIterator, GE, UE>
{
    fn default() -> Self {
        Self { builder: GenericEdgesBuilder::default(), _undirected_edges: PhantomData }
    }
}

impl<EdgeIterator, GE, UE> EdgesBuilder
    for GenericUndirectedMonopartiteEdgesBuilder<EdgeIterator, GE, UE>
where
    UE: UndirectedMonopartiteEdges<Edge = GE::Edge, EdgeId = GE::EdgeId>
        + FromDirectedMonopartiteEdges<GE>,
    GE: GrowableEdges<Error = EdgesBuilderError<GE>>
        + MonopartiteEdges<MonopartiteMatrix = GE::GrowableMatrix>
        + DirectedEdges<Matrix = GE::GrowableMatrix>,
    EdgeIterator: IntoIterator<Item = GE::Edge>,
    EdgesBuilderError<UE>: From<EdgesBuilderError<GE>>,
    <GE as GrowableEdges>::GrowableMatrix:
        Symmetrize<<UE as UndirectedMonopartiteEdges>::SymmetricSquaredMatrix>,
{
    type EdgeIterator = EdgeIterator;
    type IntermediateEdges = GE;
    type Edges = UE;

    fn expected_number_of_edges(mut self, number_of_edges: GE::EdgeId) -> Self {
        self.builder = self.builder.expected_number_of_edges(number_of_edges);
        self
    }

    fn get_expected_number_of_edges(&self) -> Option<GE::EdgeId> {
        self.builder.get_expected_number_of_edges()
    }

    fn ignore_duplicates(mut self) -> Self {
        self.builder = self.builder.ignore_duplicates();
        self
    }

    fn should_ignore_duplicates(&self) -> bool {
        self.builder.should_ignore_duplicates()
    }

    fn expected_shape(
        mut self,
        shape: <<Self::IntermediateEdges as GrowableEdges>::GrowableMatrix as SparseMatrixMut>::MinimalShape,
    ) -> Self {
        self.builder = self.builder.expected_shape(shape);
        self
    }

    fn get_expected_shape(&self) -> Option<<<Self::IntermediateEdges as GrowableEdges>::GrowableMatrix as SparseMatrixMut>::MinimalShape>{
        self.builder.get_expected_shape()
    }

    fn edges(mut self, edges: Self::EdgeIterator) -> Self {
        self.builder = self.builder.edges(edges);
        self
    }
}

impl<EdgeIterator, GE, UE> GenericUndirectedMonopartiteEdgesBuilder<EdgeIterator, GE, UE>
where
    UE: UndirectedMonopartiteEdges<Edge = GE::Edge, EdgeId = GE::EdgeId>
        + FromDirectedMonopartiteEdges<GE>,
    GE: GrowableEdges<Error = EdgesBuilderError<GE>>
        + MonopartiteEdges<MonopartiteMatrix = GE::GrowableMatrix>
        + DirectedEdges<Matrix = GE::GrowableMatrix>,
    Self: EdgesBuilder<EdgeIterator = EdgeIterator, Edges = UE, IntermediateEdges = GE>,
    EdgeIterator: IntoIterator<Item = <<Self as EdgesBuilder>::Edges as Edges>::Edge>,
    EdgesBuilderError<UE>: From<EdgesBuilderError<GE>>,
    <GE as GrowableEdges>::GrowableMatrix:
        Symmetrize<<UE as UndirectedMonopartiteEdges>::SymmetricSquaredMatrix>,
{
    /// Builds the undirected edges.
    pub fn build(self) -> Result<UE, EdgesBuilderError<UE>> {
        let directed_edges: GE = self.builder.build()?;
        let undirected_edges: UE = UE::from_directed_edges(directed_edges);
        Ok(undirected_edges)
    }
}
