//! Submodule defining a builder for the `WhereClause` struct.

use std::fmt::Display;

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{InternalToken, WhereClause};

#[derive(Default)]
/// Builder for the `WhereClause` struct.
pub struct WhereClauseBuilder {
    /// Left-hand side of the where clause.
    left: Option<InternalToken>,
    /// Right-hand side of the where clause.
    right: Option<InternalToken>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `WhereClause` struct.
pub enum WhereClauseAttribute {
    /// Left-hand side of the where clause.
    Left,
    /// Right-hand side of the where clause.
    Right,
}

impl Display for WhereClauseAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WhereClauseAttribute::Left => write!(f, "left"),
            WhereClauseAttribute::Right => write!(f, "right"),
        }
    }
}

impl WhereClauseBuilder {
    /// Sets the left-hand side of the where clause.
    ///
    /// # Arguments
    /// * `left` - The left-hand side of the where clause.
    pub fn left<T>(mut self, left: T) -> Self
    where
        T: Into<InternalToken>,
    {
        self.left = Some(left.into());
        self
    }

    /// Sets the right-hand side of the where clause.
    ///
    /// # Arguments
    /// * `right` - The right-hand side of the where clause.
    pub fn right<T>(mut self, right: T) -> Self
    where
        T: Into<InternalToken>,
    {
        self.right = Some(right.into());
        self
    }
}

impl Attributed for WhereClauseBuilder {
    type Attribute = WhereClauseAttribute;
}

impl IsCompleteBuilder for WhereClauseBuilder {
    fn is_complete(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }
}

impl Builder for WhereClauseBuilder {
    type Error = BuilderError<WhereClauseAttribute>;
    type Object = WhereClause;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(WhereClause {
            left: self.left.ok_or(BuilderError::IncompleteBuild(WhereClauseAttribute::Left))?,
            right: self.right.ok_or(BuilderError::IncompleteBuild(WhereClauseAttribute::Right))?,
        })
    }
}
