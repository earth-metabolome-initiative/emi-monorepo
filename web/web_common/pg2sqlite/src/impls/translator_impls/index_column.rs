//! Implementation of the [`Translator`] trait for the
//! [`IndexColumn`](sqlparser::ast::IndexColumn) type.

use sqlparser::ast::IndexColumn;

use crate::prelude::{Pg2Sqlite, Translator};

impl Translator for IndexColumn {
    type Schema = Pg2Sqlite;
    type SQLiteEntry = Self;

    fn translate(&self, schema: &Self::Schema) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        Ok(IndexColumn { column: self.column.translate(schema)?, ..self.clone() })
    }
}
