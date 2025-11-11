//! Submodule defining a builder for the `InternalData` struct.

use std::{collections::HashMap, error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};
use strum::IntoEnumIterator;

use crate::structs::{
    DataVariantRef, Decorator, Derive, Documentation, ExternalCrate, InternalData,
    InternalDataVariant, InternalToken, Publicness, Trait, external_trait::TraitVariantRef,
};

#[derive(Default)]
/// Builder for the `InternalData` struct.
pub struct InternalDataBuilder {
    /// Publicness of the data.
    publicness: Option<Publicness>,
    /// Name of the data.
    name: Option<String>,
    /// Documentation of the data.
    documentation: Option<Documentation>,
    /// The variant of the data (struct or enum).
    variant: Option<InternalDataVariant>,
    /// The traits implemented for the data.
    traits: Vec<InternalToken>,
    /// The derives applied to the data.
    derives: Vec<Derive>,
    /// The decorators applied to the data.
    decorators: Vec<Decorator>,
    /// Generics used in the data.
    generics: Vec<syn::GenericParam>,
    /// Generic defaults for the data.
    generic_defaults: HashMap<syn::GenericParam, DataVariantRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `InternalData` struct.
pub enum InternalDataAttribute {
    /// Publicness of the data.
    Publicness,
    /// Name of the data.
    Name,
    /// Documentation of the data.
    Documentation,
    /// The variant of the data (struct or enum).
    Variant,
    /// The traits implemented for the data.
    Traits,
    /// The derives applied to the data.
    Derives,
    /// The decorators applied to the data.
    Decorators,
    /// Generics used in the data.
    Generics,
    /// Generic defaults for the data.
    GenericDefaults,
}

impl Display for InternalDataAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InternalDataAttribute::Publicness => write!(f, "publicness"),
            InternalDataAttribute::Name => write!(f, "name"),
            InternalDataAttribute::Documentation => write!(f, "documentation"),
            InternalDataAttribute::Variant => write!(f, "variant"),
            InternalDataAttribute::Traits => write!(f, "traits"),
            InternalDataAttribute::Derives => write!(f, "derives"),
            InternalDataAttribute::Decorators => write!(f, "decorators"),
            InternalDataAttribute::Generics => write!(f, "generics"),
            InternalDataAttribute::GenericDefaults => write!(f, "generic_defaults"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of an
/// `InternalData`.
pub enum InternalDataBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<InternalDataAttribute>),
    /// The name of the data is invalid.
    InvalidName,
    /// A trait with the same name has already been added.
    DuplicatedTrait,
    /// A derive with the same name has already been added.
    DuplicatedDerive,
    /// A decorator with the same token has already been added.
    DuplicatedDecorator,
    /// A generic default was specified for a generic that does not exist.
    GenericDefaultForNonexistentGeneric(syn::GenericParam),
}

impl Display for InternalDataBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InternalDataBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            InternalDataBuilderError::InvalidName => write!(f, "Invalid data name"),
            InternalDataBuilderError::DuplicatedTrait => {
                write!(f, "A trait with the same name has already been added")
            }
            InternalDataBuilderError::DuplicatedDerive => {
                write!(f, "A derive with the same name has already been added")
            }
            InternalDataBuilderError::DuplicatedDecorator => {
                write!(f, "A decorator with the same token has already been added")
            }
            InternalDataBuilderError::GenericDefaultForNonexistentGeneric(ident) => {
                write!(
                    f,
                    "A generic default was specified for a generic that does not exist: {ident:?}",
                )
            }
        }
    }
}

impl Error for InternalDataBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InternalDataBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl InternalDataBuilder {
    /// Sets the name of the data.
    ///
    /// # Arguments
    /// * `name` - The name of the data.
    pub fn name<S: ToString>(mut self, name: S) -> Result<Self, InternalDataBuilderError> {
        let name = name.to_string();
        if name.trim().is_empty()
            || name.contains(' ')
            || !name.chars().all(|c| c.is_alphanumeric() || c == '_')
        {
            return Err(InternalDataBuilderError::InvalidName);
        }
        self.name = Some(name);
        Ok(self)
    }

    /// Sets the documentation of the data.
    ///
    /// # Arguments
    /// * `documentation` - The documentation of the data.
    pub fn documentation(mut self, documentation: Documentation) -> Self {
        self.documentation = Some(documentation);
        self
    }

    /// Sets the publicness of the data.
    ///
    /// # Arguments
    /// * `publicness` - The publicness of the data.
    pub fn publicness(mut self, publicness: Publicness) -> Self {
        self.publicness = Some(publicness);
        self
    }

    /// Sets the data as public.
    pub fn public(mut self) -> Self {
        self.publicness = Some(Publicness::Public);
        self
    }

    /// Sets the data as private.
    pub fn private(mut self) -> Self {
        self.publicness = Some(Publicness::Private);
        self
    }

    /// Sets the variant of the data.
    ///
    /// # Arguments
    /// * `variant` - The variant of the data (struct or enum).
    pub fn variant(mut self, variant: InternalDataVariant) -> Self {
        self.variant = Some(variant);
        self
    }

    /// Adds a trait implementation to the data.
    ///
    /// # Arguments
    /// * `internal_token` - The trait to implement.
    pub fn add_trait(
        mut self,
        internal_token: InternalToken,
    ) -> Result<Self, InternalDataBuilderError> {
        if self.traits.iter().any(|t| t == &internal_token) {
            return Err(InternalDataBuilderError::DuplicatedTrait);
        }
        self.traits.push(internal_token);
        Ok(self)
    }

