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
    /// Returns the syntax name as a string.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Syntax::PostgreSQL => "postgres",
            Syntax::SQLite => "sqlite",
        }
    }

    /// Returns the feature flag for the syntax.
    pub(crate) fn as_feature_flag(self) -> TokenStream {
        let flag_name = self.as_str();
        quote::quote! { #[cfg(feature = #flag_name)] }
    }

    /// Returns the diesel connection type for the syntax.
    pub(crate) fn as_connection_type(self) -> syn::Type {
        match self {
            Syntax::PostgreSQL => syn::parse_quote! { diesel::PgConnection },
            Syntax::SQLite => {
                syn::parse_quote! { diesel::SqliteConnection }
            }
        }
    }
}
