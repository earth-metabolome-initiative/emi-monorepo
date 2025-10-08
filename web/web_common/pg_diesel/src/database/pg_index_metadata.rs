//! Struct collecting information about a [`PgIndex`](crate::models::PgIndex)
//! entry's metadata.

use diesel::PgConnection;
use sqlparser::{ast::Expr, parser::Parser};

/// Struct collecting metadata about an index represented by a
/// [`PgIndex`](crate::models::PgIndex) entry.
pub struct PgIndexMetadata {
    /// The expression defining the index.
    expression: Expr,
}

impl PgIndexMetadata {
    /// Creates a new `PgIndexMetadata` instance from the given
    /// [`PgIndex`](crate::models::PgIndex) and connection to the database.
    ///
    /// # Arguments
    ///
    /// * `index` - The `PgIndex` instance for which to create the metadata.
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * Returns a [`diesel::result::Error`] if the database query fails.
    pub fn new(
        index: &crate::models::PgIndex,
        conn: &mut PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        let expression = crate::models::pg_index::expression(index, conn)?;
        let parsed_expression = Parser::new(&sqlparser::dialect::PostgreSqlDialect {})
            .try_with_sql(expression.as_str())
            .expect("Failed to parse unique constraint expression")
            .parse_expr()
            .expect("No expression found in parsed unique constraint");
        Ok(PgIndexMetadata { expression: parsed_expression })
    }

    /// Returns a reference to the expression defining the index.
    pub fn expression(&self) -> &Expr {
        &self.expression
    }
}
