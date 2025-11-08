//! Submodule providing a builder for the `InternalToken` struct.

use std::fmt::Display;

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::structs::{
    DataVariantRef, InternalToken, Publicness, external_crate::ExternalMacroRef,
    external_trait::TraitVariantRef, internal_data::InternalModuleRef,
};

#[derive(Default)]
/// Builder for the `InternalToken` struct.
pub struct InternalTokenBuilder {
    /// Publicness of the token stream.
    publicness: Publicness,
    /// The token stream.
    stream: Option<TokenStream>,
    /// External macros used in the token stream.
    external_macros: Vec<ExternalMacroRef>,
    /// Traits used in the token stream.
    employed_traits: Vec<TraitVariantRef>,
    /// Traits which are implemented by the token stream.
    implemented_traits: Vec<TraitVariantRef>,
    /// Data used in the token stream.
    data: Vec<DataVariantRef>,
    /// Internal modules from other crates in the same workspace which are used
    /// in the token stream.
    internal_modules: Vec<InternalModuleRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `InternalToken` struct.
pub enum InternalTokenAttribute {
    /// Publicness of the token stream.
    Publicness,
    /// The token stream.
    Stream,
    /// External macros used in the token stream.
    ExternalMacros,
    /// External traits used in the token stream.
    ExternalTraits,
    /// Internal data used in the token stream.
    InternalData,
    /// Internal modules used in the token stream.
    InternalModules,
}

/// Error enumeration which might occur when building a `InternalToken`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InternalTokenBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<InternalTokenAttribute>),
    /// The provided external macro does not appear in the token stream.
    ExternalMacroNotFound(String),
    /// The provided external trait does not appear in the token stream.
    TraitNotFound(String),
    /// The provided internal data does not appear in the token stream.
    InternalDataNotFound(String),
    /// The provided internal module does not appear in the token stream.
    InternalModuleNotFound(String),
}

impl From<BuilderError<InternalTokenAttribute>> for InternalTokenBuilderError {
    fn from(value: BuilderError<InternalTokenAttribute>) -> Self {
        InternalTokenBuilderError::Builder(value)
    }
}

impl Display for InternalTokenBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalTokenBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            InternalTokenBuilderError::ExternalMacroNotFound(name) => {
                write!(f, "External macro '{name}' not found in token stream")
            }
            InternalTokenBuilderError::TraitNotFound(name) => {
                write!(f, "Trait '{name}' not found in token stream")
            }
            InternalTokenBuilderError::InternalDataNotFound(name) => {
                write!(f, "Internal data '{name}' not found in token stream")
            }
            InternalTokenBuilderError::InternalModuleNotFound(name) => {
                write!(f, "Internal module '{name}' not found in token stream")
            }
        }
    }
}

impl core::error::Error for InternalTokenBuilderError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            InternalTokenBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl InternalTokenBuilder {
    /// Sets the publicness of the token stream.
    ///
    /// # Arguments
    /// * `publicness` - The publicness of the token stream.
    pub fn publicness(mut self, publicness: Publicness) -> Self {
        self.publicness = publicness;
        self
    }

    /// Set the stream as public.
    pub fn public(mut self) -> Self {
        self.publicness = Publicness::Public;
        self
    }

