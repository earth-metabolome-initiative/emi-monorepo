//! Submodule providing a builder for the `ExternalTrait` struct.

use std::{collections::HashMap, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{DataVariantRef, ExternalTrait};

#[derive(Default)]
/// Builder for the `ExternalTrait` struct.
pub struct ExternalTraitBuilder {
    /// The name of the trait.
    name: Option<String>,
    /// The [`syn::Path`](syn::Path) representing the trait
    /// within the external crate.
    path: Option<syn::Path>,
    /// Generic parameters of the trait.
    generics: Vec<syn::GenericParam>,
    /// Default values for the generic parameters of the trait.
    generic_defaults: HashMap<syn::GenericParam, DataVariantRef>,
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
            ExternalTraitBuilderError::Builder(e) => write!(f, "Builder error: {e}"),
            ExternalTraitBuilderError::InvalidName => write!(f, "Invalid trait name"),
        }
    }
}

impl core::error::Error for ExternalTraitBuilderError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            ExternalTraitBuilderError::Builder(e) => Some(e),
            ExternalTraitBuilderError::InvalidName => None,
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

    /// Adds a generic parameter to the trait.
    ///
    /// # Arguments
    /// * `generic` - The generic parameter to add.
    pub fn generic(mut self, generic: syn::GenericParam) -> Self {
        if !self.generics.contains(&generic) {
            self.generics.push(generic);
        }

        self
    }

    /// Adds multiple generic parameters to the trait.
    ///
    /// # Arguments
    /// * `generics` - The generic parameters to add.
    pub fn generics<I>(mut self, generics: I) -> Self
    where
        I: IntoIterator<Item = syn::GenericParam>,
    {
        for generic in generics {
            self = self.generic(generic);
        }
        self
    }

    /// Sets a default value for a generic parameter of the trait.
    ///
    /// # Arguments
    /// * `generic` - The generic parameter.
    /// * `default` - The default value for the generic parameter.
    pub fn generic_default(mut self, generic: syn::GenericParam, default: DataVariantRef) -> Self {
        self.generic_defaults.insert(generic, default);
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

    fn build(mut self) -> Result<Self::Object, Self::Error> {
        let generic_defaults = self
            .generics
            .iter()
            .map(|g| self.generic_defaults.remove(g))
            .collect::<Vec<Option<DataVariantRef>>>();

        Ok(ExternalTrait {
            name: self.name.ok_or(BuilderError::IncompleteBuild(ExternalTraitAttribute::Name))?,
            path: self.path.ok_or(BuilderError::IncompleteBuild(ExternalTraitAttribute::Path))?,
            generics: self.generics,
            generic_defaults,
        })
    }
}
