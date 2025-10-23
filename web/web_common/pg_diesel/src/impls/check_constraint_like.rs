//! Implementation of [`CheckConstraintLike`] for [`CheckConstraint`].
//!
//! This module implements the
//! [`CheckConstraintLike`](sql_traits::prelude::CheckConstraintLike)
//! trait for the [`CheckConstraint`] model, enabling generic introspection of
//! check constraints.
//!
//! The implementation parses the check constraint expression from the
//! `check_clause` field using the PostgreSQL SQL parser.

use sql_traits::traits::CheckConstraintLike;
use sqlparser::parser::Parser;

impl CheckConstraintLike for crate::models::CheckConstraint {
    fn expression(&self) -> sqlparser::ast::Expr {
        Parser::new(&sqlparser::dialect::PostgreSqlDialect {})
            .try_with_sql(self.check_clause.as_str())
            .expect("Failed to parse unique constraint expression")
            .parse_expr()
            .expect("No expression found in parsed unique constraint")
    }
}
