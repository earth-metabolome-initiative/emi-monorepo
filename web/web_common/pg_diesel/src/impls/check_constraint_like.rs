//! Submodule implementing the
//! [`CheckConstraintLike`](sql_traits::prelude::CheckConstraintLike)
//! trait for the [`CheckConstraint`] struct.

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
