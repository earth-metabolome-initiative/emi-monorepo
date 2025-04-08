//! Syntaxes supported by the webcodegen tool.

use proc_macro2::TokenStream;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Syntax {
    #[default]
    /// The `PostgreSQL` syntax,
    PostgreSQL,
    /// The `SQLite` syntax.
    SQLite,
}

impl Syntax {
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

    /// Returns the diesel connection type for the syntax.
    pub fn as_connection_type(self) -> syn::Type {
        match self {
            Syntax::PostgreSQL => syn::parse_quote! { diesel_async::AsyncPgConnection },
            Syntax::SQLite => {
                syn::parse_quote! { diesel_async::SyncConnectionWrapper<diesel::SqliteConnection> }
            }
        }
    }
}
