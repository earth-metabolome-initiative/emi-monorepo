//! Implementation of the [`Translator`] trait for the
//! [`OrderByExpr`](sqlparser::ast::OrderByExpr) type.

use sql_traits::structs::ParserDB;
use sqlparser::ast::OrderByExpr;

use crate::prelude::{Pg2SqliteOptions, Translator};

impl Translator for OrderByExpr {
    type Schema = ParserDB;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = Self;

    fn translate(
        &self,
        schema: &Self::Schema,
        options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        if self.with_fill.is_some() {
            return Err(crate::errors::Error::UnknownPostgresFeature(
                "WITH FILL in ORDER BY".to_string(),
            ));
        }

        Ok(OrderByExpr {
            expr: self.expr.translate(schema, options)?,
            options: self.options,
            with_fill: None,
        })
    }
}
