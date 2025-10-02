//! Submodule implementing the [`ColumnLike`] trait for `sqlparser`'s
//! [`ColumnDef`](sqlparser::ast::ColumnDef) struct.

use sqlparser::ast::ColumnDef;

use crate::traits::ColumnLike;

impl ColumnLike for ColumnDef {
    fn column_name(&self) -> &str {
        self.name.value.as_str()
    }
}
