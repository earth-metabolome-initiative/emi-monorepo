//! Submodule defining a trait for what a `Builder` should be able to do.

use crate::prelude::Basic;

/// Trait defining what a `Builder` should be able to do.
pub trait Builder: Default {
    /// The type of the object being built.
    type Object;
    /// The type of errors that can occur during building.
    type Error: core::error::Error + From<BuilderError<Self::Attribute>>;
    /// The enumeration of the attributes that can be set.
    type Attribute: Basic;

    /// Builds the object.
    /// 
    /// # Errors
    /// 
    /// * If an attribute was not set.
    /// * If the object could not be built.
    fn build(self) -> Result<Self::Object, Self::Error>;
}

#[derive(Debug, Clone)]
/// Errors that can commonly occur during building.
pub enum BuilderError<A> {
    /// An attribute was not set.
    IncompleteBuild {
        /// The attribute that was not set.
        missing_attribute: A,
    },
}

impl<A: core::fmt::Display> core::fmt::Display for BuilderError<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::IncompleteBuild { missing_attribute } => {
                write!(f, "Incomplete build: missing attribute: {missing_attribute}")
            }
        }
    }
}

impl<A: core::fmt::Debug + core::fmt::Display> std::error::Error for BuilderError<A> {}
