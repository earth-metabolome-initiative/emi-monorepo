//! Submodule defining a builder for the `InternalModule` struct.

use std::{error::Error, fmt::Display, sync::Arc};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{
    InternalData, InternalModule, InternalToken, InternalTrait, ModuleDocumentation, Publicness,
};

#[derive(Default)]
/// Builder for the `InternalModule` struct.
pub struct InternalModuleBuilder {
    /// Name of the module.
    name: Option<String>,
    /// The submodules it contains.
    submodules: Vec<InternalModule>,
    /// Publicness of the module.
    publicness: Option<Publicness>,
    /// Data structs defined within the module.
    data: Vec<Arc<InternalData>>,
    /// Internal traits defined within the module.
    internal_traits: Vec<Arc<InternalTrait>>,
    /// Other token streams defined within the module.
    internal_tokens: Vec<InternalToken>,
    /// Module documentation.
    documentation: Option<ModuleDocumentation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `InternalModule` struct.
pub enum InternalModuleAttribute {
    /// Name of the module.
    Name,
    /// The submodules it contains.
    Submodules,
    /// Publicness of the module.
    Publicness,
    /// Data structs defined within the module.
    Data,
    /// Internal traits defined within the module.
    InternalTraits,
    /// Other token streams defined within the module.
    OtherTokens,
    /// Module documentation.
    Documentation,
}

impl Display for InternalModuleAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InternalModuleAttribute::Name => write!(f, "name"),
            InternalModuleAttribute::Submodules => write!(f, "submodules"),
            InternalModuleAttribute::Publicness => write!(f, "publicness"),
            InternalModuleAttribute::Data => write!(f, "data"),
            InternalModuleAttribute::InternalTraits => write!(f, "internal_traits"),
            InternalModuleAttribute::OtherTokens => write!(f, "other_tokens"),
            InternalModuleAttribute::Documentation => write!(f, "documentation"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of an
/// `InternalModule`.
pub enum InternalModuleBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<InternalModuleAttribute>),
    /// The name of the module is invalid.
    InvalidName,
    /// A submodule with the same name has already been added to the
    /// module.
    DuplicatedSubmoduleName,
    /// A data struct with the same name has already been added to the
    /// module.
    DuplicatedDataName,
    /// An internal trait with the same name has already been added to the
    /// module.
    DuplicatedInternalTraitName,
}

impl Display for InternalModuleBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InternalModuleBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            InternalModuleBuilderError::InvalidName => write!(f, "Invalid module name"),
            InternalModuleBuilderError::DuplicatedSubmoduleName => {
                write!(f, "A submodule with the same name has already been added to the module")
            }
            InternalModuleBuilderError::DuplicatedDataName => {
                write!(f, "A data struct with the same name has already been added to the module")
            }
            InternalModuleBuilderError::DuplicatedInternalTraitName => {
                write!(
                    f,
                    "An internal trait with the same name has already been added to the module"
                )
            }
        }
    }
}

impl Error for InternalModuleBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InternalModuleBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl InternalModuleBuilder {
    /// Sets the name of the module.
    ///
    /// # Arguments
    /// * `name` - The name of the module.
    pub fn name<S: ToString>(mut self, name: S) -> Result<Self, InternalModuleBuilderError> {
        let name = name.to_string();
        if name.trim().is_empty()
            || name.contains(' ')
            || !name.chars().all(|c| c.is_alphanumeric() || c == '_')
            || !name.chars().all(|c| c.is_lowercase() || c == '_')
        {
            return Err(InternalModuleBuilderError::InvalidName);
        }
        self.name = Some(name);
        Ok(self)
    }

    /// Sets the publicness of the module.
    ///
    /// # Arguments
    /// * `publicness` - The publicness of the module.
    pub fn publicness(mut self, publicness: Publicness) -> Self {
        self.publicness = Some(publicness);
        self
    }

    /// Sets the module as public.
    pub fn public(mut self) -> Self {
        self.publicness = Some(Publicness::Public);
        self
    }

    /// Sets the module as private.
    pub fn private(mut self) -> Self {
        self.publicness = Some(Publicness::Private);
        self
    }

    /// Sets the documentation of the module.
    ///
    /// # Arguments
    /// * `documentation` - The documentation of the module.
    pub fn documentation(mut self, documentation: impl Into<ModuleDocumentation>) -> Self {
        self.documentation = Some(documentation.into());
        self
    }

    /// Adds a submodule to the module.
    ///
    /// # Arguments
    /// * `submodule` - The submodule to add.
    pub fn submodule(
        mut self,
        submodule: InternalModule,
    ) -> Result<Self, InternalModuleBuilderError> {
        if self.submodules.iter().any(|m| m.name() == submodule.name()) {
            return Err(InternalModuleBuilderError::DuplicatedSubmoduleName);
        }
        self.submodules.push(submodule);
        Ok(self)
    }

    /// Adds a data struct to the module.
    ///
    /// # Arguments
    /// * `data` - The data struct to add.
    pub fn data(mut self, data: InternalData) -> Result<Self, InternalModuleBuilderError> {
        if self.data.iter().any(|d| d.as_ref() == &data) {
            return Err(InternalModuleBuilderError::DuplicatedDataName);
        }
        self.data.push(Arc::new(data));
        Ok(self)
    }

    /// Adds an internal trait to the module.
    ///
    /// # Arguments
    /// * `internal_trait` - The internal trait to add.
    pub fn internal_trait(
        mut self,
        internal_trait: InternalTrait,
    ) -> Result<Self, InternalModuleBuilderError> {
        if self.internal_traits.iter().any(|t| t.as_ref() == &internal_trait) {
            return Err(InternalModuleBuilderError::DuplicatedInternalTraitName);
        }
        self.internal_traits.push(Arc::new(internal_trait));
        Ok(self)
    }

    /// Adds an internal token stream to the module.
    ///
    /// # Arguments
    /// * `internal_token` - The internal token stream to add.
    pub fn internal_token(mut self, internal_token: InternalToken) -> Self {
        self.internal_tokens.push(internal_token);
        self
    }

    /// Adds multiple internal token streams to the module.
    ///
    /// # Arguments
    /// * `internal_tokens` - The internal token streams to add.
    pub fn internal_tokens<I>(mut self, internal_tokens: I) -> Self
    where
        I: IntoIterator<Item = InternalToken>,
    {
        self.internal_tokens.extend(internal_tokens);
        self
    }
}

impl Attributed for InternalModuleBuilder {
    type Attribute = InternalModuleAttribute;
}

impl IsCompleteBuilder for InternalModuleBuilder {
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.publicness.is_some() && self.documentation.is_some()
    }
}

impl Builder for InternalModuleBuilder {
    type Error = BuilderError<InternalModuleAttribute>;
    type Object = InternalModule;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(InternalModule {
            name: self.name.ok_or(BuilderError::IncompleteBuild(InternalModuleAttribute::Name))?,
            submodules: self.submodules,
            publicness: self
                .publicness
                .ok_or(BuilderError::IncompleteBuild(InternalModuleAttribute::Publicness))?,
            data: self.data,
            internal_traits: self.internal_traits,
            internal_tokens: self.internal_tokens,
            documentation: self
                .documentation
                .ok_or(BuilderError::IncompleteBuild(InternalModuleAttribute::Documentation))?,
        })
    }
}
