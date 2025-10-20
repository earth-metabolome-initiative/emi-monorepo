//! Submodule defining a builder for the `Derive` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{FeatureFlag, derive::Derive, external_trait::TraitVariantRef};

#[derive(Default)]
/// Builder for the `Derive` struct.
pub struct DeriveBuilder<'data> {
    /// Features required by the derive.
    features: Vec<FeatureFlag>,
    /// Traits implemented by the derive.
    traits: Vec<TraitVariantRef<'data>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `Derive` struct.
pub enum DeriveAttribute {
    /// Features required by the derive.
    Features,
    /// Traits implemented by the derive.
    Traits,
}

impl Display for DeriveAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeriveAttribute::Features => write!(f, "features"),
            DeriveAttribute::Traits => write!(f, "traits"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of a
/// `Derive`.
pub enum DeriveBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<DeriveAttribute>),
    /// A feature with the same name has already been added.
    DuplicatedFeature,
    /// A trait with the same name has already been added.
    DuplicatedTrait,
}

impl From<BuilderError<DeriveAttribute>> for DeriveBuilderError {
    fn from(e: BuilderError<DeriveAttribute>) -> Self {
        DeriveBuilderError::Builder(e)
    }
}

impl Display for DeriveBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeriveBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            DeriveBuilderError::DuplicatedFeature => {
                write!(f, "A feature with the same name has already been added")
            }
            DeriveBuilderError::DuplicatedTrait => {
                write!(f, "A trait with the same name has already been added")
            }
        }
    }
}

impl Error for DeriveBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DeriveBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl<'data> DeriveBuilder<'data> {
    /// Adds a feature required by the derive.
    ///
    /// # Arguments
    /// * `feature` - The feature to add.
    pub fn add_feature(mut self, feature: FeatureFlag) -> Result<Self, DeriveBuilderError> {
        if self.features.contains(&feature) {
            return Err(DeriveBuilderError::DuplicatedFeature);
        }
        self.features.push(feature);
        Ok(self)
    }

    /// Adds multiple features required by the derive.
    ///
    /// # Arguments
    /// * `features` - The features to add.
    pub fn add_features<I>(mut self, features: I) -> Result<Self, DeriveBuilderError>
    where
        I: IntoIterator<Item = FeatureFlag>,
    {
        for feature in features {
            self = self.add_feature(feature)?;
        }
        Ok(self)
    }

    /// Adds a trait implemented by the derive.
    ///
    /// # Arguments
    /// * `trait_ref` - The trait to add.
    pub fn add_trait<T>(mut self, trait_ref: T) -> Result<Self, DeriveBuilderError>
    where
        T: Into<TraitVariantRef<'data>>,
    {
        let trait_ref = trait_ref.into();
        if self.traits.contains(&trait_ref) {
            return Err(DeriveBuilderError::DuplicatedTrait);
        }
        self.traits.push(trait_ref);
        Ok(self)
    }

    /// Adds multiple traits implemented by the derive.
    ///
    /// # Arguments
    /// * `traits` - The traits to add.
    pub fn add_traits<I, T>(mut self, traits: I) -> Result<Self, DeriveBuilderError>
    where
        I: IntoIterator<Item = T>,
        T: Into<TraitVariantRef<'data>>,
    {
        for trait_ref in traits {
            self = self.add_trait(trait_ref)?;
        }
        Ok(self)
    }
}

impl Attributed for DeriveBuilder<'_> {
    type Attribute = DeriveAttribute;
}

impl IsCompleteBuilder for DeriveBuilder<'_> {
    fn is_complete(&self) -> bool {
        // A derive must have at least one trait to be meaningful
        !self.traits.is_empty()
    }
}

impl<'data> Builder for DeriveBuilder<'data> {
    type Error = DeriveBuilderError;
    type Object = Derive<'data>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        if self.traits.is_empty() {
            return Err(BuilderError::IncompleteBuild(DeriveAttribute::Traits).into());
        }
        Ok(Derive { features: self.features, traits: self.traits })
    }
}
