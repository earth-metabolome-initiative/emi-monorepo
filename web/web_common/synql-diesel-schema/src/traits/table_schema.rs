//! Submodule defining the `TableSchema` trait which allows to generate the
//! `diesel` schema from a SQL schema, based on `sql_traits`.

use std::sync::Arc;

use synql_core::{
    structs::{InternalCrate, InternalModuleRef, Workspace},
    traits::TableSynLike,
};

use crate::structs::schema_macro::SchemaMacro;

/// Name of the module containing the diesel schema for a table.
pub const TABLE_SCHEMA_MODULE_NAME: &str = "schema";

/// Trait to create the `diesel` schema from a SQL schema.
pub trait TableSchema: TableSynLike + Sized {
    /// Returns the name of the crate which will contain the diesel schema for
    /// the table.
    #[inline]
    fn table_schema_crate_name(&self) -> String {
        format!("{}_schema", self.table_snake_name())
    }

    /// Returns the `SchemaMacro` representing the diesel schema for the table.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table schema.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use synql_diesel_schema::prelude::*;
    /// use quote::{ToTokens, quote};
    /// let workspace = Workspace::new().name("my_workspace")?.core().std().diesel().build()?;
    ///
    /// let db = ParserDB::try_from(
    ///     "CREATE TABLE my_table (id INT PRIMARY KEY NOT NULL, name TEXT NOT NULL, age INT);",
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let schema_macro = table.schema_macro(&workspace, &db);
    /// let tokens = quote! { #schema_macro };
    /// assert_eq!(
    ///     tokens.to_string(),
    ///     quote! {
    ///         diesel::table! {
    ///             #[doc = "Undocumented table"]
    ///             my_table (id) {
    ///                 #[doc = "Undocumented column"]
    ///                 id -> diesel :: sql_types :: Integer,
    ///                 #[doc = "Undocumented column"]
    ///                 name -> diesel :: sql_types :: Text,
    ///                 #[doc = "Undocumented column"]
    ///                 age -> diesel :: sql_types :: Nullable<diesel :: sql_types :: Integer>
    ///             }
    ///         }
    ///     }
    ///     .to_string()
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn schema_macro<'table>(
        &'table self,
        workspace: &'table Workspace,
        database: &'table Self::DB,
    ) -> SchemaMacro<'table, Self> {
        SchemaMacro::new(self, workspace, database)
    }

    /// Returns a reference to the schema module ref for the table.
    fn schema_module(&self, workspace: &Workspace) -> Option<InternalModuleRef> {
        let crate_ref = self.table_schema_ref(workspace)?;
        Some(InternalModuleRef::new(&crate_ref, crate_ref.module(TABLE_SCHEMA_MODULE_NAME)?))
    }

    /// Returns a reference to the schema crate ref for the table.
    fn table_schema_ref(&self, workspace: &Workspace) -> Option<Arc<InternalCrate>> {
        workspace.internal_crate(&self.table_schema_crate_name()).cloned()
    }

    /// Returns the Markdown formatted documentation path for the table schema
    /// module.
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
    /// let doc_path = table.table_schema_doc_path();
    /// assert_eq!(doc_path, "[`my_table`](my_table_schema::schema::my_table)");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn table_schema_doc_path(&self) -> String {
        format!(
            "[`{}`]({}::{TABLE_SCHEMA_MODULE_NAME}::{})",
            self.table_snake_name(),
            self.table_schema_crate_name(),
            self.table_snake_name()
        )
    }
}

impl<T: TableSynLike> TableSchema for T {}
