//! Submodule providing a builder for the `ExternalMacro` struct.

use std::fmt::Display;

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::ExternalMacro;

#[derive(Default)]
/// Builder for the `ExternalMacro` struct.
pub struct ExternalMacroBuilder {
    /// The name of the macro.
    name: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `ExternalMacro` struct.
pub enum ExternalMacroAttribute {
    /// The name of the macro.
    Name,
}

/// Error enumeration which might occur when building a `ExternalMacro`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExternalMacroBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<ExternalMacroAttribute>),
    /// The name of the macro is invalid.
    InvalidName,
}

impl Display for ExternalMacroBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalMacroBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            ExternalMacroBuilderError::InvalidName => write!(f, "Invalid macro name"),
        }
    }
}

impl core::error::Error for ExternalMacroBuilderError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            ExternalMacroBuilderError::Builder(e) => Some(e),
            ExternalMacroBuilderError::InvalidName => None,
        }
    }
}

impl ExternalMacroBuilder {
    /// Sets the name of the macro.
    ///
    /// # Arguments
    /// * `name` - The name of the macro.
    ///
    /// # Errors
    ///
    /// Returns an error if the name is empty or contains only whitespace.
    pub fn name(mut self, name: impl Into<String>) -> Result<Self, ExternalMacroBuilderError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(ExternalMacroBuilderError::InvalidName);
        }
        self.name = Some(name);
        Ok(self)
    }
}

impl Display for ExternalMacroAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalMacroAttribute::Name => write!(f, "name"),
        }
    }
}

impl Attributed for ExternalMacroBuilder {
    type Attribute = ExternalMacroAttribute;
}

impl IsCompleteBuilder for ExternalMacroBuilder {
    fn is_complete(&self) -> bool {
        self.name.is_some()
    }
}

impl Builder for ExternalMacroBuilder {
    type Error = BuilderError<ExternalMacroAttribute>;
    type Object = ExternalMacro;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(ExternalMacro {
            name: self.name.ok_or(BuilderError::IncompleteBuild(ExternalMacroAttribute::Name))?,
        })
    }
}
