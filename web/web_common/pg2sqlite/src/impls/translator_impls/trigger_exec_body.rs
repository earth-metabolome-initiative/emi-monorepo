//! Implementation of the [`Translator`] trait for the
//! [`TriggerExecBody`](sqlparser::ast::TriggerExecBody) type.

use sqlparser::ast::TriggerExecBody;

use crate::prelude::{Pg2SqliteOptions, PgSchema, Translator};

impl Translator for TriggerExecBody {
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
                    "TriggerExecBody translation for definition `{}` is not yet implemented.",
                    self
                )
            }
        })
    }
}
