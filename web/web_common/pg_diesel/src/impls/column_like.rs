//! Implementation of [`ColumnLike`] for [`Column`].
//!
//! This module implements the [`ColumnLike`](sql_traits::prelude::ColumnLike)
//! trait for the [`Column`] model from `information_schema.columns`, enabling
//! generic introspection of table columns.
//!
//! The implementation provides access to:
//! - Column name, data type, and nullability
//! - Whether the column is generated or has a default value
//! - The owning table (via the database metadata)
//! - Associated documentation from `pg_catalog.pg_description`

use sql_traits::traits::{ColumnLike, Metadata};

use crate::{PgDatabase, model_metadata::ColumnMetadata};

impl Metadata for crate::models::Column {
    type Meta = ColumnMetadata;
}

impl ColumnLike for crate::models::Column {
    type DB = PgDatabase;

    fn column_name(&self) -> &str {
        &self.column_name
    }

    fn column_doc<'db>(&'db self, database: &'db Self::DB) -> Option<&'db str>
    where
        Self: 'db,
    {
        database.column_metadata(self).description().map(|desc| desc.description.as_str())
    }

    fn table<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> &'db <Self::DB as sql_traits::traits::DatabaseLike>::Table
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

    fn normalized_data_type(&self, database: &Self::DB) -> String {
        database.column_metadata(self).normalized_data_type()
    }

    fn is_nullable(&self) -> bool {
        self.__is_nullable == "YES"
    }

    fn has_default(&self) -> bool {
        self.column_default.is_some()
    }
}
