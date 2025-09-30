//! Submodule implementing the [`ConstrainableColumn`] trait for `sqlparser`'s
//! [`ColumnDef`] struct.

use sqlparser::ast::ColumnDef;

use crate::traits::{Constrainable, ConstrainableColumn};

impl Constrainable for ColumnDef {
    fn context_name(&self) -> String {
        format!("`{}`", self.name)
    }
}

impl ConstrainableColumn for ColumnDef {
    fn column_name(&self) -> &str {
        self.name.value.as_str()
    }
}
