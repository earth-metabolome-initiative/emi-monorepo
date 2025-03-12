//! Implementation of the [`Translator`] trait for the
//! [`Statement`](sqlparser::ast::Statement) type.

use sqlparser::ast::Statement;

use crate::prelude::{Pg2Sqlite, Translator};

impl Translator for Statement {
    type Schema = Pg2Sqlite;
    type SQLiteEntry = Vec<Statement>;

    fn translate(&self, schema: &Self::Schema) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        Ok(match self {
            Self::CreateTable(create_table) => vec![Self::CreateTable(create_table.translate(schema)?)],
            Self::CreateIndex(create_index) => create_index.translate(schema)?,
            unsupported_statement => {
                unimplemented!("Unsupported PostgreSQL statement: `{}` - Parsed as: {unsupported_statement:?}", unsupported_statement.to_string())
            }
        })
    }
}
