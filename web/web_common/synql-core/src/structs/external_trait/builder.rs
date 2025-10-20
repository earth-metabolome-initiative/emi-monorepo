//! Submodule providing a builder for the `ExternalTrait` struct.

use std::fmt::Display;

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::ExternalTrait;

#[derive(Default)]
/// Builder for the `ExternalTrait` struct.
pub struct ExternalTraitBuilder {
    /// The name of the trait.
    name: Option<String>,
    /// The [`syn::Path`](syn::Path) representing the trait
    /// within the external crate.
    path: Option<syn::Path>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `ExternalTrait` struct.
pub enum ExternalTraitAttribute {
    /// The name of the trait.
    Name,
    /// The [`syn::Path`](syn::Path) representing the trait
    /// within the external crate.
    Path,
}

/// Error enumeration which might occur when building a `ExternalTrait`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExternalTraitBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<ExternalTraitAttribute>),
    /// The name of the trait is invalid.
    InvalidName,
}

impl Display for ExternalTraitBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalTraitBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            ExternalTraitBuilderError::InvalidName => write!(f, "Invalid trait name"),
        }
    }
}

impl core::error::Error for ExternalTraitBuilderError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            ExternalTraitBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl ExternalTraitBuilder {
    /// Sets the name of the trait.
    ///
    /// # Arguments
    /// * `name` - The name of the trait.
    ///
    /// # Errors
    ///
    /// Returns an error if the name is empty or contains only whitespace.
    pub fn name(mut self, name: impl Into<String>) -> Result<Self, ExternalTraitBuilderError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(ExternalTraitBuilderError::InvalidName);
        }
        self.name = Some(name);
        Ok(self)
    }

    /// Sets the [`syn::Path`](syn::Path) representing the trait
    /// within the external crate.
    pub fn path(mut self, path: syn::Path) -> Self {
        self.path = Some(path);
        self
    }
}

impl Display for ExternalTraitAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalTraitAttribute::Name => write!(f, "name"),
            ExternalTraitAttribute::Path => write!(f, "path"),
        }
    }
}

impl Attributed for ExternalTraitBuilder {
    type Attribute = ExternalTraitAttribute;
}

impl IsCompleteBuilder for ExternalTraitBuilder {
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.path.is_some()
    }
}

impl Builder for ExternalTraitBuilder {
    type Error = BuilderError<ExternalTraitAttribute>;
    type Object = ExternalTrait;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(ExternalTrait {
            name: self.name.ok_or(BuilderError::IncompleteBuild(ExternalTraitAttribute::Name))?,
            path: self.path.ok_or(BuilderError::IncompleteBuild(ExternalTraitAttribute::Path))?,
        })
    }
}
