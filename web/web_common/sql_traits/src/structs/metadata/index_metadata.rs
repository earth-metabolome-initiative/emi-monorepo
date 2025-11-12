//! Submodule defining a generic `IndexMetadata` struct.

use std::rc::Rc;

use sqlparser::ast::Expr;

use crate::traits::{DatabaseLike, UniqueIndexLike};

#[derive(Debug, Clone)]
/// Struct collecting metadata about a unique index.
pub struct UniqueIndexMetadata<U: UniqueIndexLike> {
    /// The expression defining the index.
    expression: Expr,
    /// The table on which the index is defined.
    table: Rc<<U::DB as DatabaseLike>::Table>,
}

impl<U: UniqueIndexLike> UniqueIndexMetadata<U> {
    /// Creates a new `UniqueIndexMetadata` instance.
    #[inline]
    pub fn new(expression: Expr, table: Rc<<U::DB as DatabaseLike>::Table>) -> Self {
        Self { expression, table }
    }

    /// Returns a reference to the expression defining the index.
    #[must_use]
    #[inline]
    pub fn expression(&self) -> &Expr {
        &self.expression
    }

    /// Returns a reference to the table on which the index is defined.
    #[must_use]
    #[inline]
    pub fn table(&self) -> &<U::DB as DatabaseLike>::Table {
        &self.table
    }
}
