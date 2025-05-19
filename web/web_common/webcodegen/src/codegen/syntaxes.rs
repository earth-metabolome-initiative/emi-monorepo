//! Syntaxes supported by the webcodegen tool.

use proc_macro2::TokenStream;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Syntax {
    /// The `PostgreSQL` syntax,
    PostgreSQL,
    /// The `SQLite` syntax.
    SQLite,
}

impl Syntax {
    /// Returns whether the syntax is `SQLite`.
    pub fn is_sqlite(self) -> bool {
        matches!(self, Syntax::SQLite)
    }

    /// Returns the feature flag for the syntax.
    pub fn as_feature_flag(self) -> TokenStream {
        match self {
            Syntax::PostgreSQL => quote::quote! { #[cfg(feature = "postgres")] },
            Syntax::SQLite => quote::quote! { #[cfg(feature = "sqlite")] },
        }
    }

    /// Returns the diesel backend type for the syntax.
    pub fn as_backend_type(self) -> syn::Type {
        match self {
            Syntax::PostgreSQL => syn::parse_quote! { diesel::pg::Pg },
            Syntax::SQLite => syn::parse_quote! { diesel::sqlite::Sqlite },
        }
    }

    /// Returns the `RunQueryDsl` type for the syntax.
    ///
    /// # Arguments
    ///
    /// * `r#async` - If true, returns the async `RunQueryDsl` type.
    pub fn as_run_query_dsl(r#async: bool) -> syn::Type {
        if r#async {
            syn::parse_quote! { diesel_async::RunQueryDsl }
        } else {
            syn::parse_quote! { diesel::RunQueryDsl }
        }
    }

    /// Returns the diesel connection type for the syntax.
    ///
    /// # Arguments
    ///
    /// * `r#async` - If true, returns the async connection type.
    pub fn as_connection_type(self, r#async: bool) -> syn::Type {
        match (self, r#async) {
            (Syntax::PostgreSQL, true) => syn::parse_quote! { diesel_async::AsyncPgConnection },
            (Syntax::SQLite, true) => {
                syn::parse_quote! { diesel_async::SyncConnectionWrapper<diesel::SqliteConnection> }
            }
            (Syntax::PostgreSQL, false) => syn::parse_quote! { diesel::PgConnection },
            (Syntax::SQLite, false) => syn::parse_quote! { diesel::SqliteConnection },
        }
    }
}
