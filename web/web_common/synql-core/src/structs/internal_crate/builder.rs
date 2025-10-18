//! Submodule defining a builder for the `InternalCrate` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{InternalCrate, InternalModule};

#[derive(Default)]
/// Builder for the `InternalCrate` struct.
pub struct InternalCrateBuilder<'data> {
    /// Name of the crate.
    name: Option<String>,
    /// The root modules of the crate.
    modules: Vec<InternalModule<'data>>,
    /// Crate documentation.
    documentation: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `InternalCrate` struct.
pub enum InternalCrateAttribute {
    /// Name of the crate.
    Name,
    /// The root modules of the crate.
    Modules,
    /// Crate documentation.
    Documentation,
}

impl Display for InternalCrateAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalCrateAttribute::Name => write!(f, "name"),
            InternalCrateAttribute::Modules => write!(f, "modules"),
            InternalCrateAttribute::Documentation => write!(f, "documentation"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of an
/// `InternalCrate`.
pub enum InternalCrateBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<InternalCrateAttribute>),
    /// The name of the crate is invalid.
    InvalidName,
    /// A module with the same name has already been added to the
    /// crate.
    DuplicatedModuleName,
    /// The documentation is invalid (empty or whitespace only).
    InvalidDocumentation,
}

impl Display for InternalCrateBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalCrateBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            InternalCrateBuilderError::InvalidName => write!(f, "Invalid crate name"),
            InternalCrateBuilderError::DuplicatedModuleName => {
                write!(f, "A module with the same name has already been added to the crate")
            }
            InternalCrateBuilderError::InvalidDocumentation => {
                write!(f, "Invalid crate documentation (empty or whitespace only)")
            }
        }
    }
}

impl Error for InternalCrateBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InternalCrateBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl<'data> InternalCrateBuilder<'data> {
    /// Sets the name of the crate.
    ///
    /// # Arguments
    /// * `name` - The name of the crate.
    pub fn name<S: ToString>(mut self, name: S) -> Result<Self, InternalCrateBuilderError> {
        let name = name.to_string();
        if name.trim().is_empty() || name.contains(' ') {
            return Err(InternalCrateBuilderError::InvalidName);
        }
        self.name = Some(name);
        Ok(self)
    }

    /// Sets the documentation of the crate.
    ///
    /// # Arguments
    /// * `documentation` - The documentation of the crate.
    pub fn documentation<S: ToString>(
        mut self,
        documentation: S,
    ) -> Result<Self, InternalCrateBuilderError> {
        let documentation = documentation.to_string();
        if documentation.trim().is_empty() {
            return Err(InternalCrateBuilderError::InvalidDocumentation);
        }
        self.documentation = Some(documentation);
        Ok(self)
    }

    /// Adds a module to the crate.
    ///
    /// # Arguments
    /// * `module` - The module to add.
    pub fn module(
        mut self,
        module: InternalModule<'data>,
    ) -> Result<Self, InternalCrateBuilderError> {
        if self.modules.iter().any(|m| m.ident() == module.ident()) {
            return Err(InternalCrateBuilderError::DuplicatedModuleName);
        }
        self.modules.push(module);
        Ok(self)
    }

    /// Adds multiple modules to the crate.
    ///
    /// # Arguments
    /// * `modules` - The modules to add.
    pub fn modules<I>(mut self, modules: I) -> Result<Self, InternalCrateBuilderError>
    where
        I: IntoIterator<Item = InternalModule<'data>>,
    {
        for module in modules {
            self = self.module(module)?;
        }
        Ok(self)
    }
}

impl Attributed for InternalCrateBuilder<'_> {
    type Attribute = InternalCrateAttribute;
}

impl IsCompleteBuilder for InternalCrateBuilder<'_> {
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.documentation.is_some()
    }
}

impl<'data> Builder for InternalCrateBuilder<'data> {
    type Error = BuilderError<InternalCrateAttribute>;
    type Object = InternalCrate<'data>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(InternalCrate {
            name: self.name.ok_or(BuilderError::IncompleteBuild(InternalCrateAttribute::Name))?,
            modules: self.modules,
            documentation: self
                .documentation
                .ok_or(BuilderError::IncompleteBuild(InternalCrateAttribute::Documentation))?,
        })
    }
}
