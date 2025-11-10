//! Submodule defining the `ForeignKeySchema` trait which provides documentation
//! representation for foreign keys in the diesel schema.

use std::sync::Arc;

use sql_traits::traits::DatabaseLike;
use synql_core::{
    structs::{InternalCrate, Workspace},
    traits::ForeignKeySynLike,
};

use crate::traits::{ColumnSchema, TableSchema};

/// Trait to provide documentation representation for foreign keys in the diesel
/// schema.
pub trait ForeignKeySchema: ForeignKeySynLike + Sized {
    /// Returns the formatted human readable representation of the foreign key
    /// with documentation links to the column schemas.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query tables and
    ///   columns from.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use synql_diesel_schema::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (
    ///     host_id INT,
    ///     host_name TEXT,
    ///     FOREIGN KEY (host_id, host_name) REFERENCES referenced_table(id, name)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let foreign_key = host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let doc_repr = foreign_key.documentation_schema_repr(&db);
    /// assert_eq!(
    ///     doc_repr,
    ///     "[`host_id`](host_table_schema::schema::host_table::host_id) -> [`id`](referenced_table_schema::schema::referenced_table::id), [`host_name`](host_table_schema::schema::host_table::host_name) -> [`name`](referenced_table_schema::schema::referenced_table::name)"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn documentation_schema_repr(&self, database: &Self::DB) -> String
    where
        <Self::DB as DatabaseLike>::Table: TableSchema,
        <Self::DB as DatabaseLike>::Column: ColumnSchema,
    {
        self.host_columns(database)
            .zip(self.referenced_columns(database))
            .map(|(host_col, ref_col)| {
                format!(
                    "{} -> {}",
                    host_col.column_schema_doc_path(database),
                    ref_col.column_schema_doc_path(database)
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Returns the vector of unique crate references necessary for the
    /// documentation schema representation.
    ///
    /// # Arguments
    ///
    /// * `workspace` - A reference to the workspace instance to query internal
    ///   crates from.
    /// * `database` - A reference to the database instance to query tables and
    ///   columns from.
    fn internal_crate_references<'data>(
        &self,
        workspace: &Workspace,
        database: &Self::DB,
    ) -> Vec<Arc<InternalCrate>>
    where
        <Self::DB as DatabaseLike>::Table: TableSchema,
        <Self::DB as DatabaseLike>::Column: ColumnSchema,
    {
        let host_table = self.host_table(database);
        let referenced_table = self.referenced_table(database);
        let mut crate_refs = Vec::new();
        crate_refs.extend(host_table.table_schema_ref(workspace));
        crate_refs.extend(referenced_table.table_schema_ref(workspace));
        crate_refs.sort_unstable();
        crate_refs.dedup();
        crate_refs
    }
}

impl<T: ForeignKeySynLike> ForeignKeySchema for T {}
