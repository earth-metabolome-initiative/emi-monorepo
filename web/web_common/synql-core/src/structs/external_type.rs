//! Submodule providing a struct defining the type required by some type found
//! in the postgres database schema.

mod builder;
mod traits_mask;
use std::{fmt::Debug, hash::Hash};

use quote::ToTokens;
pub use traits_mask::Trait;

use crate::structs::external_type::builder::ExternalTypeBuilder;

#[derive(Clone)]
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

impl PartialEq for ExternalType {
    fn eq(&self, other: &Self) -> bool {
        self.diesel_type.to_token_stream().to_string()
            == other.diesel_type.to_token_stream().to_string()
            && self.rust_type.to_token_stream().to_string()
                == other.rust_type.to_token_stream().to_string()
            && self.postgres_types == other.postgres_types
            && self.traits == other.traits
    }
}

impl Eq for ExternalType {}

impl PartialOrd for ExternalType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExternalType {
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

impl Hash for ExternalType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.diesel_type.to_token_stream().to_string().hash(state);
        self.rust_type.to_token_stream().to_string().hash(state);
        self.postgres_types.hash(state);
        self.traits.hash(state);
    }
}

unsafe impl Send for ExternalType {}
unsafe impl Sync for ExternalType {}

impl Debug for ExternalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExternalType")
            .field("diesel_type", &self.diesel_type.to_token_stream().to_string())
            .field("rust_type", &self.rust_type.to_token_stream().to_string())
            .field("postgres_types", &self.postgres_types)
            .field("traits", &self.traits)
            .finish()
    }
}

impl ExternalType {
    /// Inizializes a new `ExternalTypeBuilder`.
    pub fn new() -> ExternalTypeBuilder {
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
    /// * `trait` - The trait to check support for.
    pub fn supports(&self, r#trait: Trait) -> bool {
        self.traits.supports(r#trait)
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
