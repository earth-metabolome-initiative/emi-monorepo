//! Submodule providing a struct defining the type required by some type found
//! in the postgres database schema.

mod builder;
mod traits_mask;
use std::{fmt::Debug, hash::Hash};

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
pub use traits_mask::Trait;

use crate::{
    structs::{
        DataVariantRef, external_crate::ExternalTraitRef, external_trait::TraitVariantRef,
        external_type::builder::ExternalTypeBuilder,
    },
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// Struct defining the type required by some type found in the postgres
/// database schema.
pub struct ExternalType {
    /// The diesel type defined within the crate compatible with the given
    /// postgres type.
    diesel_type: Option<syn::Type>,
    /// The rust type defined within the crate compatible with the given
    /// postgres type.
    rust_type: syn::Type,
    /// The postgres types which are compatible with the diesel and rust types
    /// defined within the crate.
    postgres_types: Vec<&'static str>,
    /// The traits supported by the current type.
    traits: traits_mask::TraitsMask,
    /// External traits implemented by the type.
    external_traits: Vec<ExternalTraitRef>,
    /// Generic parameters of the type.
    generics: Vec<syn::GenericParam>,
    /// Default values for the generic parameters of the type.
    generic_defaults: Vec<Option<DataVariantRef>>,
}

unsafe impl Send for ExternalType {}
unsafe impl Sync for ExternalType {}

impl ExternalType {
    /// Inizializes a new `ExternalTypeBuilder`.
    #[must_use]
    pub fn new() -> ExternalTypeBuilder {
        ExternalTypeBuilder::default()
    }

    /// Returns the diesel type defined within the crate compatible with the
    /// given postgres type.
    #[must_use]
    pub fn diesel_type(&self) -> Option<&syn::Type> {
        self.diesel_type.as_ref()
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
    pub fn supports(&self, trait_ref: &TraitVariantRef) -> bool {
        match trait_ref {
            TraitVariantRef::External(ext_trait_ref) => {
                if self.external_traits.contains(ext_trait_ref) {
                    return true;
                }
                if let Ok(r#trait) = ext_trait_ref.external_trait().try_into() {
                    return self.traits.supports(r#trait);
                }
            }
            TraitVariantRef::Internal(_, _) => {}
        }
        false
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

    /// Returns an iterator over the generic idents without defaults.
    pub fn generics_without_defaults(&self) -> impl Iterator<Item = &syn::GenericParam> + '_ {
        self.generics.iter().enumerate().filter_map(|(i, generic)| {
            if self.generic_defaults[i].is_none() { Some(generic) } else { None }
        })
    }

    /// Returns the formatted generics, with defaults in place of the generic
    /// where they exist.
    #[must_use]
    pub fn generics_with_defaults(&self) -> Option<TokenStream> {
        if self.generics.is_empty() {
            None
        } else {
            let generics_with_defaults =
                self.generics.iter().zip(self.generic_defaults.iter()).map(
                    |(generic_param, default)| {
                        if let Some(default) = default {
                            quote::quote! { #default }
                        } else {
                            quote::quote! { #generic_param }
                        }
                    },
                );
            Some(quote::quote! { <#(#generics_with_defaults),*> })
        }
    }

    /// Sets a generic field to the provided `DataVariantRef`.
    #[must_use]
    pub fn set_generic_field(
        &self,
        field: &syn::GenericParam,
        value: DataVariantRef,
    ) -> Option<Self> {
        if !self.generics.contains(field) {
            return None;
        }

        let mut new = self.clone();
        for (i, generic) in self.generics.iter().enumerate() {
            if generic == field {
                new.generic_defaults[i] = Some(value);
                return Some(new);
            }
        }

        unreachable!()
    }
}

impl ExternalDependencies for ExternalType {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &crate::structs::ExternalCrate> {
        self.generic_defaults
            .iter()
            .filter_map(|d| d.as_ref())
            .flat_map(ExternalDependencies::external_dependencies)
    }
}

impl InternalDependencies for ExternalType {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &crate::structs::InternalCrate> {
        self.generic_defaults
            .iter()
            .filter_map(|d| d.as_ref())
            .flat_map(InternalDependencies::internal_dependencies)
    }
}
