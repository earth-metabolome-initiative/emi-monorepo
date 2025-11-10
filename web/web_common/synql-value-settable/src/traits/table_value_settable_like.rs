//! Submodule providing the `TableValueSettableLike` trait for SynQL table
//! models.

use sql_relations::{
    prelude::ColumnLike,
    traits::{InheritableDatabaseLike, MostConcreteColumnLike},
};
use sql_traits::traits::DatabaseLike;
use syn::Ident;
use synql_core::structs::{TraitVariantRef, Workspace};
use synql_models::traits::TableModelLike;

use crate::structs::table_value_settable::TableValueSettable;

/// Name of the module containing the trait for a table.
pub(crate) const TRAIT_MODULE_NAME: &str = "value_settable";

/// Trait representing a SynQL table value settable trait.
pub trait TableValueSettableLike: TableModelLike {
    /// Returns the crate name for the table value settable trait.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_settable::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT PRIMARY KEY);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// assert_eq!(table.table_value_settable_crate_name(), "my_table_value_settable");
    /// # Ok(())
    /// # }
    /// ```
    fn table_value_settable_crate_name(&self) -> String {
        format!("{}_value_settable", self.table_singular_snake_name())
    }

    /// Returns the trait name for the table value settable trait.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_settable::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT PRIMARY KEY);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// assert_eq!(table.table_value_settable_trait_name(), "MyTableValueSettable");
    /// # Ok(())
    /// # }
    /// ```
    fn table_value_settable_trait_name(&self) -> String {
        format!("{}ValueSettable", self.table_singular_camel_name())
    }

    /// Returns the trait ident for the table value settable trait.
    fn table_value_settable_trait_ident(&self) -> Ident {
        Ident::new(&self.table_value_settable_trait_name(), proc_macro2::Span::call_site())
    }

    /// Returns an iterator over the value settable columns for the table.
    fn value_settable_columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>
    where
        Self::DB: InheritableDatabaseLike,
    {
        self.columns(database).filter(move |column| {
            !(column.is_generated()
                || column.is_most_concrete(database)
                || column.is_foreign_key(database))
        })
    }

    /// Returns the [`TableValueSettable`]
    /// representing the value settable trait for the table.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table model.
    fn value_settable_trait<'table>(
        &'table self,
        workspace: &'table Workspace,
        database: &'table Self::DB,
    ) -> TableValueSettable<'table, Self> {
        TableValueSettable::new(self, workspace, database)
    }

    /// Returns the trait reference for the table value settable trait.
    fn value_settable_trait_ref<'table>(
        &'table self,
        workspace: &'table Workspace,
    ) -> Option<TraitVariantRef> {
        let crate_ref = workspace.internal_crate(&self.table_value_settable_crate_name())?;
        let trait_ref = crate_ref.internal_trait(&self.table_value_settable_trait_name())?;
        Some(TraitVariantRef::Internal(trait_ref.clone(), Some(crate_ref.clone())))
    }
}

impl<T: TableModelLike> TableValueSettableLike for T {}
