//! A generic edges builder that can be used to build a edges for any type of
//! graph.

use algebra::{impls::MutabilityError, prelude::SparseMatrixMut};

use crate::{
    errors::builder::edges::EdgesBuilderError,
    traits::{Edges, EdgesBuilder, GrowableEdges},
};

/// A generic edges builder that can be used to build a edges for any type of
/// graph.
pub struct GenericEdgesBuilder<EdgeIterator, GE: GrowableEdges> {
    /// The edges to build the edges from.
    pub(super) edges: Option<EdgeIterator>,
    /// The expected number of edges.
    expected_number_of_edges: Option<GE::EdgeId>,
    /// The expected shape of the graph.
    expected_shape: Option<<GE::GrowableMatrix as SparseMatrixMut>::MinimalShape>,
    /// Whether to ignore duplicated edges.
    ignore_duplicates: bool,
    /// The edges type.
    _edges: core::marker::PhantomData<GE>,
}

impl<EdgeIterator, GE: GrowableEdges> Default for GenericEdgesBuilder<EdgeIterator, GE> {
    fn default() -> Self {
        Self {
            edges: None,
            expected_number_of_edges: None,
            expected_shape: None,
            ignore_duplicates: false,
            _edges: core::marker::PhantomData,
        }
    }
}

impl<EdgeIterator, GE: GrowableEdges> EdgesBuilder for GenericEdgesBuilder<EdgeIterator, GE>
where
    GE: GrowableEdges<Error = EdgesBuilderError<GE>>,
    EdgeIterator: IntoIterator<Item = GE::Edge>,
{
    type EdgeIterator = EdgeIterator;
    type IntermediateEdges = GE;
    type Edges = GE;

    fn expected_number_of_edges(mut self, number_of_edges: GE::EdgeId) -> Self {
        self.expected_number_of_edges = Some(number_of_edges);
        self
    }

    fn get_expected_number_of_edges(&self) -> Option<GE::EdgeId> {
        self.expected_number_of_edges
    }

    fn ignore_duplicates(mut self) -> Self {
        self.ignore_duplicates = true;
        self
    }

    fn should_ignore_duplicates(&self) -> bool {
        self.ignore_duplicates
    }

    fn expected_shape(
        mut self,
        shape: <GE::GrowableMatrix as SparseMatrixMut>::MinimalShape,
    ) -> Self {
        self.expected_shape = Some(shape);
        self
    }

    fn get_expected_shape(
        &self,
    ) -> Option<<<GE as GrowableEdges>::GrowableMatrix as SparseMatrixMut>::MinimalShape> {
        self.expected_shape
    }

    fn edges(mut self, edges: Self::EdgeIterator) -> Self {
        self.edges = Some(edges);
        self
    }
}

impl<EdgeIterator, GE> GenericEdgesBuilder<EdgeIterator, GE>
where
    GE: GrowableEdges<Error = EdgesBuilderError<GE>>,
    GenericEdgesBuilder<EdgeIterator, GE>:
        EdgesBuilder<EdgeIterator = EdgeIterator, Edges = GE, IntermediateEdges = GE>,
    EdgeIterator: IntoIterator<
        Item = <<GenericEdgesBuilder<EdgeIterator, GE> as EdgesBuilder>::Edges as Edges>::Edge,
    >,
{
    /// Builds the edges.
    ///
    /// # Errors
    ///
    /// * If any edge is invalid.
    pub fn build(self: GenericEdgesBuilder<EdgeIterator, GE>) -> Result<GE, EdgesBuilderError<GE>> {
        let expected_number_of_edges = self.get_expected_number_of_edges();
        let mut edges = match (expected_number_of_edges, self.get_expected_shape()) {
            (Some(number_of_edges), Some(shape)) => {
                GE::with_shaped_capacity(shape, number_of_edges)
            }
            (Some(number_of_edges), None) => GE::with_capacity(number_of_edges),
            (None, Some(shape)) => GE::with_shape(shape),
            (None, None) => Default::default(),
        };
        let should_ignore_duplicates = self.should_ignore_duplicates();
        self.edges.ok_or(EdgesBuilderError::MissingAttribute("edges"))?.into_iter().try_for_each(
            |edge| {
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
            },
        )?;

        if let Some(expected_number_of_edges) = expected_number_of_edges
            && edges.number_of_edges() != expected_number_of_edges
        {
            return Err(crate::errors::builder::edges::EdgesBuilderError::NumberOfEdges {
                expected: expected_number_of_edges,
                actual: edges.number_of_edges(),
            });
        }

        Ok(edges)
    }
}
