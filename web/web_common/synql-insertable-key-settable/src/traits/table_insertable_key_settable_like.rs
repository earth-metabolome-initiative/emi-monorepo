//! Submodule providing the `TableInsertableKeySettableLike` trait for SynQL
//! table models.

use sql_traits::traits::{ColumnLike, DatabaseLike, ForeignKeyLike};
use syn::Ident;
use synql_core::structs::{TraitVariantRef, Workspace};
use synql_models::traits::TableModelLike;

use crate::structs::table_insertable_key_settable::TableInsertableKeySettable;

/// Name of the module containing the trait for a table.
pub(crate) const TRAIT_MODULE_NAME: &str = "insertable_key_settable";

/// Trait representing a SynQL table insertable key settable trait.
pub trait TableInsertableKeySettableLike: TableModelLike {
    /// Returns the crate name for the table insertable key settable trait.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_settable::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT PRIMARY KEY);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// assert_eq!(
    ///     table.table_insertable_key_settable_crate_name(),
    ///     "my_table_insertable_key_settable"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn table_insertable_key_settable_crate_name(&self) -> String {
        format!("{}_insertable_key_settable", self.table_singular_snake_name())
    }

    /// Returns the trait name for the table insertable key settable trait.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_settable::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT PRIMARY KEY);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// assert_eq!(table.table_insertable_key_settable_trait_name(), "MyTableInsertableKeySettable");
    /// # Ok(())
    /// # }
    /// ```
    fn table_insertable_key_settable_trait_name(&self) -> String {
        format!("{}InsertableKeySettable", self.table_singular_camel_name())
    }

    /// Returns the trait ident for the table insertable key settable trait.
    fn table_insertable_key_settable_trait_ident(&self) -> Ident {
        Ident::new(&self.table_insertable_key_settable_trait_name(), proc_macro2::Span::call_site())
    }

    /// Returns an iterator over the insertable key settable columns for the
    /// table.
    fn insertable_key_settable_foreign_keys<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey> {
        let mut covered_columns: Vec<&'db <Self::DB as DatabaseLike>::Column> = vec![];
        self.foreign_keys(database).filter(move |foreign_key| {
            let Some(host_column) = foreign_key.host_column(database) else {
                return false;
            };
            if covered_columns.contains(&host_column) || host_column.is_generated() {
                return false;
            }
            covered_columns.push(host_column);
            true
        })
    }

    /// Returns the [`TableInsertableKeySettable`]
    /// representing the insertable key settable trait for the table.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table model.
    fn insertable_key_settable_trait<'table>(
        &'table self,
        workspace: &'table Workspace,
        database: &'table Self::DB,
    ) -> TableInsertableKeySettable<'table, Self> {
        TableInsertableKeySettable::new(self, workspace, database)
    }

    /// Returns the trait reference for the table insertable key settable trait.
    fn insertable_key_settable_trait_ref<'table>(
        &'table self,
        workspace: &'table Workspace,
    ) -> Option<TraitVariantRef> {
        let crate_ref =
            workspace.internal_crate(&self.table_insertable_key_settable_crate_name())?;
        let trait_ref =
            crate_ref.internal_trait(&self.table_insertable_key_settable_trait_name())?;
        Some(TraitVariantRef::Internal(trait_ref.clone(), Some(crate_ref.clone())))
    }
}

impl<T: TableModelLike + ?Sized> TableInsertableKeySettableLike for T {}
