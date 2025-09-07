//! Implementation of the [`Translator`] trait for the
//! [`OrderByExpr`](sqlparser::ast::OrderByExpr) type.

use sqlparser::ast::OrderByExpr;

use crate::prelude::{Pg2Sqlite, Translator};

impl Translator for OrderByExpr {
    type Schema = Pg2Sqlite;
    type SQLiteEntry = Self;

    fn translate(&self, schema: &Self::Schema) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        if self.with_fill.is_some() {
            return Err(crate::errors::Error::UnknownPostgresFeature(
                "WITH FILL in ORDER BY".to_string(),
            ));
        }

        Ok(OrderByExpr {
            expr: self.expr.translate(schema)?,
            options: self.options.clone(),
            with_fill: None,
        })
    }
}
