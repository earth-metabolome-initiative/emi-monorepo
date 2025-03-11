//! Implementation of the [`Translator`] trait for the
//! [`CreateTable`](sqlparser::ast::CreateTable) type.

use sqlparser::ast::CreateTable;

use crate::prelude::{Pg2Sqlite, Translator};

impl Translator for CreateTable {
    type Schema = Pg2Sqlite;
    type SQLiteEntry = CreateTable;

    fn translate(&self, schema: &Self::Schema) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        Ok(self.clone())
    }
}
