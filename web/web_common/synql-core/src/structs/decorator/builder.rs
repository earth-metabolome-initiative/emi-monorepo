//! Submodule defining a builder for the `Decorator` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};
use quote::ToTokens;

use crate::structs::{FeatureFlag, InternalToken, decorator::Decorator};

#[derive(Default)]
/// Builder for the `Decorator` struct.
pub struct DecoratorBuilder<'data> {
    /// Features required by the decorator.
    features: Vec<FeatureFlag>,
    /// Internal token which represents the decorator.
    token: Option<InternalToken<'data>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `Decorator` struct.
pub enum DecoratorAttribute {
    /// Features required by the decorator.
    Features,
    /// Internal token which represents the decorator.
    Token,
}

impl Display for DecoratorAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecoratorAttribute::Features => write!(f, "features"),
            DecoratorAttribute::Token => write!(f, "token"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of a
/// `Decorator`.
pub enum DecoratorBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<DecoratorAttribute>),
    /// A feature with the same name has already been added.
    DuplicatedFeature,
    /// The internal token is invalid (empty token stream).
    InvalidToken,
}

impl Display for DecoratorBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecoratorBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            DecoratorBuilderError::DuplicatedFeature => {
                write!(f, "A feature with the same name has already been added")
            }
            DecoratorBuilderError::InvalidToken => {
                write!(f, "The internal token is invalid (empty token stream)")
            }
        }
    }
}

impl Error for DecoratorBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DecoratorBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl<'data> DecoratorBuilder<'data> {
    /// Adds a feature required by the decorator.
    ///
    /// # Arguments
    /// * `feature` - The feature to add.
    pub fn add_feature(mut self, feature: FeatureFlag) -> Result<Self, DecoratorBuilderError> {
        if self.features.iter().any(|f| f == &feature) {
            return Err(DecoratorBuilderError::DuplicatedFeature);
        }
        self.features.push(feature);
        Ok(self)
    }

    /// Adds multiple features required by the decorator.
    ///
    /// # Arguments
    /// * `features` - The features to add.
    pub fn add_features<I>(mut self, features: I) -> Result<Self, DecoratorBuilderError>
    where
        I: IntoIterator<Item = FeatureFlag>,
    {
        for feature in features {
            self = self.add_feature(feature)?;
        }
        Ok(self)
    }

    /// Sets the internal token which represents the decorator.
    ///
    /// # Arguments
    /// * `token` - The internal token to set.
    pub fn token(mut self, token: InternalToken<'data>) -> Result<Self, DecoratorBuilderError> {
        if token.to_token_stream().is_empty() {
            return Err(DecoratorBuilderError::InvalidToken);
        }
        self.token = Some(token);
        Ok(self)
    }
}

impl Attributed for DecoratorBuilder<'_> {
    type Attribute = DecoratorAttribute;
}

impl IsCompleteBuilder for DecoratorBuilder<'_> {
    fn is_complete(&self) -> bool {
        self.token.is_some()
    }
}

impl<'data> Builder for DecoratorBuilder<'data> {
    type Error = BuilderError<DecoratorAttribute>;
    type Object = Decorator<'data>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Decorator {
            features: self.features,
            token: self.token.ok_or(BuilderError::IncompleteBuild(DecoratorAttribute::Token))?,
        })
    }
}
