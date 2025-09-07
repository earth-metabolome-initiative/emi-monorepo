//! Implementation of the [`Translator`] trait for the
//! [`Expr`](sqlparser::ast::Expr) type.

use sqlparser::ast::Expr;

use crate::prelude::{Pg2Sqlite, Translator};

impl Translator for Expr {
    type Schema = Pg2Sqlite;
    type SQLiteEntry = Self;

    fn translate(&self, schema: &Self::Schema) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        Ok(match self {
            Expr::Function(func) => Expr::Function(func.translate(schema)?),
            _ => {
                unimplemented!("Expr translation for definition `{}` is not yet implemented.", self)
            }
        })
    }
}
