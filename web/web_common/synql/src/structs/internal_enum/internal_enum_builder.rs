//! Submodule defining a builder for the `InternalEnum` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::internal_enum::{InternalEnum, InternalVariant};

#[derive(Default)]
/// Builder for the `InternalEnum` struct.
pub struct InternalEnumBuilder {
    /// Variants of the enum.
    variants: Vec<InternalVariant>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `InternalEnum` struct.
pub enum InternalEnumAttribute {
    /// Variants of the enum.
    Variants,
}

impl Display for InternalEnumAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InternalEnumAttribute::Variants => write!(f, "variants"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of an
/// `InternalEnum`.
pub enum InternalEnumBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<InternalEnumAttribute>),
    /// A variant with the same identifier has already been added.
    DuplicatedVariant,
}

impl From<BuilderError<InternalEnumAttribute>> for InternalEnumBuilderError {
    fn from(e: BuilderError<InternalEnumAttribute>) -> Self {
        InternalEnumBuilderError::Builder(e)
    }
}

impl Display for InternalEnumBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InternalEnumBuilderError::Builder(e) => write!(f, "Builder error: {e}"),
            InternalEnumBuilderError::DuplicatedVariant => {
                write!(f, "A variant with the same identifier has already been added")
            }
        }
    }
}

impl Error for InternalEnumBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InternalEnumBuilderError::Builder(e) => Some(e),
            InternalEnumBuilderError::DuplicatedVariant => None,
        }
    }
}

impl InternalEnumBuilder {
    /// Adds a variant to the enum.
    ///
    /// # Arguments
    /// * `variant` - The variant to add.
    pub fn variant(mut self, variant: InternalVariant) -> Result<Self, InternalEnumBuilderError> {
        if self.variants.iter().any(|v| v.ident() == variant.ident()) {
            return Err(InternalEnumBuilderError::DuplicatedVariant);
        }
        self.variants.push(variant);
        Ok(self)
    }

    /// Adds multiple variants to the enum.
    ///
    /// # Arguments
    /// * `variants` - The variants to add.
    pub fn variants<I>(mut self, variants: I) -> Result<Self, InternalEnumBuilderError>
    where
        I: IntoIterator<Item = InternalVariant>,
    {
        for variant in variants {
            self = self.variant(variant)?;
        }
        Ok(self)
    }
}

impl Attributed for InternalEnumBuilder {
    type Attribute = InternalEnumAttribute;
}

impl IsCompleteBuilder for InternalEnumBuilder {
    fn is_complete(&self) -> bool {
        // An enum can be empty (though not ideal), so it's always complete
        true
    }
}

impl Builder for InternalEnumBuilder {
    type Error = InternalEnumBuilderError;
    type Object = InternalEnum;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(InternalEnum { variants: self.variants })
    }
}
