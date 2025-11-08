//! Submodule defining a builder for the `InternalStruct` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::internal_struct::{InternalAttribute, InternalStruct};

#[derive(Default)]
/// Builder for the `InternalStruct` struct.
pub struct InternalStructBuilder {
    /// Attributes of the struct.
    attributes: Vec<InternalAttribute>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `InternalStruct` struct.
pub enum InternalStructAttribute {
    /// Attributes of the struct.
    Attributes,
}

impl Display for InternalStructAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InternalStructAttribute::Attributes => write!(f, "attributes"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of an
/// `InternalStruct`.
pub enum InternalStructBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<InternalStructAttribute>),
    /// An attribute with the same identifier has already been added.
    DuplicatedAttribute,
}

impl Display for InternalStructBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InternalStructBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            InternalStructBuilderError::DuplicatedAttribute => {
                write!(f, "An attribute with the same identifier has already been added")
            }
        }
    }
}

impl Error for InternalStructBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InternalStructBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl InternalStructBuilder {
    /// Adds an attribute to the struct.
    ///
    /// # Arguments
    /// * `attribute` - The attribute to add.
    pub fn attribute(
        mut self,
        attribute: InternalAttribute,
    ) -> Result<Self, InternalStructBuilderError> {
        if self.attributes.iter().any(|a| a.ident() == attribute.ident()) {
            return Err(InternalStructBuilderError::DuplicatedAttribute);
        }
        self.attributes.push(attribute);
        Ok(self)
    }

    /// Adds multiple attributes to the struct.
    ///
    /// # Arguments
    /// * `attributes` - The attributes to add.
    pub fn attributes<I>(mut self, attributes: I) -> Result<Self, InternalStructBuilderError>
    where
        I: IntoIterator<Item = InternalAttribute>,
    {
        for attribute in attributes {
            self = self.attribute(attribute)?;
        }
        Ok(self)
    }
}

impl Attributed for InternalStructBuilder {
    type Attribute = InternalStructAttribute;
}

impl IsCompleteBuilder for InternalStructBuilder {
    fn is_complete(&self) -> bool {
        // A struct can be empty, so it's always complete
        true
    }
}

impl Builder for InternalStructBuilder {
    type Error = BuilderError<InternalStructAttribute>;
    type Object = InternalStruct;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(InternalStruct { attributes: self.attributes })
    }
}
