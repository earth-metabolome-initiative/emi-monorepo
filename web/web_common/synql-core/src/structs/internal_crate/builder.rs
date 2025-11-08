//! Submodule defining a builder for the `InternalCrate` struct.

use std::{error::Error, fmt::Display, sync::Arc};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{InternalCrate, InternalModule, ModuleDocumentation};

#[derive(Default)]
/// Builder for the `InternalCrate` struct.
pub struct InternalCrateBuilder {
    /// Name of the crate.
    name: Option<String>,
    /// The root modules of the crate.
    modules: Vec<Arc<InternalModule>>,
    /// Crate documentation.
    documentation: Option<ModuleDocumentation>,
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
}

impl Display for InternalCrateBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InternalCrateBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            InternalCrateBuilderError::InvalidName => write!(f, "Invalid crate name"),
            InternalCrateBuilderError::DuplicatedModuleName => {
                write!(f, "A module with the same name has already been added to the crate")
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

impl InternalCrateBuilder {
    /// Sets the name of the crate.
    ///
    /// # Arguments
    /// * `name` - The name of the crate.
    pub fn name<S: ToString>(mut self, name: S) -> Result<Self, InternalCrateBuilderError> {
        let name = name.to_string();
        if name.trim().is_empty()
            || name.contains(' ')
            || !name.chars().all(|c| c.is_alphanumeric() || c == '_')
            || !name.chars().all(|c| c.is_lowercase() || c == '_')
        {
            return Err(InternalCrateBuilderError::InvalidName);
        }
        self.name = Some(name);
        Ok(self)
    }

    /// Sets the documentation of the crate.
    ///
    /// # Arguments
    /// * `documentation` - The documentation of the crate.
    pub fn documentation(mut self, documentation: impl Into<ModuleDocumentation>) -> Self {
        self.documentation = Some(documentation.into());
        self
    }

    /// Adds a module to the crate.
    ///
    /// # Arguments
    /// * `module` - The module to add.
    pub fn module(mut self, module: InternalModule) -> Result<Self, InternalCrateBuilderError> {
        if self.modules.iter().any(|m| m.as_ref() == &module) {
            return Err(InternalCrateBuilderError::DuplicatedModuleName);
        }
        self.modules.push(Arc::new(module));
        Ok(self)
    }

    /// Adds multiple modules to the crate.
    ///
    /// # Arguments
    /// * `modules` - The modules to add.
    pub fn modules<I>(mut self, modules: I) -> Result<Self, InternalCrateBuilderError>
    where
        I: IntoIterator<Item = InternalModule>,
    {
        for module in modules {
            self = self.module(module)?;
        }
        Ok(self)
    }
}

impl Attributed for InternalCrateBuilder {
    type Attribute = InternalCrateAttribute;
}

impl IsCompleteBuilder for InternalCrateBuilder {
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.documentation.is_some()
    }
}

impl Builder for InternalCrateBuilder {
    type Error = BuilderError<InternalCrateAttribute>;
    type Object = InternalCrate;

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
