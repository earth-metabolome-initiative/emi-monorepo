//! Submodule defining the `ColumnSchema` trait which provides documentation
//! paths for columns in the diesel schema.

use sql_traits::traits::{ColumnLike, DatabaseLike};
use synql_core::traits::{ColumnSynLike, TableSynLike};

use crate::traits::TableSchema;

/// Trait to provide documentation paths for columns in the diesel schema.
pub trait ColumnSchema: ColumnSynLike + Sized {
    /// Returns the Markdown formatted documentation path for the column in the
    /// table schema module.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the table
    ///   from.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use synql_diesel_schema::prelude::*;
    /// let db = ParserDB::try_from(
    ///     "CREATE TABLE my_table (id INT PRIMARY KEY NOT NULL, name TEXT NOT NULL);",
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let column = table.column("name", &db).unwrap();
    /// let doc_path = column.column_schema_doc_path(&db);
    /// assert_eq!(doc_path, "[`name`](my_table_schema::schema::my_table::name)");
    /// # Ok(())
    /// # }
    /// ```
    fn column_schema_doc_path(&self, database: &Self::DB) -> String
    where
        <Self::DB as DatabaseLike>::Table: TableSchema,
    {
        let table: &<Self::DB as DatabaseLike>::Table = ColumnLike::table(self, database);
        format!(
            "[`{}`]({}::schema::{}::{})",
            self.column_snake_name(),
            table.table_schema_crate_name(),
            table.table_snake_name(),
            self.column_snake_name()
        )
    }

    /// Returns the `syn::Path` to the column in the table schema module.
    fn column_path(&self, database: &Self::DB) -> syn::Path
    where
        <Self::DB as DatabaseLike>::Table: TableSchema,
    {
        let table: &<Self::DB as DatabaseLike>::Table = ColumnLike::table(self, database);
        let crate_name = table.table_schema_crate_ident();
        let table_name = table.table_snake_ident();
        let column_name = self.column_snake_ident();

        syn::parse_quote!(
            #crate_name::schema::#table_name::#column_name
        )
    }
}

impl<T: ColumnSynLike> ColumnSchema for T {}
