//! Struct representing a reference to an external crate and one of its types.

use quote::ToTokens;

use crate::structs::{ExternalCrate, ExternalType, external_type::Trait};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Struct representing a reference to an external crate and one of its types.
pub struct ExternalTypeRef<'a> {
    crate_ref: &'a ExternalCrate,
    type_ref: &'a ExternalType,
}

impl ExternalTypeRef<'_> {
	/// Creates a new `ExternalTypeRef`.
	pub(super) fn new<'a>(
		crate_ref: &'a ExternalCrate,
		type_ref: &'a ExternalType,
	) -> ExternalTypeRef<'a> {
		ExternalTypeRef { crate_ref, type_ref }
	}

    /// Returns a reference to the name of the crate.
    #[inline]
    #[must_use]
    pub fn crate_name(&self) -> &str {
        self.crate_ref.name()
    }

    /// Returns a reference to the diesel type.
    #[inline]
    #[must_use]
    pub fn diesel_type(&self) -> &syn::Type {
        self.type_ref.diesel_type()
    }

    /// Returns a reference to the rust type.
    #[inline]
    #[must_use]
    pub fn rust_type(&self) -> &syn::Type {
        self.type_ref.rust_type()
    }

    /// Returns a reference to the external crate.
    #[inline]
    #[must_use]
    pub fn external_crate(&self) -> &ExternalCrate {
        &self.crate_ref
    }

    /// Returns true if the crate is a dependency.
    #[inline]
    #[must_use]
    pub fn is_dependency(&self) -> bool {
        self.crate_ref.is_dependency()
    }

    /// Returns the version of the crate if it is a dependency.
    #[inline]
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.crate_ref.version()
    }

    /// Returns the git repository and branch of the crate if it is a
    /// dependency.
    #[inline]
    #[must_use]
    pub fn git(&self) -> Option<(&str, &str)> {
        self.crate_ref.git()
    }

    /// Returns true if the type supports the provided trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait variant to check support for.
    #[must_use]
    pub fn supports_trait(&self, trait_ref: Trait) -> bool {
        self.type_ref.supports(trait_ref)
    }

    /// Returns whether the type supports the `Copy` trait in Rust.
    #[must_use]
    pub fn supports_copy(&self) -> bool {
        self.type_ref.supports(Trait::Copy)
    }

    /// Returns whether the type is a unit type.
    #[must_use]
    pub fn is_unit(&self) -> bool {
        self.type_ref.is_unit()
    }

    /// Casts a value to the external type.
    pub(crate) fn cast(&self, value: &str) -> Result<proc_macro2::TokenStream, syn::Error> {
        self.type_ref.cast(value)
    }

	/// Returns true if the `ExternalTypeRef` is of boolean type.
    #[must_use]
    pub fn is_bool(&self) -> bool {
        self.type_ref.is_bool()
    }

    /// Returns true if the `ExternalTypeRef` is of numeric type.
    #[must_use]
    pub fn is_numeric(&self) -> bool {
        self.type_ref.is_numeric()
    }
}

impl ToTokens for ExternalTypeRef<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.type_ref.rust_type().to_tokens(tokens);
    }
}
