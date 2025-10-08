//! Implement the [`CheckConstraint`] trait for the `sqlparser` crate's

use sqlparser::ast::{CheckConstraint, Expr};

use crate::traits::CheckConstraintLike;

impl CheckConstraintLike for CheckConstraint {
    fn expression(&self) -> Expr {
        self.expr.as_ref().clone()
    }
}
