//! Submodule defining a builder for the `InternalAttribute` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{
    Publicness, internal_data::DataVariantRef, internal_struct::InternalAttribute,
};

#[derive(Default)]
/// Builder for the `InternalAttribute` struct.
pub struct InternalAttributeBuilder<'data> {
    /// Publicness of the attribute.
    pubness: Option<Publicness>,
    /// The documentation of the attribute.
    documentation: Option<String>,
    /// Name of the attribute.
    name: Option<String>,
    /// Type of the attribute.
    ty: Option<DataVariantRef<'data>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `InternalAttribute` struct.
pub enum InternalAttributeAttribute {
    /// Publicness of the attribute.
    Pubness,
    /// The documentation of the attribute.
    Documentation,
    /// Name of the attribute.
    Name,
    /// Type of the attribute.
    Type,
}

impl Display for InternalAttributeAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalAttributeAttribute::Pubness => write!(f, "pubness"),
            InternalAttributeAttribute::Documentation => write!(f, "documentation"),
            InternalAttributeAttribute::Name => write!(f, "name"),
            InternalAttributeAttribute::Type => write!(f, "type"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of an
/// `InternalAttribute`.
pub enum InternalAttributeBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<InternalAttributeAttribute>),
    /// The name of the attribute is invalid.
    InvalidName,
    /// The documentation is invalid (empty or whitespace only).
    InvalidDocumentation,
}

impl From<BuilderError<InternalAttributeAttribute>> for InternalAttributeBuilderError {
    fn from(e: BuilderError<InternalAttributeAttribute>) -> Self {
        InternalAttributeBuilderError::Builder(e)
    }
}

impl Display for InternalAttributeBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalAttributeBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            InternalAttributeBuilderError::InvalidName => {
                write!(f, "Invalid attribute name (empty or whitespace only)")
            }
            InternalAttributeBuilderError::InvalidDocumentation => {
                write!(f, "Invalid attribute documentation (empty or whitespace only)")
            }
        }
    }
}

impl Error for InternalAttributeBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InternalAttributeBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl<'data> InternalAttributeBuilder<'data> {
    /// Sets the publicness of the attribute.
    ///
    /// # Arguments
    /// * `pubness` - The publicness of the attribute.
    pub fn pubness(mut self, pubness: Publicness) -> Self {
        self.pubness = Some(pubness);
        self
    }

    /// Sets the attribute as public.
    pub fn public(mut self) -> Self {
        self.pubness = Some(Publicness::Public);
        self
    }

    /// Sets the attribute as private.
    pub fn private(mut self) -> Self {
        self.pubness = Some(Publicness::Private);
        self
    }

    /// Sets the documentation of the attribute.
    ///
    /// # Arguments
    /// * `documentation` - The documentation of the attribute.
    pub fn documentation<S: ToString>(
        mut self,
        documentation: S,
    ) -> Result<Self, InternalAttributeBuilderError> {
        let documentation = documentation.to_string();
        if documentation.trim().is_empty() {
            return Err(InternalAttributeBuilderError::InvalidDocumentation);
        }
        self.documentation = Some(documentation);
        Ok(self)
    }

    /// Sets the name of the attribute.
    ///
    /// # Arguments
    /// * `name` - The name of the attribute.
    pub fn name<S: ToString>(mut self, name: S) -> Result<Self, InternalAttributeBuilderError> {
        let name = name.to_string();
        if name.trim().is_empty() {
            return Err(InternalAttributeBuilderError::InvalidName);
        }
        self.name = Some(name);
        Ok(self)
    }

    /// Sets the type of the attribute.
    ///
    /// # Arguments
    /// * `ty` - The type of the attribute.
    pub fn ty<V>(mut self, ty: V) -> Self
    where
        V: Into<DataVariantRef<'data>>,
    {
        self.ty = Some(ty.into());
        self
    }
}

impl Attributed for InternalAttributeBuilder<'_> {
    type Attribute = InternalAttributeAttribute;
}

impl IsCompleteBuilder for InternalAttributeBuilder<'_> {
    fn is_complete(&self) -> bool {
        self.pubness.is_some() && self.name.is_some() && self.ty.is_some()
    }
}

impl<'data> Builder for InternalAttributeBuilder<'data> {
    type Error = InternalAttributeBuilderError;
    type Object = InternalAttribute<'data>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(InternalAttribute {
            pubness: self
                .pubness
                .ok_or(BuilderError::IncompleteBuild(InternalAttributeAttribute::Pubness))?,
            documentation: self.documentation,
            name: self
                .name
                .ok_or(BuilderError::IncompleteBuild(InternalAttributeAttribute::Name))?,
            ty: self.ty.ok_or(BuilderError::IncompleteBuild(InternalAttributeAttribute::Type))?,
        })
    }
}
