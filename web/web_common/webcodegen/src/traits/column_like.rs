//! Submodule defining the `ColumnLike` trait.

use crate::Column;

/// A trait representing types that can be treated as columns.
pub trait ColumnLike {}

impl<T> ColumnLike for T where T: AsRef<Column> {}
