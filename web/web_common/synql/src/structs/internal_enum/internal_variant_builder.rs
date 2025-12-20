//! Submodule defining a builder for the `InternalVariant` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};
use syn::Ident;

use crate::structs::{
    Documentation, internal_data::DataVariantRef, internal_enum::InternalVariant,
};

#[derive(Default)]
/// Builder for the `InternalVariant` struct.
pub struct InternalVariantBuilder {
    /// Name of the variant.
    name: Option<Ident>,
    /// Documentation comment of the variant.
    doc: Option<Documentation>,
    /// Type of the variant.
    ty: Option<DataVariantRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `InternalVariant` struct.
pub enum InternalVariantAttribute {
    /// Name of the variant.
    Name,
    /// Documentation comment of the variant.
    Doc,
    /// Type of the variant.
    Type,
}

impl Display for InternalVariantAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InternalVariantAttribute::Name => write!(f, "name"),
            InternalVariantAttribute::Doc => write!(f, "doc"),
            InternalVariantAttribute::Type => write!(f, "type"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of an
/// `InternalVariant`.
pub enum InternalVariantBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<InternalVariantAttribute>),
}

impl From<BuilderError<InternalVariantAttribute>> for InternalVariantBuilderError {
    fn from(e: BuilderError<InternalVariantAttribute>) -> Self {
        InternalVariantBuilderError::Builder(e)
    }
}

impl Display for InternalVariantBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InternalVariantBuilderError::Builder(e) => write!(f, "Builder error: {e}"),
        }
    }
}

impl Error for InternalVariantBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InternalVariantBuilderError::Builder(e) => Some(e),
        }
    }
}

impl InternalVariantBuilder {
    /// Sets the name of the variant.
    ///
    /// # Arguments
    /// * `name` - The name of the variant.
    pub fn name(mut self, name: Ident) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the type of the variant.
    ///
    /// # Arguments
    /// * `ty` - The type of the variant.
    pub fn ty<V>(mut self, ty: V) -> Self
    where
        V: Into<DataVariantRef>,
    {
        self.ty = Some(ty.into());
        self
    }

    /// Sets the documentation comment of the variant.
    ///
    /// # Arguments
    /// * `doc` - The documentation comment of the variant.
    pub fn doc(mut self, doc: Documentation) -> Self {
        self.doc = Some(doc);
        self
    }
}

impl Attributed for InternalVariantBuilder {
    type Attribute = InternalVariantAttribute;
}

impl IsCompleteBuilder for InternalVariantBuilder {
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.doc.is_some()
    }
}

impl Builder for InternalVariantBuilder {
    type Error = InternalVariantBuilderError;
    type Object = InternalVariant;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(InternalVariant {
            name: self.name.ok_or(BuilderError::IncompleteBuild(InternalVariantAttribute::Name))?,
            ty: self.ty,
            doc: self.doc.ok_or(BuilderError::IncompleteBuild(InternalVariantAttribute::Doc))?,
        })
    }
}
