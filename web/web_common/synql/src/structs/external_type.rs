//! Submodule providing a struct defining the type required by some type found
//! in the postgres database schema.

mod builder;
mod traits_mask;
use std::{fmt::Debug, hash::Hash};

pub use builder::ExternalTypeBuilder;
use quote::{ToTokens, quote};
pub use traits_mask::Trait;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// Struct defining the type required by some type found in the postgres
/// database schema.
pub struct ExternalType {
    /// The diesel type defined within the crate compatible with the given
    /// postgres type.
    diesel_type: syn::Type,
    /// The rust type defined within the crate compatible with the given
    /// postgres type.
    rust_type: syn::Type,
    /// The postgres types which are compatible with the diesel and rust types
    /// defined within the crate.
    postgres_types: Vec<&'static str>,
    /// The traits supported by the current type.
    traits: traits_mask::TraitsMask,
}

impl ExternalType {
    /// Inizializes a new `ExternalTypeBuilder`.
    #[must_use]
    pub fn new(
        diesel_type: syn::Type,
        rust_type: syn::Type,
    ) -> ExternalTypeBuilder {
        ExternalTypeBuilder::new(diesel_type, rust_type)
    }

    /// Returns the diesel type defined within the crate compatible with the
    /// given postgres type.
    #[must_use]
    pub fn diesel_type(&self) -> &syn::Type {
        &self.diesel_type
    }

    /// Returns the rust type defined within the crate compatible with the given
    /// postgres type.
    #[must_use]
    pub fn rust_type(&self) -> &syn::Type {
        &self.rust_type
    }

    /// Returns a reference over the postgres types which are compatible with
    /// the diesel and rust types defined within the crate.
    #[must_use]
    pub fn postgres_types(&self) -> &[&'static str] {
        &self.postgres_types
    }

    /// Returns whether the type is a `Unit` type.
    #[must_use]
    pub fn is_unit(&self) -> bool {
        self.rust_type.to_token_stream().to_string() == "()"
    }

    /// Returns whether the Rust type associated with the current `ExternalType`
    /// supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait to check support for.
    #[must_use]
    pub fn supports(&self, trait_ref: Trait) -> bool {
        self.traits.supports(trait_ref)
    }

    /// Returns whether the current `ExternalType` is compatible with the given
    /// postgres type.
    ///
    /// # Arguments
    /// * `postgres_type` - The postgres type to check compatibility with.
    #[must_use]
    pub fn is_compatible_with(&self, postgres_type: &str) -> bool {
        self.postgres_types.iter().any(|t| t.eq_ignore_ascii_case(postgres_type))
    }

    /// Casts a value to the external type.
    pub(crate) fn cast(&self, value: &str) -> Result<proc_macro2::TokenStream, syn::Error> {
        let string_rust_type = self.rust_type.to_token_stream().to_string();

        // Reusable error factory for parse failures in the match arms.
        let mk_err = || {
            syn::Error::new_spanned(
                self.rust_type.to_token_stream(),
                format!("Cannot cast value to external type: {self:?}"),
            )
        };

        Ok(match string_rust_type.as_str() {
            "String" | "str" => {
                quote! { #value }
            }
            "bool" => {
                let casted: bool = value.parse().map_err(|_| mk_err())?;
                quote! { #casted }
            }
            // Numeric parsing arms. Keep them explicit (simple and clear)
            // while still using the shared `mk_err` closure above to
            // avoid duplicated error construction.
            "i16" => {
                let casted: i16 = value.parse().map_err(|_| mk_err())?;
                quote! { #casted }
            }
            "u16" => {
                let casted: u16 = value.parse().map_err(|_| mk_err())?;
                quote! { #casted }
            }
            "i32" => {
                let casted: i32 = value.parse().map_err(|_| mk_err())?;
                quote! { #casted }
            }
            "u32" => {
                let casted: u32 = value.parse().map_err(|_| mk_err())?;
                quote! { #casted }
            }
            "i64" => {
                let casted: i64 = value.parse().map_err(|_| mk_err())?;
                quote! { #casted }
            }
            "u64" => {
                let casted: u64 = value.parse().map_err(|_| mk_err())?;
                quote! { #casted }
            }
            "f32" => {
                let casted: f32 = value.parse().map_err(|_| mk_err())?;
                quote! { #casted }
            }
            "f64" => {
                let casted: f64 = value.parse().map_err(|_| mk_err())?;
                quote! { #casted }
            }
            "u128" => {
                let casted: u128 = value.parse().map_err(|_| mk_err())?;
                quote! { #casted }
            }
            "i128" => {
                let casted: i128 = value.parse().map_err(|_| mk_err())?;
                quote! { #casted }
            }
            _ => {
                return Err(mk_err());
            }
        })
    }
}
