//! Submodule defining a trait for what a `Builder` should be able to do.

use std::fmt::{Debug, Display};

/// Trait defining what a `CompleteBuilder` should be able to do.
pub trait IsCompleteBuilder {
    /// Returns whether the builder is complete, meaning all required attributes
    /// have been set.
    fn is_complete(&self) -> bool;
}

impl<T> IsCompleteBuilder for Option<T> {
    fn is_complete(&self) -> bool {
        self.is_some()
    }
}

/// Trait defining an associated enumeration of attributes.
pub trait Attributed {
    /// The enumeration of the attributes that can be set.
    type Attribute: core::fmt::Debug + Display + 'static;
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// A struct representing an empty tuple, used for builders without attributes.
pub struct EmptyTuple;

impl Debug for EmptyTuple {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "()")
    }
}

impl Display for EmptyTuple {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "()")
    }
}

impl<T> Attributed for Option<T> {
    type Attribute = EmptyTuple;
}

/// Trait defining what a `Builder` should be able to do.
pub trait Builder: Default + IsCompleteBuilder + Attributed {
    /// The type of the object being built.
    type Object;
    /// The type of errors that can occur during building.
    type Error: core::error::Error + From<BuilderError<Self::Attribute>>;

    /// Builds the object.
    ///
    /// # Errors
    ///
    /// * If an attribute was not set.
    /// * If the object could not be built.
    fn build(self) -> Result<Self::Object, Self::Error>;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Errors that can commonly occur during building.
pub enum BuilderError<A> {
    /// An attribute was not set.
    IncompleteBuild(A),
    /// An automatically generated attribute was unexpectedly set.
    UnexpectedAttribute(A),
}

impl<FieldName> BuilderError<FieldName> {
    /// Converts the `BuilderError` into a new `BuilderError` with a different
    /// field name.
    pub fn into_field_name<F, NewFieldName>(self, convert: F) -> BuilderError<NewFieldName>
    where
        F: Fn(FieldName) -> NewFieldName,
    {
        match self {
            BuilderError::IncompleteBuild(missing_attribute) => {
                BuilderError::IncompleteBuild(convert(missing_attribute))
            }
            BuilderError::UnexpectedAttribute(unexpected_attribute) => {
                BuilderError::UnexpectedAttribute(convert(unexpected_attribute))
            }
        }
    }
}

impl<A: core::fmt::Display> core::fmt::Display for BuilderError<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::IncompleteBuild(missing_attribute) => {
                write!(f, "Incomplete build: missing attribute: `{missing_attribute}`")
            }
            Self::UnexpectedAttribute(unexpected_attribute) => {
                write!(
                    f,
                    "Unexpected attribute: the attribute `{unexpected_attribute}` was set, but it should not have been, as it will be overwritten."
                )
            }
        }
    }
}

impl<A: core::fmt::Debug> core::fmt::Debug for BuilderError<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::IncompleteBuild(missing_attribute) => {
                write!(f, "Incomplete build: missing attribute: `{missing_attribute:?}`")
            }
            Self::UnexpectedAttribute(unexpected_attribute) => {
                write!(
                    f,
                    "Unexpected attribute: the attribute `{unexpected_attribute:?}` was set, but it should not have been, as it will be overwritten."
                )
            }
        }
    }
}

impl<A: core::fmt::Debug + core::fmt::Display> std::error::Error for BuilderError<A> {}
