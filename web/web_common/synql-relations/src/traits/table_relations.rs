//! Submodule providing the `TableRelationsLike` trait for SynQL table models.

use syn::Ident;
use synql_core::structs::{InternalDataRef, Workspace};
use synql_models::traits::TableModelLike;

use crate::structs::TableRelations;

/// Name of the module containing the trait for a table.
pub const TRAIT_MODULE_NAME: &str = "relations";

/// Trait representing a SynQL table model.
pub trait TableRelationsLike: TableModelLike {
    /// Returns the name of the crate which will contain the trait
    /// `{TableName}Relations`.
    fn table_relations_crate_name(&self) -> String {
        format!("{}_relations", self.table_singular_snake_name())
    }

    /// Returns the trait name for the table relations trait.
    fn table_relations_trait_name(&self) -> String {
        format!("{}Relations", self.table_singular_camel_name())
    }

    /// Returns the trait ident for the table relations trait.
    fn table_relations_trait_ident(&self) -> Ident {
        Ident::new(&self.table_relations_trait_name(), proc_macro2::Span::call_site())
    }

    /// Returns the
    /// [`InternalData`](synql_core::structs::InternalData)
    /// representing the model for the table.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table model.
    ///
    /// # Examples
    fn relations_trait<'table>(
        &'table self,
        workspace: &'table Workspace,
        database: &'table Self::DB,
    ) -> TableRelations<'table, Self> {
        TableRelations::new(self, workspace, database)
    }

    /// Returns a reference to the model module ref for the table.
    fn relations_trait_ref(&self, workspace: &Workspace) -> Option<InternalDataRef> {
        let crate_ref = workspace.internal_crate(&self.table_relations_crate_name())?;
        Some(InternalDataRef::new(
            crate_ref,
            crate_ref.internal_data(&self.table_singular_camel_name())?,
        ))
    }
}

impl<T: TableModelLike> TableRelationsLike for T {}
