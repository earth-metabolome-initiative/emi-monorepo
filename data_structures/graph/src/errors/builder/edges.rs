//! Error enumeration for the edges builder.

use algebra::{
    impls::{MutabilityError, SymmetricCSR2D, UpperTriangularCSR2D},
    prelude::{Matrix2D, SizedSparseMatrix2D},
};
use numeric_common_traits::prelude::TryFromUsize;

use crate::traits::Edges;

#[derive(Debug, thiserror::Error)]
/// Enum representing the possible errors that can occur when building a graph.
pub enum EdgesBuilderError<E: Edges> {
    #[error("Missing builder attribute: {0}")]
    /// An attribute was not set in the builder.
    MissingAttribute(&'static str),
    #[error("Unexpected number of edges: expected {expected}, got {actual}")]
    /// Whether the expected number of edges was not reached or it was
    /// overreached.
    NumberOfEdges {
        /// The expected number of edges.
        expected: E::EdgeId,
        /// The actual number of edges.
        actual: E::EdgeId,
    },
    #[error("Matrix error: {0}")]
    /// An error occurred while building the underlying matrix.
    MatrixError(#[from] MutabilityError<E::Matrix>),
}

impl<M> From<EdgesBuilderError<UpperTriangularCSR2D<M>>> for EdgesBuilderError<SymmetricCSR2D<M>>
where
    M: SizedSparseMatrix2D<ColumnIndex = <M as Matrix2D>::RowIndex>,
    M::RowIndex: TryFromUsize,
    M::SparseIndex: TryFromUsize,
{
    fn from(e: EdgesBuilderError<UpperTriangularCSR2D<M>>) -> Self {
        match e {
            EdgesBuilderError::MissingAttribute(attr) => EdgesBuilderError::MissingAttribute(attr),
            EdgesBuilderError::NumberOfEdges { expected, actual } => {
                EdgesBuilderError::NumberOfEdges { expected, actual }
            }
            EdgesBuilderError::MatrixError(e) => EdgesBuilderError::MatrixError(e.into()),
        }
    }
}
