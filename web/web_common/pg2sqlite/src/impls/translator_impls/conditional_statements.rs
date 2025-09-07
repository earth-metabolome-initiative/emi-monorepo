//! Implementation of the [`Translator`] trait for the
//! [`ConditionalStatements`](sqlparser::ast::ConditionalStatements) type.

use sqlparser::ast::ConditionalStatements;

use crate::prelude::{Pg2SqliteOptions, PgSchema, Translator};

impl Translator for ConditionalStatements {
    type Schema = PgSchema;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = Self;

    fn translate(
        &self,
        schema: &mut Self::Schema,
        options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        Ok(match self {
            _ => {
                unimplemented!(
                    "ConditionalStatements translation for definition `{}` is not yet implemented.",
                    self
                )
            }
        })
    }
}
