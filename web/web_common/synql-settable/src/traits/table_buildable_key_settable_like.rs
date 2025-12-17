//! Submodule providing the `TableSettableLike` trait for SynQL
//! table models.

use sql_relations::traits::HorizontalSameAsColumnLike;
use sql_traits::traits::{ColumnLike, DatabaseLike, ForeignKeyLike};
use syn::Ident;
use synql_core::structs::{TraitVariantRef, Workspace};
use synql_models::traits::TableModelLike;

use crate::structs::table_buildable_key_settable::TableSettable;

/// Name of the module containing the trait for a table.
pub(crate) const TRAIT_MODULE_NAME: &str = "buildable_key_settable";

/// Trait representing a SynQL table buildable key settable trait.
pub trait TableSettableLike: TableModelLike {
    /// Returns the crate name for the table buildable key settable trait.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_settable::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT PRIMARY KEY);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// assert_eq!(table.table_buildable_key_settable_crate_name(), "my_table_buildable_key_settable");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn table_buildable_key_settable_crate_name(&self) -> String {
        format!("{}_buildable_key_settable", self.table_singular_snake_name())
    }

    /// Returns the trait name for the table buildable key settable trait.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_settable::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT PRIMARY KEY);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// assert_eq!(table.table_buildable_key_settable_trait_name(), "MyTableSettable");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn table_buildable_key_settable_trait_name(&self) -> String {
        format!("{}BuildableKeySettable", self.table_singular_camel_name())
    }

    /// Returns the trait ident for the table buildable key settable trait.
    #[inline]
    fn table_buildable_key_settable_trait_ident(&self) -> Ident {
        Ident::new(&self.table_buildable_key_settable_trait_name(), proc_macro2::Span::call_site())
    }

    /// Returns an iterator over the buildable key settable columns for the
    /// table.
    fn buildable_key_settable_foreign_keys<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey> {
        let mut covered_columns: Vec<&'db <Self::DB as DatabaseLike>::Column> = vec![];
        self.foreign_keys(database).filter(move |foreign_key| {
            let Some(host_column) = foreign_key.host_column(database) else {
                return false;
            };
            if covered_columns.contains(&host_column)
                || host_column.is_generated()
                || host_column.has_horizontal_same_as_foreign_key(database)
                || host_column.is_primary_key(database)
            {
                return false;
            }
            covered_columns.push(host_column);
            true
        })
    }

    /// Returns the [`TableSettable`]
    /// representing the buildable key settable trait for the table.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table model.
    fn buildable_key_settable_trait<'table>(
        &'table self,
        workspace: &'table Workspace,
        database: &'table Self::DB,
    ) -> TableSettable<'table, Self> {
        TableSettable::new(self, workspace, database)
    }

    /// Returns the trait reference for the table buildable key settable trait.
    fn buildable_key_settable_trait_ref<'table>(
        &'table self,
        workspace: &'table Workspace,
    ) -> Option<TraitVariantRef> {
        let crate_ref =
            workspace.internal_crate(&self.table_buildable_key_settable_crate_name())?;
        let trait_ref =
            crate_ref.internal_trait(&self.table_buildable_key_settable_trait_name())?;
        Some(TraitVariantRef::Internal(trait_ref.clone(), Some(crate_ref.clone())))
    }
}

impl<T: TableModelLike + ?Sized> TableSettableLike for T {}
