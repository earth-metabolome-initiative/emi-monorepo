//! Submodule defining a builder for the `InternalData` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{
    Derive, InternalData, InternalDataVariant, Publicness, external_trait::TraitVariantRef,
};

#[derive(Default)]
/// Builder for the `InternalData` struct.
pub struct InternalDataBuilder<'data> {
    /// Publicness of the data.
    publicness: Option<Publicness>,
    /// Name of the data.
    name: Option<String>,
    /// Documentation of the data.
    documentation: Option<String>,
    /// The variant of the data (struct or enum).
    variant: Option<InternalDataVariant<'data>>,
    /// The traits implemented for the data.
    traits: Vec<TraitVariantRef<'data>>,
    /// The derives applied to the data.
    derives: Vec<Derive<'data>>,
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
}

impl Display for InternalDataAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalDataAttribute::Publicness => write!(f, "publicness"),
            InternalDataAttribute::Name => write!(f, "name"),
            InternalDataAttribute::Documentation => write!(f, "documentation"),
            InternalDataAttribute::Variant => write!(f, "variant"),
            InternalDataAttribute::Traits => write!(f, "traits"),
            InternalDataAttribute::Derives => write!(f, "derives"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of an
/// `InternalData`.
pub enum InternalDataBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<InternalDataAttribute>),
    /// The name of the data is invalid.
    InvalidName,
    /// The documentation of the data is invalid.
    InvalidDocumentation,
    /// A trait with the same name has already been added.
    DuplicatedTrait,
    /// A derive with the same name has already been added.
    DuplicatedDerive,
}

impl Display for InternalDataBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalDataBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            InternalDataBuilderError::InvalidName => write!(f, "Invalid data name"),
            InternalDataBuilderError::InvalidDocumentation => {
                write!(f, "Invalid data documentation")
            }
            InternalDataBuilderError::DuplicatedTrait => {
                write!(f, "A trait with the same name has already been added")
            }
            InternalDataBuilderError::DuplicatedDerive => {
                write!(f, "A derive with the same name has already been added")
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

impl<'data> InternalDataBuilder<'data> {
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
    pub fn documentation<S: ToString>(
        mut self,
        documentation: S,
    ) -> Result<Self, InternalDataBuilderError> {
        let documentation = documentation.to_string();
        if documentation.trim().is_empty() {
            return Err(InternalDataBuilderError::InvalidDocumentation);
        }
        self.documentation = Some(documentation);
        Ok(self)
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
    pub fn variant<V>(mut self, variant: V) -> Self
    where
        V: Into<InternalDataVariant<'data>>,
    {
        self.variant = Some(variant.into());
        self
    }

    /// Adds a trait implementation to the data.
    ///
    /// # Arguments
    /// * `trait_ref` - The trait to implement.
    pub fn add_trait(
        mut self,
        trait_ref: TraitVariantRef<'data>,
    ) -> Result<Self, InternalDataBuilderError> {
        if self.traits.iter().any(|t| t == &trait_ref) {
            return Err(InternalDataBuilderError::DuplicatedTrait);
        }
        self.traits.push(trait_ref);
        Ok(self)
    }

    /// Adds multiple trait implementations to the data.
    ///
    /// # Arguments
    /// * `traits` - The traits to implement.
    pub fn add_traits<I>(mut self, traits: I) -> Result<Self, InternalDataBuilderError>
    where
        I: IntoIterator<Item = TraitVariantRef<'data>>,
    {
        for trait_ref in traits {
            self = self.add_trait(trait_ref)?;
        }
        Ok(self)
    }

    /// Adds a derive to the data.
    ///
    /// # Arguments
    /// * `derive` - The derive to add.
    pub fn add_derive(mut self, derive: Derive<'data>) -> Result<Self, InternalDataBuilderError> {
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
    pub fn add_derives<I>(mut self, derives: I) -> Result<Self, InternalDataBuilderError>
    where
        I: IntoIterator<Item = Derive<'data>>,
    {
        for derive in derives {
            self = self.add_derive(derive)?;
        }
        Ok(self)
    }
}

impl Attributed for InternalDataBuilder<'_> {
    type Attribute = InternalDataAttribute;
}

impl IsCompleteBuilder for InternalDataBuilder<'_> {
    fn is_complete(&self) -> bool {
        self.publicness.is_some() && self.name.is_some() && self.variant.is_some()
    }
}

impl<'data> Builder for InternalDataBuilder<'data> {
    type Error = BuilderError<InternalDataAttribute>;
    type Object = InternalData<'data>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        // We add the auto-derives depending on which traits are supported by all of the
        // data variants.

        Ok(InternalData {
            publicness: self
                .publicness
                .ok_or(BuilderError::IncompleteBuild(InternalDataAttribute::Publicness))?,
            name: self.name.ok_or(BuilderError::IncompleteBuild(InternalDataAttribute::Name))?,
            documentation: self.documentation,
            variant: self
                .variant
                .ok_or(BuilderError::IncompleteBuild(InternalDataAttribute::Variant))?,
            traits: self.traits,
            derives: self.derives,
        })
    }
}
