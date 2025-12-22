//! Submodule providing the `UniqueIndexSynLike` trait extending the
//! `UniqueIndexLike` trait adding method to convert to a Syn representation.

use proc_macro2::TokenStream;
use quote::quote;
use sql_traits::traits::UniqueIndexLike;

use crate::traits::{ColumnSynLike, TableSynLike};

/// Trait extending `UniqueIndexLike` to provide methods for generating
/// Rust code representations of SQL unique indexes.
pub trait UniqueIndexSynLike: UniqueIndexLike {
    /// Converts the unique index to its Syn representation,
    /// using the `diesel-builders` crate.
    fn to_syn(&self, database: &Self::DB) -> TokenStream {
        let table_ident = self.table(database).table_snake_ident();
        let column_idents =
            self.columns(database).map(|col| col.column_snake_ident()).map(|ident| {
                quote! {#table_ident::#ident}
            });
        quote! {
            diesel_builders::prelude::unique_index!(#(#column_idents),*);
        }
    }
}

impl<UI> UniqueIndexSynLike for UI where UI: UniqueIndexLike {}
