//! Submodule providing a struct defining the type required by some type found
//! in the postgres database schema.

mod builder;
mod traits_mask;
use std::{fmt::Debug, hash::Hash};

use quote::ToTokens;
pub use traits_mask::Trait;

use crate::structs::{
    external_crate::ExternalTraitRef, external_trait::TraitVariantRef,
    external_type::builder::ExternalTypeBuilder,
};

#[derive(Clone)]
/// Struct defining the type required by some type found in the postgres
/// database schema.
pub struct ExternalType<'data> {
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
    /// External traits implemented by the type.
    external_traits: Vec<ExternalTraitRef<'data>>,
}

impl PartialEq for ExternalType<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.diesel_type.to_token_stream().to_string()
            == other.diesel_type.to_token_stream().to_string()
            && self.rust_type.to_token_stream().to_string()
                == other.rust_type.to_token_stream().to_string()
            && self.postgres_types == other.postgres_types
            && self.traits == other.traits
            && self.external_traits == other.external_traits
    }
}

impl<'data> Eq for ExternalType<'data> {}

impl<'data> PartialOrd for ExternalType<'data> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'data> Ord for ExternalType<'data> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let diesel_cmp = self
            .diesel_type
            .to_token_stream()
            .to_string()
            .cmp(&other.diesel_type.to_token_stream().to_string());
        if diesel_cmp != std::cmp::Ordering::Equal {
            return diesel_cmp;
        }

        let rust_cmp = self
            .rust_type
            .to_token_stream()
            .to_string()
            .cmp(&other.rust_type.to_token_stream().to_string());
        if rust_cmp != std::cmp::Ordering::Equal {
            return rust_cmp;
        }

        let pg_types_cmp = self.postgres_types.cmp(&other.postgres_types);
        if pg_types_cmp != std::cmp::Ordering::Equal {
            return pg_types_cmp;
        }

        self.traits.cmp(&other.traits)
    }
}

impl<'data> Hash for ExternalType<'data> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.diesel_type.to_token_stream().to_string().hash(state);
        self.rust_type.to_token_stream().to_string().hash(state);
        self.postgres_types.hash(state);
        self.traits.hash(state);
    }
}

unsafe impl<'data> Send for ExternalType<'data> {}
unsafe impl<'data> Sync for ExternalType<'data> {}

impl<'data> Debug for ExternalType<'data> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExternalType")
            .field("diesel_type", &self.diesel_type.to_token_stream().to_string())
            .field("rust_type", &self.rust_type.to_token_stream().to_string())
            .field("postgres_types", &self.postgres_types)
            .field("traits", &self.traits)
            .finish()
    }
}

impl<'data> ExternalType<'data> {
    /// Inizializes a new `ExternalTypeBuilder`.
    pub fn new() -> ExternalTypeBuilder<'data> {
        ExternalTypeBuilder::default()
    }

    /// Returns the diesel type defined within the crate compatible with the
    /// given postgres type.
    pub fn diesel_type(&self) -> &syn::Type {
        &self.diesel_type
    }

    /// Returns the rust type defined within the crate compatible with the given
    /// postgres type.
    pub fn rust_type(&self) -> &syn::Type {
        &self.rust_type
    }

    /// Returns a reference over the postgres types which are compatible with
    /// the diesel and rust types defined within the crate.
    pub fn postgres_types(&self) -> &[&'static str] {
        &self.postgres_types
    }

    /// Returns whether the Rust type associated with the current `ExternalType`
    /// supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait to check support for.
    pub fn supports(&self, trait_ref: &TraitVariantRef<'data>) -> bool {
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
    pub fn is_compatible_with(&self, postgres_type: &str) -> bool {
        self.postgres_types.iter().any(|t| t.eq_ignore_ascii_case(postgres_type))
    }
}
