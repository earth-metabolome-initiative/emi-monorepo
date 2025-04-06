//! Submodule defining traits to characterize a dense bi-dimensional matrix.

use super::{DenseMatrix, Matrix2D, ValuedMatrix2D};

/// Trait defining a dense bi-dimensional matrix.
pub trait DenseMatrix2D: DenseMatrix + ValuedMatrix2D + Matrix2D {}

impl<M: DenseMatrix2D> DenseMatrix2D for &M {}
