//! A generic edges builder that can be used to build a edges for any type of graph.

use algebra::{impls::MutabilityError, prelude::SparseMatrixMut};
use common_traits::prelude::Builder;

use super::GenericEdgesBuilder;
use crate::{
    errors::builder::edges::EdgesBuilderError,
    traits::{DirectedEdges, Edge, Edges, EdgesBuilder, EdgesBuilderOptions, GrowableEdges},
};

/// A generic edges builder that can be used to build a edges for any type of graph.
pub struct GenericDirectedEdgesBuilder<EdgeIterator, GE: GrowableEdges> {
    /// The inner builder for the edges.
    builder: GenericEdgesBuilder<EdgeIterator, GE>,
    /// Whether to ignore self-loops.
    ignore_self_loops: bool,
}

impl<EdgeIterator, GE: GrowableEdges> Default for GenericDirectedEdgesBuilder<EdgeIterator, GE> {
    fn default() -> Self {
        Self { builder: GenericEdgesBuilder::default(), ignore_self_loops: false }
    }
}

impl<EdgeIterator, GE: GrowableEdges> EdgesBuilder for GenericDirectedEdgesBuilder<EdgeIterator, GE>
where
    GE: GrowableEdges<Error = EdgesBuilderError<GE>>
        + DirectedEdges<DirectedMatrix = GE::GrowableMatrix>,
    EdgeIterator: IntoIterator<Item = GE::Edge>,
{
    type EdgeIterator = EdgeIterator;
    type IntermediateEdges = GE;
    type Edges = GE;

    fn expected_number_of_edges(&mut self, number_of_edges: GE::EdgeId) -> &mut Self {
        self.builder.expected_number_of_edges(number_of_edges);
        self
    }

    fn get_expected_number_of_edges(&self) -> Option<GE::EdgeId> {
        self.builder.get_expected_number_of_edges()
    }

    fn ignore_duplicates(&mut self) -> &mut Self {
        self.builder.ignore_duplicates();
        self
    }

    fn should_ignore_duplicates(&self) -> bool {
        self.builder.should_ignore_duplicates()
    }

    fn expected_shape(
        &mut self,
        shape: <<Self::IntermediateEdges as GrowableEdges>::GrowableMatrix as SparseMatrixMut>::MinimalShape,
    ) -> &mut Self {
        self.builder.expected_shape(shape);
        self
    }

    fn get_expected_shape(&self) -> Option<<<Self::IntermediateEdges as GrowableEdges>::GrowableMatrix as SparseMatrixMut>::MinimalShape>{
        self.builder.get_expected_shape()
    }

    fn edges(&mut self, edges: Self::EdgeIterator) -> &mut Self {
        self.builder.edges(edges);
        self
    }
}

impl<EdgeIterator, GE: GrowableEdges> GenericDirectedEdgesBuilder<EdgeIterator, GE> {
    /// Set whether to ignore self-loops.
    pub fn ignore_self_loops(&mut self) -> &mut Self {
        self.ignore_self_loops = true;
        self
    }

    /// Returns whether to ignore self-loops.
    pub fn should_ignore_self_loops(&self) -> bool {
        self.ignore_self_loops
    }
}

impl<EdgeIterator, GE> Builder for GenericDirectedEdgesBuilder<EdgeIterator, GE>
where
    GE: GrowableEdges<Error = EdgesBuilderError<GE>>
        + DirectedEdges<DirectedMatrix = GE::GrowableMatrix>,
    Self: EdgesBuilder<EdgeIterator = EdgeIterator, Edges = GE, IntermediateEdges = GE>,
    EdgeIterator: IntoIterator<Item = <<Self as EdgesBuilder>::Edges as Edges>::Edge>,
{
    type Object = GE;
    type Error = EdgesBuilderError<GE>;
    type Attribute = EdgesBuilderOptions;

    fn build(self) -> Result<Self::Object, Self::Error> {
        let expected_number_of_edges = self.get_expected_number_of_edges();
        let mut edges = if let Some(number_of_edges) = expected_number_of_edges {
            if let Some(shape) = self.get_expected_shape() {
                GE::with_shaped_capacity(shape, number_of_edges)
            } else {
                GE::with_capacity(number_of_edges)
            }
        } else {
            Default::default()
        };
        let should_ignore_duplicates = self.should_ignore_duplicates();
        self.builder
            .edges
            .ok_or({
                common_traits::prelude::BuilderError::IncompleteBuild {
                    missing_attribute: EdgesBuilderOptions::Edges,
                }
            })?
            .into_iter()
            .try_for_each(|edge| {
                if self.ignore_self_loops && edge.is_self_loop() {
                    return Ok(());
                }
                if let Err(err) = edges.add(edge) {
                    match err {
                        crate::errors::builder::edges::EdgesBuilderError::MatrixError(
                            MutabilityError::DuplicatedEntry(_),
                        ) => {
                            if should_ignore_duplicates {
                                Ok(())
                            } else {
                                Err(err)
                            }
                        }
                        other => Err(other),
                    }
                } else {
                    Ok(())
                }
            })?;

        if let Some(expected_number_of_edges) = expected_number_of_edges {
            if edges.number_of_edges() != expected_number_of_edges {
                return Err(crate::errors::builder::edges::EdgesBuilderError::NumberOfEdges {
                    expected: expected_number_of_edges,
                    actual: edges.number_of_edges(),
                });
            }
        }

        Ok(edges)
    }
}