    /// Adds multiple trait implementations to the data.
    ///
    /// # Arguments
    /// * `traits` - The traits to implement.
    pub fn add_traits<I>(mut self, internal_tokens: I) -> Result<Self, InternalDataBuilderError>
    where
        I: IntoIterator<Item = InternalToken>,
    {
        for internal_token in internal_tokens {
            self = self.add_trait(internal_token)?;
        }
        Ok(self)
    }

    /// Adds a derive to the data.
    ///
    /// # Arguments
    /// * `derive` - The derive to add.
    pub fn derive(mut self, derive: Derive) -> Result<Self, InternalDataBuilderError> {
        if self.derives.iter().any(|d| d == &derive) {
            return Err(InternalDataBuilderError::DuplicatedDerive);
        }
        self.derives.push(derive);
        Ok(self)
    }

    /// Adds multiple derives to the data.
    ///
    /// # Arguments
    /// * `derives` - The derives to add.
    pub fn derives<I>(mut self, derives: I) -> Result<Self, InternalDataBuilderError>
    where
        I: IntoIterator<Item = Derive>,
    {
        for derive in derives {
            self = self.derive(derive)?;
        }
        Ok(self)
    }

    /// Adds a decorator to the data.
    ///
    /// # Arguments
    /// * `decorator` - The decorator to add.
    pub fn decorator(mut self, decorator: Decorator) -> Result<Self, InternalDataBuilderError> {
        if self.decorators.iter().any(|d| d == &decorator) {
            return Err(InternalDataBuilderError::DuplicatedDecorator);
        }
        self.decorators.push(decorator);
        Ok(self)
    }

    /// Adds multiple decorators to the data.
    ///
    /// # Arguments
    /// * `decorators` - The decorators to add.
    pub fn decorators<I>(mut self, decorators: I) -> Result<Self, InternalDataBuilderError>
    where
        I: IntoIterator<Item = Decorator>,
    {
        for decorator in decorators {
            self = self.decorator(decorator)?;
        }
        Ok(self)
    }

    /// Adds a generic to the data.
    ///
    /// # Arguments
    /// * `generic` - The generic to add.
    pub fn generic(mut self, generic: syn::GenericParam) -> Self {
        if !self.generics.contains(&generic) {
            self.generics.push(generic);
        }
        self
    }

    /// Adds multiple generics to the data.
    ///
    /// # Arguments
    /// * `generics` - The generics to add.
    pub fn generics<I>(mut self, generics: I) -> Self
    where
        I: IntoIterator<Item = syn::GenericParam>,
    {
        for generic in generics {
            self = self.generic(generic);
        }
        self
    }

    /// Adds a type default for a generic.
    ///
    /// # Arguments
    ///
    /// * `generic` - The generic to add a default for.
    /// * `default` - The default type for the generic.
    ///
    /// # Errors
    /// * If the generic does not exist.
    pub fn generic_default(
        mut self,
        generic: syn::GenericParam,
        default: DataVariantRef,
    ) -> Result<Self, InternalDataBuilderError> {
        if !self.generics.contains(&generic) {
            return Err(InternalDataBuilderError::GenericDefaultForNonexistentGeneric(generic));
        }
        self.generic_defaults.insert(generic, default);
        Ok(self)
    }
}

impl Attributed for InternalDataBuilder {
    type Attribute = InternalDataAttribute;
}

impl IsCompleteBuilder for InternalDataBuilder {
    fn is_complete(&self) -> bool {
        self.publicness.is_some()
            && self.name.is_some()
            && self.variant.is_some()
            && self.documentation.is_some()
    }
}

impl Builder for InternalDataBuilder {
    type Error = BuilderError<InternalDataAttribute>;
    type Object = InternalData;

    fn build(mut self) -> Result<Self::Object, Self::Error> {
        // We add the auto-derives depending on which traits are supported by all of the
        // data variants.
        let variant =
            self.variant.ok_or(BuilderError::IncompleteBuild(InternalDataAttribute::Variant))?;
        let mut derive_builder = Derive::new();
        for trait_variant in Trait::iter() {
            // If the current trait variant is supported by the data variant, and it has not
            // been already been added, we add it as an auto-derive.
            if variant.supports_trait(&trait_variant.into())
                && !self.traits.iter().any(|t| t.implements_trait(&trait_variant.into()))
            {
                derive_builder = derive_builder.add_trait(trait_variant);
            }
        }
        if let Ok(derive) = derive_builder.build() {
            self.derives.push(derive);
        }

        let mut serde_derive = Derive::new();
        for serde_trait in ExternalCrate::serde().external_trait_refs() {
            let serde_trait: TraitVariantRef = serde_trait.into();
            if variant.supports_trait(&serde_trait)
                && !self.traits.iter().any(|t| t.implements_trait(&serde_trait))
            {
                serde_derive = serde_derive.add_trait(serde_trait);
            }
        }
        if let Ok(derive) = serde_derive.build() {
            self.derives.push(derive);
        }

        let generic_defaults = self
            .generics
            .iter()
            .map(|g| {
                let default = self.generic_defaults.remove(g);
                default
            })
            .collect::<Vec<Option<DataVariantRef>>>();

        Ok(InternalData {
            publicness: self
                .publicness
                .ok_or(BuilderError::IncompleteBuild(InternalDataAttribute::Publicness))?,
            name: self.name.ok_or(BuilderError::IncompleteBuild(InternalDataAttribute::Name))?,
            documentation: self
                .documentation
                .ok_or(BuilderError::IncompleteBuild(InternalDataAttribute::Documentation))?,
            variant: variant,
            traits: self.traits,
            derives: self.derives,
            decorators: self.decorators,
            generics: self.generics,
            generic_defaults,
        })
    }
}
