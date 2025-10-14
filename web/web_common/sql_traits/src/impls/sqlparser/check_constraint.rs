//! Implement the [`CheckConstraint`] trait for the `sqlparser` crate's

use sqlparser::ast::{CheckConstraint, CreateTable, Expr};

use crate::{structs::TableAttribute, traits::CheckConstraintLike};

impl<'db> CheckConstraintLike for TableAttribute<CreateTable, CheckConstraint> {
    fn expression(&self) -> Expr {
        self.attribute().expr.as_ref().clone()
    }
}
