//! Submodule defining the `TableSchema` trait which allows to generate the
//! `diesel` schema from a SQL schema, based on `sql_traits`.

use synql_core::{
    structs::{InternalModuleRef, Workspace},
    traits::TableSynLike,
};

use crate::structs::schema_macro::SchemaMacro;

/// Name of the module containing the diesel schema for a table.
pub const TABLE_SCHEMA_MODULE_NAME: &str = "schema";

/// Trait to create the `diesel` schema from a SQL schema.
pub trait TableSchema: TableSynLike + Sized {
    /// Returns the name of the crate which will contain the diesel schema for
    /// the table.
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
    /// let workspace = Workspace::new().name("my_workspace")?.core()?.std()?.build()?;
    ///
    /// let db = ParserDB::try_from(
    ///     "CREATE TABLE my_table (id INT PRIMARY KEY NOT NULL, name TEXT NOT NULL, age INT);",
    /// )?;
    /// let table = db.table(None, "my_table");
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
    fn schema_macro<'data, 'table>(
        &'table self,
        workspace: &'table Workspace<'data>,
        database: &'table Self::Database,
    ) -> SchemaMacro<'data, 'table, Self>
    where
        Self: 'data,
    {
        SchemaMacro::new(self, workspace, database)
    }

    /// Returns a reference to the schema module ref for the table.
    fn schema_module<'data>(
        &self,
        workspace: &Workspace<'data>,
    ) -> Option<InternalModuleRef<'data>> {
        let crate_ref = workspace.internal_crate(&self.table_schema_crate_name())?;
        Some(InternalModuleRef::new(crate_ref, crate_ref.module(TABLE_SCHEMA_MODULE_NAME)?))
    }
}

impl<T: TableSynLike> TableSchema for T {}
