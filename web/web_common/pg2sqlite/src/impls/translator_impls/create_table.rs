//! Implementation of the [`Translator`] trait for the
//! [`CreateTable`](sqlparser::ast::CreateTable) type.

use sqlparser::ast::{CreateTable, TableConstraint};

use crate::prelude::{Pg2Sqlite, Translator};

impl Translator for CreateTable {
    type Schema = Pg2Sqlite;
    type SQLiteEntry = CreateTable;

    fn translate(&self, schema: &Self::Schema) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        Ok(Self {
            constraints: self
                .constraints
                .iter()
                .map(|c| c.translate(schema))
                .collect::<Result<Vec<Option<TableConstraint>>, _>>()?
                .into_iter()
                .filter_map(|c| c)
                .collect(),
            ..self.clone()
        })
    }
}
