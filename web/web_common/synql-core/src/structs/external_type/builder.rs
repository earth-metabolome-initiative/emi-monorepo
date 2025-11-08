//! Submodule providing a builder for the `ExternalType` struct.

use std::{collections::HashMap, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{
    DataVariantRef, ExternalCrate, ExternalType,
    external_crate::ExternalTraitRef,
    external_type::{Trait, traits_mask::TraitsMask},
};

#[derive(Default)]
/// Builder for the `ExternalType` struct.
pub struct ExternalTypeBuilder {
    /// The diesel type defined within the crate compatible with the given
    /// postgres type.
    diesel_type: Option<syn::Type>,
    /// The rust type defined within the crate compatible with the given
    /// postgres type.
    rust_type: Option<syn::Type>,
    /// The postgres types which are compatible with the diesel and rust types
    /// defined within the crate.
    postgres_types: Vec<&'static str>,
    /// Trait mask representing the traits supported by the current type.
    traits: TraitsMask,
    /// External traits implemented by the type.
    external_traits: Vec<ExternalTraitRef>,
    /// Generic parameters of the type.
    generics: Vec<syn::Ident>,
    /// Default values for the generic parameters of the type.
    generic_defaults: HashMap<syn::Ident, DataVariantRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `ExternalType` struct.
pub enum ExternalTypeAttribute {
    /// The diesel type defined within the crate compatible with the given
    /// postgres type.
    DieselType,
    /// The rust type defined within the crate compatible with the given
    /// postgres type.
    RustType,
    /// The postgres types which are compatible with the diesel and rust types
    /// defined within the crate.
    PostgresTypes,
    /// The trait mask representing the traits.
    Traits,
    /// External traits implemented by the type.
    ExternalTraits,
}

/// Error enumeration which might occur when building a `ExternalType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExternalTypeBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<ExternalTypeAttribute>),
    /// Provided a duplicated postgres type.
    DuplicatedPostgresType,
    /// If the provided postgres type is not lowercase.
    NotLowercasePostgresType,
    /// An external trait with the same name has already been added.
    DuplicatedExternalTrait,
}

impl Display for ExternalTypeBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalTypeBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            ExternalTypeBuilderError::DuplicatedPostgresType => {
                write!(f, "Provided a duplicated postgres type")
            }
            ExternalTypeBuilderError::NotLowercasePostgresType => {
                write!(f, "Provided a postgres type which is not lowercase")
            }
            ExternalTypeBuilderError::DuplicatedExternalTrait => {
                write!(f, "An external trait with the same name has already been added")
            }
        }
    }
}

impl core::error::Error for ExternalTypeBuilderError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            ExternalTypeBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl ExternalTypeBuilder {
    /// Sets the diesel type defined within the crate compatible with the given
    /// postgres type.
    pub fn diesel_type(mut self, diesel_type: syn::Type) -> Self {
        self.diesel_type = Some(diesel_type);
        self
    }

    /// Sets the rust type defined within the crate compatible with the given
    /// postgres type.
    pub fn rust_type(mut self, rust_type: syn::Type) -> Self {
        self.rust_type = Some(rust_type);
        self
    }

    /// Adds a postgres type which is compatible with the diesel and rust types
    /// defined within the crate.
    ///
    /// # Arguments
    /// * `postgres_type` - The postgres type to add.
    ///
    /// # Errors
    ///
    /// Returns an error if the postgres type is already present.
    pub fn postgres_type(
        mut self,
        postgres_type: &'static str,
    ) -> Result<Self, ExternalTypeBuilderError> {
        if self.postgres_types.contains(&postgres_type) {
            return Err(ExternalTypeBuilderError::DuplicatedPostgresType);
        }
        if postgres_type != postgres_type.to_lowercase() {
            return Err(ExternalTypeBuilderError::NotLowercasePostgresType);
        }
        self.postgres_types.push(postgres_type);
        Ok(self)
    }

    /// Sets that the current type supports copy.
    pub fn supports_copy(mut self) -> Self {
        self.traits.set_supports(Trait::Copy);
        self.traits.set_supports(Trait::Clone);
        self
    }

    /// Sets that the current type supports clone.
    pub fn supports_clone(mut self) -> Self {
        self.traits.set_supports(Trait::Clone);
        self
    }

    /// Sets that the current type supports default.
    pub fn supports_default(mut self) -> Self {
        self.traits.set_supports(Trait::Default);
        self
    }

    /// Sets that the current type supports hash.
    pub fn supports_hash(mut self) -> Self {
        self.traits.set_supports(Trait::Hash);
        self
    }

    /// Sets that the current type supports partial eq.
    pub fn supports_partial_eq(mut self) -> Self {
        self.traits.set_supports(Trait::PartialEq);
        self
    }

    /// Sets that the current type supports eq.
    pub fn supports_eq(mut self) -> Self {
        self.traits.set_supports(Trait::Eq);
        self.traits.set_supports(Trait::PartialEq);
        self
    }

    /// Sets that the current type supports partial ord.
    pub fn supports_partial_ord(mut self) -> Self {
        self.traits.set_supports(Trait::PartialOrd);
        self.traits.set_supports(Trait::PartialEq);
        self
    }

    /// Sets that the current type supports ord.
    pub fn supports_ord(mut self) -> Self {
        self.traits.set_supports(Trait::Ord);
        self.traits.set_supports(Trait::PartialOrd);
        self.traits.set_supports(Trait::Eq);
        self.traits.set_supports(Trait::PartialEq);
        self
    }

