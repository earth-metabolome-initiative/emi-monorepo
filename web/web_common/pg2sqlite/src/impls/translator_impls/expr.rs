//! Implementation of the [`Translator`] trait for the
//! [`Expr`](sqlparser::ast::Expr) type.

use sql_traits::structs::ParserDB;
use sqlparser::ast::Expr;

use crate::prelude::{Pg2SqliteOptions, Translator};

impl Translator for Expr {
    type Schema = ParserDB;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = Self;

    fn translate(
        &self,
        schema: &Self::Schema,
        options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        Ok(match self {
            Expr::Function(func) => Expr::Function(func.translate(schema, options)?),
            Expr::Identifier(_) => self.clone(),
            _ => {
                unimplemented!(
                    "Expr translation for definition `{:?}` is not yet implemented.",
                    self
                )
            }
        })
    }
}
