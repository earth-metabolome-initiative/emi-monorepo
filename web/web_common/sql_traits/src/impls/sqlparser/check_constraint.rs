//! Implement the [`CheckConstraint`] trait for the `sqlparser` crate's

use sqlparser::ast::CheckConstraint;

use crate::traits::CheckConstraintLike;

impl CheckConstraintLike for CheckConstraint {
    fn clause(&self) -> String {
        self.expr.to_string()
    }
}
