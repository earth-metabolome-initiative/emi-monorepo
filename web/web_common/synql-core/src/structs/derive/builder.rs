//! Submodule defining a builder for the `Derive` struct.

use std::fmt::Display;

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{FeatureFlag, derive::Derive, external_trait::TraitVariantRef};

#[derive(Default)]
/// Builder for the `Derive` struct.
pub struct DeriveBuilder {
    /// Features required by the derive.
    features: Vec<FeatureFlag>,
    /// Traits implemented by the derive.
    traits: Vec<TraitVariantRef>,
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DeriveAttribute::Features => write!(f, "features"),
            DeriveAttribute::Traits => write!(f, "traits"),
        }
    }
}

impl DeriveBuilder {
    /// Adds a feature required by the derive.
    ///
    /// # Arguments
    /// * `feature` - The feature to add.
    pub fn add_feature(mut self, feature: FeatureFlag) -> Self {
        if !self.features.contains(&feature) {
            self.features.push(feature);
        }
        self
    }

    /// Adds multiple features required by the derive.
    ///
    /// # Arguments
    /// * `features` - The features to add.
    pub fn add_features<I>(mut self, features: I) -> Self
    where
        I: IntoIterator<Item = FeatureFlag>,
    {
        for feature in features {
            self = self.add_feature(feature);
        }
        self
    }

    /// Adds a trait implemented by the derive.
    ///
    /// # Arguments
    /// * `trait_ref` - The trait to add.
    pub fn add_trait<T>(mut self, trait_ref: T) -> Self
    where
        T: Into<TraitVariantRef>,
    {
        let trait_ref = trait_ref.into();
        if !self.traits.contains(&trait_ref) {
            self.traits.push(trait_ref);
        }
        self
    }

    /// Adds multiple traits implemented by the derive.
    ///
    /// # Arguments
    /// * `traits` - The traits to add.
    pub fn add_traits<I, T>(mut self, traits: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<TraitVariantRef>,
    {
        for trait_ref in traits {
            self = self.add_trait(trait_ref);
        }
        self
    }
}

impl Attributed for DeriveBuilder {
    type Attribute = DeriveAttribute;
}

impl IsCompleteBuilder for DeriveBuilder {
    fn is_complete(&self) -> bool {
        // A derive must have at least one trait to be meaningful
        !self.traits.is_empty()
    }
}

impl Builder for DeriveBuilder {
    type Error = BuilderError<DeriveAttribute>;
    type Object = Derive;

    fn build(self) -> Result<Self::Object, Self::Error> {
        if self.traits.is_empty() {
            return Err(BuilderError::IncompleteBuild(DeriveAttribute::Traits));
        }
        Ok(Derive { features: self.features, traits: self.traits })
    }
}
