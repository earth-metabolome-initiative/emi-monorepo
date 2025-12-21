//! Submodule for core types used in workspaces.

use crate::structs::{ExternalTypeRef, Workspace};

impl Workspace {
    #[inline]
    #[must_use]
    /// Returns a reference to `f64` external type.
    pub fn f64(&self) -> ExternalTypeRef<'_> {
        self.external_type(&syn::parse_quote!(f64)).unwrap()
    }

    #[inline]
    #[must_use]
    /// Returns a reference to `bool` external type.
    pub fn bool(&self) -> ExternalTypeRef<'_> {
        self.external_type(&syn::parse_quote!(bool)).unwrap()
    }

    #[inline]
    #[must_use]
    /// Returns a reference to `String` external type.
    pub fn string(&self) -> ExternalTypeRef<'_> {
        self.external_type(&syn::parse_quote!(String)).unwrap()
    }
}
