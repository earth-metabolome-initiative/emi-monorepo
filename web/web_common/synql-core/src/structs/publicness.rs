//! The visibility level of something (e.g., a module or struct, or an attribute
//! within).

use quote::ToTokens;

/// Enum representing the publicness of something.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Publicness {
    /// Public (e.g., `pub`).
    Public,
    /// Private (e.g., no `pub`).
    Private,
    /// Crate-level visibility (e.g., `pub(crate)`).
    Crate,
    /// Restricted visibility (e.g., `pub(super)`
    Restricted,
}

impl ToTokens for Publicness {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let token = match self {
            Publicness::Public => quote::quote! { pub },
            Publicness::Private => quote::quote! {},
            Publicness::Crate => quote::quote! { pub(crate) },
            Publicness::Restricted => quote::quote! { pub(super) },
        };
        tokens.extend(token);
    }
}

impl Publicness {
    /// Returns true if the publicness is `Public`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use synql_core::structs::Publicness;
    ///
    /// assert!(Publicness::Public.is_public());
    /// assert!(!Publicness::Private.is_public());
    /// assert!(!Publicness::Crate.is_public());
    /// assert!(!Publicness::Restricted.is_public());
    /// ```
    pub fn is_public(&self) -> bool {
        matches!(self, Publicness::Public)
    }
}
