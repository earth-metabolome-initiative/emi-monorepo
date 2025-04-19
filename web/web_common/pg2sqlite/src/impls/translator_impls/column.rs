//! Implementation of the [`Translator`] trait for the
//! [`Column`](sqlparser::ast::Column) type.

use sqlparser::ast::ColumnDef;

use crate::prelude::{Pg2Sqlite, Translator};

impl Translator for ColumnDef {
    type Schema = Pg2Sqlite;
    type SQLiteEntry = ColumnDef;

    fn translate(&self, schema: &Self::Schema) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        Ok(ColumnDef {
            name: self.name.clone(),
            data_type: self.data_type.translate(schema)?,
            options: self
                .options
                .iter()
                .map(|o| o.translate(schema))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect(),
        })
    }
}
