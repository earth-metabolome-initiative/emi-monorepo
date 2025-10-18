//! Submodule implementing the [`ColumnLike`](sql_traits::prelude::ColumnLike)
//! trait for the [`Column`] struct.

use sql_traits::traits::{ColumnLike, Metadata};

use crate::{
    PgDatabase,
    model_metadata::ColumnMetadata,
    models::{KeyColumnUsage, Table},
};

impl Metadata for crate::models::Column {
    type Meta = ColumnMetadata;
}

impl ColumnLike for crate::models::Column {
    type Database = PgDatabase;
    type ForeignKey = KeyColumnUsage;
    type Table = Table;

    fn column_name(&self) -> &str {
        &self.column_name
    }

    fn column_doc<'db>(&'db self, database: &'db Self::Database) -> Option<&'db str>
    where
        Self: 'db,
    {
        database.column_metadata(self).description().and_then(|desc| desc.description.as_deref())
    }

    fn table<'db>(&'db self, database: &'db Self::Database) -> &'db Self::Table
    where
        Self: 'db,
    {
        database.column_metadata(self).table()
    }

    fn is_generated(&self) -> bool {
        self.is_generated == "ALWAYS"
            || self.column_default.as_ref().is_some_and(|d| d.starts_with("nextval"))
            || self.is_identity.as_ref().is_some_and(|i| i == "YES")
    }

    fn data_type(&self) -> String {
        self.data_type_str().to_owned()
    }

    fn normalized_data_type(&self) -> String {
        self.data_type_str().to_owned()
    }

    fn is_nullable(&self) -> bool {
        self.__is_nullable == "YES"
    }

    fn has_default(&self) -> bool {
        self.column_default.is_some()
    }
}
