//! Submodule defining a builder for the `InternalTrait` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{InternalToken, InternalTrait, Publicness};

#[derive(Default)]
/// Builder for the `InternalTrait` struct.
pub struct InternalTraitBuilder<'data> {
    /// Name of the trait.
    name: Option<String>,
    /// Publicness of the trait.
    publicness: Option<Publicness>,
    /// Internal token streams defined within the trait.
    internal_tokens: Vec<InternalToken<'data>>,
    /// Trait documentation.
    documentation: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `InternalTrait` struct.
pub enum InternalTraitAttribute {
    /// Name of the trait.
    Name,
    /// Publicness of the trait.
    Publicness,
    /// Internal token streams defined within the trait.
    InternalTokens,
    /// Trait documentation.
    Documentation,
}

impl Display for InternalTraitAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalTraitAttribute::Name => write!(f, "name"),
            InternalTraitAttribute::Publicness => write!(f, "publicness"),
            InternalTraitAttribute::InternalTokens => write!(f, "internal_tokens"),
            InternalTraitAttribute::Documentation => write!(f, "documentation"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of an
/// `InternalTrait`.
pub enum InternalTraitBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<InternalTraitAttribute>),
    /// The name of the trait is invalid.
    InvalidName,
    /// The documentation is invalid (empty or whitespace only).
    InvalidDocumentation,
}

impl Display for InternalTraitBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalTraitBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            InternalTraitBuilderError::InvalidName => write!(f, "Invalid trait name"),
            InternalTraitBuilderError::InvalidDocumentation => {
                write!(f, "Invalid trait documentation (empty or whitespace only)")
            }
        }
    }
}

impl Error for InternalTraitBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InternalTraitBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl<'data> InternalTraitBuilder<'data> {
    /// Sets the name of the trait.
    ///
    /// # Arguments
    /// * `name` - The name of the trait.
    pub fn name<S: ToString>(mut self, name: S) -> Result<Self, InternalTraitBuilderError> {
        let name = name.to_string();
        if name.trim().is_empty()
            || name.contains(' ')
            || !name.chars().all(|c| c.is_alphanumeric() || c == '_')
        {
            return Err(InternalTraitBuilderError::InvalidName);
        }
        self.name = Some(name);
        Ok(self)
    }

    /// Sets the publicness of the trait.
    ///
    /// # Arguments
    /// * `publicness` - The publicness of the trait.
    pub fn publicness(mut self, publicness: Publicness) -> Self {
        self.publicness = Some(publicness);
        self
    }

    /// Sets the trait as public.
    pub fn public(mut self) -> Self {
        self.publicness = Some(Publicness::Public);
        self
    }

    /// Sets the trait as private.
    pub fn private(mut self) -> Self {
        self.publicness = Some(Publicness::Private);
        self
    }

    /// Sets the documentation of the trait.
    ///
    /// # Arguments
    /// * `documentation` - The documentation of the trait.
    pub fn documentation<S: ToString>(
        mut self,
        documentation: S,
    ) -> Result<Self, InternalTraitBuilderError> {
        let documentation = documentation.to_string();
        if documentation.trim().is_empty() {
            return Err(InternalTraitBuilderError::InvalidDocumentation);
        }
        self.documentation = Some(documentation);
        Ok(self)
    }

    /// Adds an internal token stream to the trait.
    ///
    /// # Arguments
    /// * `internal_token` - The internal token stream to add.
    pub fn internal_token(mut self, internal_token: InternalToken<'data>) -> Self {
        self.internal_tokens.push(internal_token);
        self
    }
}

impl Attributed for InternalTraitBuilder<'_> {
    type Attribute = InternalTraitAttribute;
}

impl IsCompleteBuilder for InternalTraitBuilder<'_> {
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.publicness.is_some() && self.documentation.is_some()
    }
}

impl<'data> Builder for InternalTraitBuilder<'data> {
    type Error = BuilderError<InternalTraitAttribute>;
    type Object = InternalTrait<'data>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(InternalTrait {
            name: self.name.ok_or(BuilderError::IncompleteBuild(InternalTraitAttribute::Name))?,
            publicness: self
                .publicness
                .ok_or(BuilderError::IncompleteBuild(InternalTraitAttribute::Publicness))?,
            internal_tokens: self.internal_tokens,
            documentation: self
                .documentation
                .ok_or(BuilderError::IncompleteBuild(InternalTraitAttribute::Documentation))?,
        })
    }
}
