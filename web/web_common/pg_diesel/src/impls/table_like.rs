//! Submodule implementing the [`TableLike`](sql_traits::prelude::TableLike)
//! trait for the [`Table`] struct.

use sql_traits::traits::TableLike;

impl TableLike for crate::models::Table {
    fn table_name(&self) -> &str {
        &self.table_name
    }
}
