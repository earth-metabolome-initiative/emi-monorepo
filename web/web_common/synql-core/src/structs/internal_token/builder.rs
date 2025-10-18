//! Submodule providing a builder for the `InternalToken` struct.

use std::fmt::Display;

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};
use proc_macro2::TokenStream;

use crate::structs::{
    InternalToken, Publicness,
    external_crate::{ExternalMacroRef, ExternalTraitRef},
};

#[derive(Default)]
/// Builder for the `InternalToken` struct.
pub struct InternalTokenBuilder<'data> {
    /// Publicness of the token stream.
    publicness: Option<Publicness>,
    /// The token stream.
    stream: Option<TokenStream>,
    /// External macros used in the token stream.
    external_macros: Vec<ExternalMacroRef<'data>>,
    /// External traits used in the token stream.
    external_traits: Vec<ExternalTraitRef<'data>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `InternalToken` struct.
pub enum InternalTokenAttribute {
    /// Publicness of the token stream.
    Publicness,
    /// The token stream.
    Stream,
}

/// Error enumeration which might occur when building a `InternalToken`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InternalTokenBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<InternalTokenAttribute>),
    /// Duplicate external macro detected.
    DuplicateExternalMacro,
    /// Duplicate external trait detected.
    DuplicateExternalTrait,
}

impl Display for InternalTokenBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalTokenBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            InternalTokenBuilderError::DuplicateExternalMacro => {
                write!(f, "Duplicate external macro detected")
            }
            InternalTokenBuilderError::DuplicateExternalTrait => {
                write!(f, "Duplicate external trait detected")
            }
        }
    }
}

impl core::error::Error for InternalTokenBuilderError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            InternalTokenBuilderError::Builder(e) => Some(e),
            InternalTokenBuilderError::DuplicateExternalMacro => None,
            InternalTokenBuilderError::DuplicateExternalTrait => None,
        }
    }
}

impl<'data> InternalTokenBuilder<'data> {
    /// Sets the publicness of the token stream.
    ///
    /// # Arguments
    /// * `publicness` - The publicness of the token stream.
    pub fn publicness(mut self, publicness: Publicness) -> Self {
        self.publicness = Some(publicness);
        self
    }

    /// Set the stream as public.
    pub fn public(mut self) -> Self {
        self.publicness = Some(Publicness::Public);
        self
    }

    /// Sets the token stream.
    ///
    /// # Arguments
    /// * `stream` - The token stream.
    pub fn stream(mut self, stream: TokenStream) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Adds an external macro reference to the token stream.
    ///
    /// # Arguments
    /// * `external_macro` - The external macro reference.
    ///
    /// # Errors
    ///
    /// Returns an error if the external macro is already present.
    pub fn external_macro(
        mut self,
        external_macro: ExternalMacroRef<'data>,
    ) -> Result<Self, InternalTokenBuilderError> {
        if self.external_macros.contains(&external_macro) {
            return Err(InternalTokenBuilderError::DuplicateExternalMacro);
        }
        self.external_macros.push(external_macro);
        Ok(self)
    }

    /// Sets the external macros used in the token stream.
    ///
    /// # Arguments
    /// * `external_macros` - The external macros.
    ///
    /// # Errors
    ///
    /// Returns an error if any duplicate external macros are detected.
    pub fn external_macros(
        mut self,
        external_macros: Vec<ExternalMacroRef<'data>>,
    ) -> Result<Self, InternalTokenBuilderError> {
        // Check for duplicates within the provided vector
        for (i, macro1) in external_macros.iter().enumerate() {
            for macro2 in external_macros.iter().skip(i + 1) {
                if macro1 == macro2 {
                    return Err(InternalTokenBuilderError::DuplicateExternalMacro);
                }
            }
        }
        self.external_macros = external_macros;
        Ok(self)
    }

    /// Adds an external trait reference to the token stream.
    ///
    /// # Arguments
    /// * `external_trait` - The external trait reference.
    ///
    /// # Errors
    ///
    /// Returns an error if the external trait is already present.
    pub fn external_trait(
        mut self,
        external_trait: ExternalTraitRef<'data>,
    ) -> Result<Self, InternalTokenBuilderError> {
        if self.external_traits.contains(&external_trait) {
            return Err(InternalTokenBuilderError::DuplicateExternalTrait);
        }
        self.external_traits.push(external_trait);
        Ok(self)
    }

    /// Sets the external traits used in the token stream.
    ///
    /// # Arguments
    /// * `external_traits` - The external traits.
    ///
    /// # Errors
    ///
    /// Returns an error if any duplicate external traits are detected.
    pub fn external_traits(
        mut self,
        external_traits: Vec<ExternalTraitRef<'data>>,
    ) -> Result<Self, InternalTokenBuilderError> {
        // Check for duplicates within the provided vector
        for (i, trait1) in external_traits.iter().enumerate() {
            for trait2 in external_traits.iter().skip(i + 1) {
                if trait1 == trait2 {
                    return Err(InternalTokenBuilderError::DuplicateExternalTrait);
                }
            }
        }
        self.external_traits = external_traits;
        Ok(self)
    }
}

impl Display for InternalTokenAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalTokenAttribute::Publicness => write!(f, "publicness"),
            InternalTokenAttribute::Stream => write!(f, "stream"),
        }
    }
}

impl<'data> Attributed for InternalTokenBuilder<'data> {
    type Attribute = InternalTokenAttribute;
}

impl<'data> IsCompleteBuilder for InternalTokenBuilder<'data> {
    fn is_complete(&self) -> bool {
        self.publicness.is_some() && self.stream.is_some()
    }
}

impl<'data> Builder for InternalTokenBuilder<'data> {
    type Error = BuilderError<InternalTokenAttribute>;
    type Object = InternalToken<'data>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(InternalToken {
            publicness: self
                .publicness
                .ok_or(BuilderError::IncompleteBuild(InternalTokenAttribute::Publicness))?,
            stream: self
                .stream
                .ok_or(BuilderError::IncompleteBuild(InternalTokenAttribute::Stream))?,
            external_macros: self.external_macros,
            external_traits: self.external_traits,
        })
    }
}
