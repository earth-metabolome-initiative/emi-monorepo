//! Submodule defining a trait for what a `Builder` should be able to do.

/// Trait defining what a `Builder` should be able to do.
pub trait Builder: Default {
    /// The type of the object being built.
    type Object;
    /// The type of errors that can occur during building.
    type Error: core::error::Error + From<BuilderError<Self::Attribute>>;
    /// The enumeration of the attributes that can be set.
    type Attribute;

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
