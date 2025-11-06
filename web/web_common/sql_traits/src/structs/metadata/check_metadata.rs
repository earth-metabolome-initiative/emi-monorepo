//! Submodule defining a generic `IndexMetadata` struct.

use std::rc::Rc;

use sqlparser::ast::Expr;

use crate::traits::{CheckConstraintLike, DatabaseLike};

#[derive(Debug, Clone)]
/// Struct collecting metadata about a check constraint.
pub struct CheckMetadata<U: CheckConstraintLike> {
    /// The expression defining the constraint.
    expression: Expr,
    /// The table on which the constraint is defined.
    table: Rc<<U::DB as DatabaseLike>::Table>,
    /// The columns involved in the constraint.
    columns: Vec<Rc<<U::DB as DatabaseLike>::Column>>,
}

impl<U: CheckConstraintLike> CheckMetadata<U> {
    /// Creates a new `CheckMetadata` instance.
    pub fn new(
        expression: Expr,
        table: Rc<<U::DB as DatabaseLike>::Table>,
        columns: Vec<Rc<<U::DB as DatabaseLike>::Column>>,
    ) -> Self {
        Self { expression, table, columns }
    }

    /// Returns a reference to the expression defining the constraint.
    pub fn expression(&self) -> &Expr {
        &self.expression
    }

    /// Returns a reference to the table on which the constraint is defined.
    pub fn table(&self) -> &<U::DB as DatabaseLike>::Table {
        &self.table
    }

    /// Returns an iterator over the columns involved in the constraint.
    pub fn columns(&self) -> impl Iterator<Item = &<U::DB as DatabaseLike>::Column> {
        self.columns.iter().map(|col| col.as_ref())
    }
}
