//! Submodule providing a builder for the `RequiredType` struct.

use std::fmt::Display;

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::{required_type::traits_mask::{Trait, TraitsMask}, RequiredType};

#[derive(Default)]
/// Builder for the `RequiredType` struct.
pub struct RequiredTypeBuilder {
    /// The diesel type defined within the crate compatible with the given postgres type.
    diesel_type: Option<syn::Type>,
    /// The rust type defined within the crate compatible with the given postgres type.
    rust_type: Option<syn::Type>,
    /// The postgres types which are compatible with the diesel and rust types defined within the crate.
    postgres_types: Vec<&'static str>,
    /// Trait mask representing the traits supported by the current type.
    traits: TraitsMask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `RequiredType` struct.
pub enum RequiredTypeAttribute {
    /// The diesel type defined within the crate compatible with the given postgres type.
    DieselType,
    /// The rust type defined within the crate compatible with the given postgres type.
    RustType,
    /// The postgres types which are compatible with the diesel and rust types defined within the crate.
    PostgresTypes,
    /// The trait mask representing the traits.
    Traits,
}

/// Error enumeration which might occur when building a `RequiredType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequiredTypeBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<RequiredTypeAttribute>),
    /// Provided a duplicated postgres type.
    DuplicatedPostgresType,
}

impl Display for RequiredTypeBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequiredTypeBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            RequiredTypeBuilderError::DuplicatedPostgresType => {
                write!(f, "Provided a duplicated postgres type")
            }
        }
    }
}

impl core::error::Error for RequiredTypeBuilderError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            RequiredTypeBuilderError::Builder(e) => Some(e),
            RequiredTypeBuilderError::DuplicatedPostgresType => None,
        }
    }
}

impl RequiredTypeBuilder {
    /// Sets the diesel type defined within the crate compatible with the given postgres type.
    pub fn diesel_type(mut self, diesel_type: syn::Type) -> Self {
        self.diesel_type = Some(diesel_type);
        self
    }

    /// Sets the rust type defined within the crate compatible with the given postgres type.
    pub fn rust_type(mut self, rust_type: syn::Type) -> Self {
        self.rust_type = Some(rust_type);
        self
    }

    /// Adds a postgres type which is compatible with the diesel and rust types defined within the crate.
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
    ) -> Result<Self, RequiredTypeBuilderError> {
        if self.postgres_types.contains(&postgres_type) {
            return Err(RequiredTypeBuilderError::DuplicatedPostgresType);
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

    /// Sets that the current type supports debug.
    pub fn supports_debug(mut self) -> Self {
        self.traits.set_supports(Trait::Debug);
        self
    }

    /// Adds several postgres types which are compatible with the diesel and rust types defined within the crate.
    ///
    /// # Arguments
    /// * `postgres_types` - The postgres types to add.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the postgres types is already present.
    pub fn postgres_types<I>(mut self, postgres_types: I) -> Result<Self, RequiredTypeBuilderError>
    where
        I: IntoIterator<Item = &'static str>,
    {
        for postgres_type in postgres_types {
            self = self.postgres_type(postgres_type)?;
        }
        Ok(self)
    }
}

impl Display for RequiredTypeAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequiredTypeAttribute::DieselType => write!(f, "diesel_type"),
            RequiredTypeAttribute::RustType => write!(f, "rust_type"),
            RequiredTypeAttribute::PostgresTypes => write!(f, "postgres_types"),
            RequiredTypeAttribute::Traits => write!(f, "traits"),
        }
    }
}

impl Attributed for RequiredTypeBuilder {
    type Attribute = RequiredTypeAttribute;
}

impl IsCompleteBuilder for RequiredTypeBuilder {
    fn is_complete(&self) -> bool {
        self.diesel_type.is_some() && self.rust_type.is_some() && !self.postgres_types.is_empty()
    }
}

impl Builder for RequiredTypeBuilder {
    type Error = BuilderError<RequiredTypeAttribute>;
    type Object = RequiredType;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(RequiredType {
            diesel_type: self
                .diesel_type
                .ok_or(BuilderError::IncompleteBuild(RequiredTypeAttribute::DieselType))?,
            rust_type: self
                .rust_type
                .ok_or(BuilderError::IncompleteBuild(RequiredTypeAttribute::RustType))?,
            postgres_types: if self.postgres_types.is_empty() {
                return Err(BuilderError::IncompleteBuild(RequiredTypeAttribute::PostgresTypes));
            } else {
                self.postgres_types
            },
            traits: self.traits,
        })
    }
}
