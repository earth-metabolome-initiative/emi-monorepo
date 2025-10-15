//! Submodule defining the `TableSchema` trait which allows to generate the
//! `diesel` schema from a SQL schema, based on `sql_traits`.

use synql_core::{structs::Workspace, traits::TableSynLike};

use crate::structs::schema_macro::SchemaMacro;

/// Trait to create the `diesel` schema from a SQL schema.
pub trait TableSchema: TableSynLike + Sized {
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
    /// # use synql_schema::prelude::*;
    /// use quote::{ToTokens, quote};
    /// let workspace = Workspace::new()
    ///    .name("my_workspace")?
    ///    .version(0, 1, 0)
    ///    .core()?
    ///    .std()?
    ///    .build()?;
    ///
    /// let db = ParserDB::try_from(
    ///     "CREATE TABLE my_table (id INT PRIMARY KEY NOT NULL, name TEXT NOT NULL, age INT);"
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let schema_macro = table.schema_macro(&workspace, &db);
    /// let tokens = quote! { #schema_macro };
    /// assert_eq!(tokens.to_string(), quote! {
    ///     diesel::table! {
    ///         my_table (id) {
    ///             id -> diesel :: sql_types :: Integer,
    ///             name -> diesel :: sql_types :: Text,
    ///             age -> diesel :: sql_types :: Nullable<diesel :: sql_types :: Integer>
    ///         }
    ///     }
    /// }.to_string());
    /// # Ok(())
    /// # }
    /// ```
    fn schema_macro<'table>(
        &'table self,
        workspace: &'table Workspace,
        database: &'table Self::Database,
    ) -> SchemaMacro<'table, Self> {
        SchemaMacro::new(self, workspace, database)
    }
}

impl<T: TableSynLike> TableSchema for T {}