    /// Set the stream as private.
    pub fn private(mut self) -> Self {
        self.publicness = Publicness::Private;
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

    /// Inherits properties from another `InternalToken`.
    pub fn inherit(mut self, other: InternalToken) -> Self {
        if other.is_public() {
            self = self.public();
        }
        self = self.external_macros(other.external_macros);
        self = self.employed_traits(other.employed_traits);
        self = self.implemented_traits(other.implemented_traits);
        self = self.datas(other.data);
        self = self.internal_modules(other.internal_modules);

        self
    }

    /// Inherits properties from several `InternalToken`s
    pub fn inherits<I>(mut self, others: I) -> Self
    where
        I: IntoIterator<Item = InternalToken>,
    {
        for other in others {
            self = self.inherit(other);
        }
        self
    }

    /// Adds an external macro reference to the token stream.
    ///
    /// # Arguments
    /// * `external_macro` - The external macro reference.
    pub fn external_macro(mut self, external_macro: ExternalMacroRef) -> Self {
        if !self.external_macros.contains(&external_macro) {
            self.external_macros.push(external_macro);
        }
        self
    }

    /// Sets the external macros used in the token stream.
    ///
    /// # Arguments
    /// * `external_macros` - The external macros.
    ///
    /// # Errors
    ///
    /// Returns an error if any duplicate external macros are detected.
    pub fn external_macros<I>(mut self, external_macros: I) -> Self
    where
        I: IntoIterator<Item = ExternalMacroRef>,
    {
        for external_macro in external_macros {
            self = self.external_macro(external_macro);
        }
        self
    }

    /// Adds a trait employed by the token stream.
    ///
    /// # Arguments
    /// * `employed_trait` - The employed trait.
    pub fn employed_trait(mut self, employed_trait: TraitVariantRef) -> Self {
        if !self.employed_traits.contains(&employed_trait) {
            self.employed_traits.push(employed_trait);
        }
        self
    }

    /// Adds several traits employed by the token stream.
    pub fn employed_traits<I>(mut self, employed_traits: I) -> Self
    where
        I: IntoIterator<Item = TraitVariantRef>,
    {
        for employed_trait in employed_traits {
            self = self.employed_trait(employed_trait);
        }
        self
    }

    /// Adds a trait implemented by the token stream.
    ///
    /// # Arguments
    /// * `implemented_trait` - The implemented trait.
    ///
    /// # Errors
    ///
    /// * If the implemented trait is already present.
    pub fn implemented_trait(mut self, implemented_trait: TraitVariantRef) -> Self {
        if !self.implemented_traits.contains(&implemented_trait) {
            self.implemented_traits.push(implemented_trait);
        }
        self
    }

    /// Adds several traits implemented by the token stream.
    pub fn implemented_traits<I>(mut self, implemented_traits: I) -> Self
    where
        I: IntoIterator<Item = TraitVariantRef>,
    {
        for implemented_trait in implemented_traits {
            self = self.implemented_trait(implemented_trait);
        }
        self
    }

    /// Adds an internal data reference to the token stream.
    ///
    /// # Arguments
    ///
    /// * `data` - The internal data reference.
    pub fn data(mut self, data: DataVariantRef) -> Self {
        if !self.data.contains(&data) {
            self.data.push(data);
        }
        self
    }

    /// Adds several internal data references to the token stream.
    pub fn datas<I>(mut self, data: I) -> Self
    where
        I: IntoIterator<Item = DataVariantRef>,
    {
        for data_ref in data {
            self = self.data(data_ref);
        }
        self
    }

    /// Adds an internal module reference to the token stream.
    ///
    /// # Arguments
    ///
    /// * `internal_module` - The internal module reference.
    pub fn internal_module(mut self, internal_module: InternalModuleRef) -> Self {
        if !self.internal_modules.contains(&internal_module) {
            self.internal_modules.push(internal_module);
        }
        self
    }

    /// Adds internal module references to the token stream.
    pub fn internal_modules<I>(mut self, internal_modules: I) -> Self
    where
        I: IntoIterator<Item = InternalModuleRef>,
    {
        for internal_module in internal_modules {
            self = self.internal_module(internal_module);
        }
        self
    }
}

impl Display for InternalTokenAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalTokenAttribute::Publicness => write!(f, "publicness"),
            InternalTokenAttribute::Stream => write!(f, "stream"),
            InternalTokenAttribute::ExternalMacros => write!(f, "external_macros"),
            InternalTokenAttribute::ExternalTraits => write!(f, "external_traits"),
            InternalTokenAttribute::InternalData => write!(f, "internal_data"),
            InternalTokenAttribute::InternalModules => write!(f, "internal_modules"),
        }
    }
}

impl Attributed for InternalTokenBuilder {
    type Attribute = InternalTokenAttribute;
}

impl IsCompleteBuilder for InternalTokenBuilder {
    fn is_complete(&self) -> bool {
        self.stream.is_some()
    }
}

impl Builder for InternalTokenBuilder {
    type Error = InternalTokenBuilderError;
    type Object = InternalToken;

    fn build(self) -> Result<Self::Object, Self::Error> {
        let string_token_stream = self.stream.to_token_stream().to_string();
        for external_macro in &self.external_macros {
            let macro_name = external_macro.name();
            if !string_token_stream.contains(macro_name) {
                return Err(InternalTokenBuilderError::ExternalMacroNotFound(
                    macro_name.to_string(),
                ));
            }
        }
        for employed_trait in &self.employed_traits {
            let trait_name = employed_trait.name();
            if !string_token_stream.contains(trait_name) {
                return Err(InternalTokenBuilderError::TraitNotFound(trait_name.to_string()));
            }
        }
        for implemented_trait in &self.implemented_traits {
            let trait_name = implemented_trait.name();
            if !string_token_stream.contains(trait_name) {
                return Err(InternalTokenBuilderError::TraitNotFound(trait_name.to_string()));
            }
        }
        for data in &self.data {
            if !string_token_stream.contains(&data.format_with_generics().to_string()) {
                return Err(InternalTokenBuilderError::InternalDataNotFound(
                    data.format_with_generics().to_string(),
                ));
            }
        }
        for internal_module in &self.internal_modules {
            let module_name = internal_module.name();
            if !string_token_stream.contains(module_name) {
                return Err(InternalTokenBuilderError::InternalModuleNotFound(
                    module_name.to_string(),
                ));
            }
        }
        Ok(InternalToken {
            publicness: self.publicness,
            stream: self
                .stream
                .ok_or(BuilderError::IncompleteBuild(InternalTokenAttribute::Stream))?,
            external_macros: self.external_macros,
            employed_traits: self.employed_traits,
            implemented_traits: self.implemented_traits,
            data: self.data,
            internal_modules: self.internal_modules,
        })
    }
}
