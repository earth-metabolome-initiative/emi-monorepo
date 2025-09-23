//! Submodule providing a builder for the `RequiredCrate` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::{RequiredCrate, RequiredType};

#[derive(Default)]
/// Builder for the `RequiredCrate` struct.
pub struct RequiredCrateBuilder {
    /// The name of the crate.
    name: Option<String>,
    /// The types provided by the crate.
    types: Vec<RequiredType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `RequiredCrate` struct.
pub enum RequiredCrateAttribute {
    /// The name of the crate.
    Name,
    /// The types provided by the crate.
    Types,
}

impl Display for RequiredCrateAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequiredCrateAttribute::Name => write!(f, "name"),
            RequiredCrateAttribute::Types => write!(f, "types"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of a `RequiredCrate`.
pub enum RequiredCrateBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<RequiredCrateAttribute>),
    /// The name of the crate is invalid.
    InvalidName,
    /// A type handling the same postgres type has already been added to the crate.
    DuplicatedPostgresType,
}

impl Display for RequiredCrateBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequiredCrateBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            RequiredCrateBuilderError::InvalidName => write!(f, "Invalid crate name"),
            RequiredCrateBuilderError::DuplicatedPostgresType => {
                write!(
                    f,
                    "A type handling the same postgres type has already been added to the crate"
                )
            }
        }
    }
}

impl Error for RequiredCrateBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RequiredCrateBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl RequiredCrateBuilder {
    /// Sets the name of the crate.
    ///
    /// # Arguments
    /// * `name` - The name of the crate.
    pub fn name(mut self, name: String) -> Result<Self, RequiredCrateBuilderError> {
        if name.trim().is_empty() || name.contains(' ') {
            return Err(RequiredCrateBuilderError::InvalidName);
        }
        self.name = Some(name);
        Ok(self)
    }

    /// Adds a type provided by the crate.
    ///
    /// # Arguments
    /// * `required_type` - The type provided by the crate.
    pub fn add_type(
        mut self,
        required_type: RequiredType,
    ) -> Result<Self, RequiredCrateBuilderError> {
        for postgres_type in required_type.postgres_types() {
            if self.types.iter().any(|t| t.is_compatible_with(postgres_type)) {
                return Err(RequiredCrateBuilderError::DuplicatedPostgresType);
            }
        }
        self.types.push(required_type);
        Ok(self)
    }

    /// Adds the provided types to the crate.
    ///
    /// # Arguments
    /// * `required_types` - The types provided by the crate.
    pub fn add_types<I>(mut self, required_types: I) -> Result<Self, RequiredCrateBuilderError>
    where
        I: IntoIterator<Item = RequiredType>,
    {
        for required_type in required_types {
            self = self.add_type(required_type)?;
        }
        Ok(self)
    }
}

impl Attributed for RequiredCrateBuilder {
    type Attribute = RequiredCrateAttribute;
}

impl IsCompleteBuilder for RequiredCrateBuilder {
    fn is_complete(&self) -> bool {
        self.name.is_some() && !self.types.is_empty()
    }
}

impl Builder for RequiredCrateBuilder {
    type Error = BuilderError<RequiredCrateAttribute>;
    type Object = RequiredCrate;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(RequiredCrate {
            name: self.name.ok_or(BuilderError::IncompleteBuild(RequiredCrateAttribute::Name))?,
            types: if self.types.is_empty() {
                return Err(BuilderError::IncompleteBuild(RequiredCrateAttribute::Types));
            } else {
                self.types
            },
        })
    }
}
