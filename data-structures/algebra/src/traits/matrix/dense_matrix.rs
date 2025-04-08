//! Submodule defining traits to operate over Dense matrices.

use super::{Matrix, ValuedMatrix};

/// Trait defining a dense matrix.
pub trait DenseMatrix: Matrix + ValuedMatrix {}

impl<M: DenseMatrix> DenseMatrix for &M {}
