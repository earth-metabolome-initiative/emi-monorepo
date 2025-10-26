//! Submodule defining a builder for the `Argument` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{Argument, internal_data::DataVariantRef};

#[derive(Default)]
/// Builder for the `Argument` struct.
pub struct ArgumentBuilder<'data> {
    /// Name of the argument.
    name: Option<String>,
    /// Type of the argument.
    arg_type: Option<DataVariantRef<'data>>,
    /// Whether the argument is mutable.
    mutable: bool,
    /// Documentation of the argument.
    documentation: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `Argument` struct.
pub enum ArgumentAttribute {
    /// Name of the argument.
    Name,
    /// Type of the argument.
    ArgType,
    /// Whether the argument is mutable.
    Mutable,
    /// Documentation of the argument.
    Documentation,
}

impl Display for ArgumentAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgumentAttribute::Name => write!(f, "name"),
            ArgumentAttribute::ArgType => write!(f, "arg_type"),
            ArgumentAttribute::Mutable => write!(f, "mutable"),
            ArgumentAttribute::Documentation => write!(f, "documentation"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of an
/// `Argument`.
pub enum ArgumentBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<ArgumentAttribute>),
    /// The name of the argument is invalid.
    InvalidName,
    /// The documentation of the argument is invalid.
    InvalidDocumentation,
}

impl Display for ArgumentBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgumentBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            ArgumentBuilderError::InvalidName => write!(f, "Invalid argument name"),
            ArgumentBuilderError::InvalidDocumentation => {
                write!(f, "Invalid argument documentation")
            }
        }
    }
}

impl Error for ArgumentBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ArgumentBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl<'data> ArgumentBuilder<'data> {
    /// Sets the name of the argument.
    ///
    /// # Arguments
    /// * `name` - The name of the argument.
    pub fn name<S: ToString>(mut self, name: S) -> Result<Self, ArgumentBuilderError> {
        let name = name.to_string();
        if name.trim().is_empty()
            || (!name.starts_with("self")
                && (name.contains(' ') || !name.chars().all(|c| c.is_alphanumeric() || c == '_')))
        {
            return Err(ArgumentBuilderError::InvalidName);
        }
        self.name = Some(name);
        Ok(self)
    }

    /// Sets the type of the argument.
    ///
    /// # Arguments
    /// * `arg_type` - The type of the argument.
    pub fn arg_type<T>(mut self, arg_type: T) -> Self
    where
        T: Into<DataVariantRef<'data>>,
    {
        self.arg_type = Some(arg_type.into());
        self
    }

    /// Sets whether the argument is mutable.
    ///
    /// # Arguments
    /// * `mutable` - Whether the argument is mutable.
    pub fn mutable(mut self, mutable: bool) -> Self {
        self.mutable = mutable;
        self
    }

    /// Sets the documentation of the argument.
    ///
    /// # Arguments
    /// * `documentation` - The documentation of the argument.
    pub fn documentation<S: ToString>(
        mut self,
        documentation: S,
    ) -> Result<Self, ArgumentBuilderError> {
        let documentation = documentation.to_string();
        if documentation.trim().is_empty() {
            return Err(ArgumentBuilderError::InvalidDocumentation);
        }
        self.documentation = Some(documentation);
        Ok(self)
    }
}

impl Attributed for ArgumentBuilder<'_> {
    type Attribute = ArgumentAttribute;
}

impl IsCompleteBuilder for ArgumentBuilder<'_> {
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.arg_type.is_some()
    }
}

impl<'data> Builder for ArgumentBuilder<'data> {
    type Error = BuilderError<ArgumentAttribute>;
    type Object = Argument<'data>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Argument {
            name: self.name.ok_or(BuilderError::IncompleteBuild(ArgumentAttribute::Name))?,
            arg_type: self
                .arg_type
                .ok_or(BuilderError::IncompleteBuild(ArgumentAttribute::ArgType))?,
            mutable: self.mutable,
            documentation: self.documentation,
        })
    }
}
