//! Submodule providing the builder for `ConstraintErrorInfo`.

use std::fmt::Display;

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::error::ConstraintErrorInfo;

#[derive(Default)]
/// Builder for `ConstraintErrorInfo`.
pub struct ConstraintErrorInfoBuilder {
    constraint: Option<&'static str>,
    object: Option<String>,
    message: Option<String>,
    resolution: Option<String>,
}

impl ConstraintErrorInfoBuilder {
    /// Set the `constraint` attribute.
    pub fn constraint(
        mut self,
        constraint: &'static str,
    ) -> Result<Self, ConstraintErrorInfoBuilderError> {
        if constraint.trim().is_empty() {
            return Err(ConstraintErrorInfoBuilderError::EmptyConstraint);
        }
        self.constraint = Some(constraint);
        Ok(self)
    }

    /// Set the `object` attribute.
    pub fn object(mut self, object: String) -> Result<Self, ConstraintErrorInfoBuilderError> {
        if object.trim().is_empty() {
            return Err(ConstraintErrorInfoBuilderError::EmptyObject);
        }
        self.object = Some(object);
        Ok(self)
    }

    /// Set the `message` attribute.
    pub fn message(mut self, message: String) -> Result<Self, ConstraintErrorInfoBuilderError> {
        if message.trim().is_empty() {
            return Err(ConstraintErrorInfoBuilderError::EmptyMessage);
        }
        self.message = Some(message);
        Ok(self)
    }

    /// Set the `resolution` attribute.
    pub fn resolution(
        mut self,
        resolution: String,
    ) -> Result<Self, ConstraintErrorInfoBuilderError> {
        if resolution.trim().is_empty() {
            return Err(ConstraintErrorInfoBuilderError::EmptyResolution);
        }
        self.resolution = Some(resolution);
        Ok(self)
    }
}

#[derive(Debug)]
/// Attributes for `ConstraintErrorInfoBuilder`.
pub enum ConstraintErrorInfoAttribute {
    Constraint,
    Object,
    Message,
    Resolution,
}

impl Display for ConstraintErrorInfoAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstraintErrorInfoAttribute::Constraint => write!(f, "constraint"),
            ConstraintErrorInfoAttribute::Object => write!(f, "object"),
            ConstraintErrorInfoAttribute::Message => write!(f, "message"),
            ConstraintErrorInfoAttribute::Resolution => write!(f, "resolution"),
        }
    }
}

impl Attributed for ConstraintErrorInfoBuilder {
    type Attribute = ConstraintErrorInfoAttribute;
}

impl IsCompleteBuilder for ConstraintErrorInfoBuilder {
    fn is_complete(&self) -> bool {
        self.constraint.is_some() && self.object.is_some() && self.message.is_some()
    }
}

#[derive(Debug)]
/// Errors that can occur when building a `ConstraintErrorInfo`.
pub enum ConstraintErrorInfoBuilderError {
    Builder(BuilderError<ConstraintErrorInfoAttribute>),
    EmptyConstraint,
    EmptyMessage,
    EmptyObject,
    EmptyResolution,
}

impl From<BuilderError<ConstraintErrorInfoAttribute>> for ConstraintErrorInfoBuilderError {
    fn from(err: BuilderError<ConstraintErrorInfoAttribute>) -> Self {
        ConstraintErrorInfoBuilderError::Builder(err)
    }
}
impl Display for ConstraintErrorInfoBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstraintErrorInfoBuilderError::Builder(err) => write!(f, "Builder error: {err}"),
            ConstraintErrorInfoBuilderError::EmptyConstraint => {
                write!(f, "Constraint cannot be empty")
            }
            ConstraintErrorInfoBuilderError::EmptyMessage => write!(f, "Message cannot be empty"),
            ConstraintErrorInfoBuilderError::EmptyObject => write!(f, "Object cannot be empty"),
            ConstraintErrorInfoBuilderError::EmptyResolution => {
                write!(f, "Resolution cannot be empty")
            }
        }
    }
}

impl std::error::Error for ConstraintErrorInfoBuilderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConstraintErrorInfoBuilderError::Builder(err) => Some(err),
            ConstraintErrorInfoBuilderError::EmptyConstraint => None,
            ConstraintErrorInfoBuilderError::EmptyMessage => None,
            ConstraintErrorInfoBuilderError::EmptyObject => None,
            ConstraintErrorInfoBuilderError::EmptyResolution => None,
        }
    }
}

impl Builder for ConstraintErrorInfoBuilder {
    type Error = ConstraintErrorInfoBuilderError;
    type Object = ConstraintErrorInfo;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(ConstraintErrorInfo {
            constraint: self
                .constraint
                .ok_or(BuilderError::IncompleteBuild(ConstraintErrorInfoAttribute::Constraint))?,
            object: self
                .object
                .ok_or(BuilderError::IncompleteBuild(ConstraintErrorInfoAttribute::Object))?,
            message: self
                .message
                .ok_or(BuilderError::IncompleteBuild(ConstraintErrorInfoAttribute::Message))?,
            resolution: self.resolution,
        })
    }
}
