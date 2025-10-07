//! Submodule implementing the [`ColumnLike`](sql_traits::prelude::ColumnLike)
//! trait for the [`Column`] struct.

use sql_traits::traits::ColumnLike;

impl ColumnLike for crate::models::Column {
    fn column_name(&self) -> &str {
        &self.column_name
    }
}