    /// Sets that the current type supports [`Serialize`](serde::Serialize).
    pub fn supports_serialize(self) -> Result<Self, ExternalTypeBuilderError> {
        let serde = ExternalCrate::serde();
        self.add_external_trait(
            serde
                .external_trait_ref("Serialize")
                .expect("Serde must contain the `Serialize` trait"),
        )
    }

    /// Sets that the current type supports [`Deserialize`](serde::Deserialize).
    pub fn supports_deserialize(self) -> Result<Self, ExternalTypeBuilderError> {
        let serde = ExternalCrate::serde();
        self.add_external_trait(
            serde
                .external_trait_ref("Deserialize")
                .expect("Serde must contain the `Deserialize` trait"),
        )
    }

    /// Sets support for both [`Serialize`](serde::Serialize) and
    /// [`Deserialize`](serde::Deserialize).
    pub fn supports_serde(mut self) -> Result<Self, ExternalTypeBuilderError> {
        self = self.supports_serialize()?;
        self.supports_deserialize()
    }

    /// Sets that the current type supports debug.
    pub fn supports_debug(mut self) -> Self {
        self.traits.set_supports(Trait::Debug);
        self
    }

    /// Adds several postgres types which are compatible with the diesel and
    /// rust types defined within the crate.
    ///
    /// # Arguments
    /// * `postgres_types` - The postgres types to add.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the postgres types is already present.
    pub fn postgres_types<I>(mut self, postgres_types: I) -> Result<Self, ExternalTypeBuilderError>
    where
        I: IntoIterator<Item = &'static str>,
    {
        for postgres_type in postgres_types {
            self = self.postgres_type(postgres_type)?;
        }
        Ok(self)
    }

    /// Adds an external trait implemented by the type.
    ///
    /// # Arguments
    /// * `external_trait` - The external trait to add.
    ///
    /// # Errors
    ///
    /// Returns an error if an external trait with the same name has already
    /// been added.
    pub fn add_external_trait(
        mut self,
        external_trait: ExternalTraitRef,
    ) -> Result<Self, ExternalTypeBuilderError> {
        if self.external_traits.iter().any(|t| t == &external_trait) {
            return Err(ExternalTypeBuilderError::DuplicatedExternalTrait);
        }
        self.external_traits.push(external_trait);
        Ok(self)
    }

    /// Adds multiple external traits implemented by the type.
    ///
    /// # Arguments
    /// * `external_traits` - The external traits to add.
    ///
    /// # Errors
    ///
    /// Returns an error if any external trait with the same name has already
    /// been added.
    pub fn add_external_traits<I>(
        mut self,
        external_traits: I,
    ) -> Result<Self, ExternalTypeBuilderError>
    where
        I: IntoIterator<Item = ExternalTraitRef>,
    {
        for external_trait in external_traits {
            self = self.add_external_trait(external_trait)?;
        }
        Ok(self)
    }

    /// Adds a generic parameter to the type.
    ///
    /// # Arguments
    /// * `generic` - The generic parameter to add.
    pub fn generic(mut self, generic: syn::Ident) -> Self {
        if !self.generics.contains(&generic) {
            self.generics.push(generic);
        }

        self
    }

    /// Adds multiple generic parameters to the type.
    ///
    /// # Arguments
    /// * `generics` - The generic parameters to add.
    pub fn generics<I>(mut self, generics: I) -> Self
    where
        I: IntoIterator<Item = syn::Ident>,
    {
        for generic in generics {
            self = self.generic(generic);
        }
        self
    }

    /// Sets a default value for a generic parameter of the type.
    ///
    /// # Arguments
    /// * `generic` - The generic parameter.
    /// * `default` - The default value for the generic parameter.
    pub fn generic_default(mut self, generic: syn::Ident, default: DataVariantRef) -> Self {
        self.generic_defaults.insert(generic, default);
        self
    }
}

impl Display for ExternalTypeAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalTypeAttribute::DieselType => write!(f, "diesel_type"),
            ExternalTypeAttribute::RustType => write!(f, "rust_type"),
            ExternalTypeAttribute::PostgresTypes => write!(f, "postgres_types"),
            ExternalTypeAttribute::Traits => write!(f, "traits"),
            ExternalTypeAttribute::ExternalTraits => write!(f, "external_traits"),
        }
    }
}

impl Attributed for ExternalTypeBuilder {
    type Attribute = ExternalTypeAttribute;
}

impl IsCompleteBuilder for ExternalTypeBuilder {
    fn is_complete(&self) -> bool {
        self.diesel_type.is_some() && self.rust_type.is_some() && !self.postgres_types.is_empty()
    }
}

impl Builder for ExternalTypeBuilder {
    type Error = BuilderError<ExternalTypeAttribute>;
    type Object = ExternalType;

    fn build(mut self) -> Result<Self::Object, Self::Error> {
        let generic_defaults = self
            .generics
            .iter()
            .map(|g| self.generic_defaults.remove(g))
            .collect::<Vec<Option<DataVariantRef>>>();

        Ok(ExternalType {
            diesel_type: self.diesel_type,
            rust_type: self
                .rust_type
                .ok_or(BuilderError::IncompleteBuild(ExternalTypeAttribute::RustType))?,
            postgres_types: self.postgres_types,
            traits: self.traits,
            external_traits: self.external_traits,
            generics: self.generics,
            generic_defaults,
        })
    }
}
