//! Submodule providing the [`Backend`] trait.

use syn::Ident;

pub(crate) trait Backend {
    /// Returns the SQL type name for the given backend.
    ///
    /// ## Arguments
    ///
    /// * `name` - The name of the type.
    fn sql_type_attribute(&self, name: &str) -> proc_macro2::TokenStream;

    /// Returns the `ToSql` implementation for the given backend.
    ///
    /// ## Arguments
    ///
    /// * `ident` - The ident of the type.
    fn to_sql_impl(&self, ident: &Ident) -> proc_macro2::TokenStream;

    #[allow(clippy::wrong_self_convention)]
    /// Returns the `FromSql` implementation for the given backend.
    ///
    /// ## Arguments
    ///
    /// * `ident` - The ident of the type.
    fn from_sql_impl(&self, ident: &Ident) -> proc_macro2::TokenStream;
}
