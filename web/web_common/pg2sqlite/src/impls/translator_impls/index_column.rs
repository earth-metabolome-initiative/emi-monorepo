//! Implementation of the [`Translator`] trait for the
//! [`IndexColumn`](sqlparser::ast::IndexColumn) type.

use sqlparser::ast::IndexColumn;

use crate::prelude::{Pg2SqliteOptions, PgSchema, Translator};

impl Translator for IndexColumn {
    type Schema = PgSchema;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = Self;

    fn translate(
        &self,
        schema: &mut Self::Schema,
        options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        Ok(IndexColumn { column: self.column.translate(schema, options)?, ..self.clone() })
    }
}
