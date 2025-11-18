//! Submodule providing the `TableBuildable` trait for SynQL table buildables.

use proc_macro2::TokenStream;
use synql_builders_metadata::traits::TableBuildableMetadataLike;
use synql_core::{structs::Workspace, utils::generic_type};

use crate::structs::TableBuildable;

/// Name of the module containing the buildable for a table.
pub const BUILDABLE_MODULE_NAME: &str = "buildable";

/// Trait representing a SynQL table buildable.
pub trait TableBuildableLike: TableBuildableMetadataLike {
    /// Returns the generics for the buildable struct.
    ///
    /// # Arguments
    ///
    /// * `database` - The database connection to use to query the table
    ///  buildable.
    #[inline]
    fn table_buildable_generics(&self, database: &Self::DB) -> Vec<syn::GenericParam> {
        self.extended_tables(database)
            .into_iter()
            .map(|t| generic_type(&t.table_singular_camel_name()))
            .collect()
    }

    /// Returns the identifier of the buildable struct for the table.
    #[inline]
    fn formatted_table_buildable_generics(&self, database: &Self::DB) -> Option<TokenStream> {
        let generics = self.table_buildable_generics(database);
        if generics.is_empty() { None } else { Some(syn::parse_quote! { <#(#generics),*> }) }
    }

    /// Returns the [`TableBuildable`](crate::structs::TableBuildable)
    /// representing the buildable for the table.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table
    ///   buildable.
    #[inline]
    fn buildable<'table>(
        &'table self,
        workspace: &'table Workspace,
        database: &'table Self::DB,
    ) -> TableBuildable<'table, Self> {
        TableBuildable::new(self, workspace, database)
    }
}

impl<T: TableBuildableMetadataLike> TableBuildableLike for T {}
