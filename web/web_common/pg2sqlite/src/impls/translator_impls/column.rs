//! Implementation of the [`Translator`] trait for the
//! [`Column`](sqlparser::ast::Column) type.

use sql_traits::structs::ParserDB;
use sqlparser::ast::ColumnDef;

use crate::prelude::{Pg2SqliteOptions, Translator};

impl Translator for ColumnDef {
    type Schema = ParserDB;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = ColumnDef;

    fn translate(
        &self,
        schema: &Self::Schema,
        options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        Ok(ColumnDef {
            name: self.name.clone(),
            data_type: self.data_type.translate(schema, options)?,
            options: self
                .options
                .iter()
                .map(|o| o.translate(schema, options))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect(),
        })
    }
}
