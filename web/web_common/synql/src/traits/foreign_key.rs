//! Submodule providing syn functionalities for `ForeignKeyLike` objects.

use proc_macro2::TokenStream;
use quote::quote;
use sql_traits::traits::ForeignKeyLike;

use crate::{
    structs::Workspace,
    traits::{ColumnSynLike, TableSynLike},
};

/// Trait defining syn functionalities for `ForeignKeyLike` objects.
pub trait ForeignKeySynLike: ForeignKeyLike {
    /// Returns the syn representation of the foreign key using
    /// the `fk` or the `fpk` macro, depending on whether the foreign key
    /// is a foreign primary key or not.
    fn to_syn(&self, database: &Self::DB, workspace: &Workspace) -> TokenStream {
        let host_table_ident = self.host_table(database).table_snake_ident();
        let foreign_table_ident = self.referenced_table(database).table_snake_ident();
        let foreign_table_crate_ident = self.referenced_table(database).crate_ident(workspace);
        let host_column_paths = self
            .host_columns(database)
            .map(|col| {
                let col_ident = col.column_snake_ident();
                syn::parse_quote!(#host_table_ident::#col_ident)
            })
            .collect::<Vec<syn::Path>>();
        let foreign_column_paths = self
            .referenced_columns(database)
            .map(|col| {
                let col_ident = col.column_snake_ident();
                if self.is_self_referential(database) {
                    syn::parse_quote!(#foreign_table_ident::#col_ident)
                } else {
                    syn::parse_quote!(#foreign_table_crate_ident::#foreign_table_ident::#col_ident)
                }
            })
            .collect::<Vec<syn::Path>>();
        quote! {
            diesel_builders::prelude::fk!((#(#host_column_paths),*) -> (#(#foreign_column_paths),*));
        }
    }
}

impl<FK: ForeignKeyLike> ForeignKeySynLike for FK {}
