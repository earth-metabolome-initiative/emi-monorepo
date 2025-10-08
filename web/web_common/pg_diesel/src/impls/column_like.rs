//! Submodule implementing the [`ColumnLike`](sql_traits::prelude::ColumnLike)
//! trait for the [`Column`] struct.

use sql_traits::traits::ColumnLike;

impl ColumnLike for crate::models::Column {
    fn column_name(&self) -> &str {
        &self.column_name
    }

    fn data_type(&self) -> String {
        self.data_type_str().to_owned()
    }

    fn is_nullable(&self) -> bool {
        self.__is_nullable == "YES"
    }
}
