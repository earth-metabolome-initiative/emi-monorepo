//! Implementation of the [`Translator`] trait for the
//! [`CreateTable`](sqlparser::ast::CreateTable) type.

use sqlparser::ast::{CreateTable, TableConstraint};

use crate::{
    prelude::{Pg2SqliteOptions, PgSchema, Translator},
    traits::schema::Schema,
};

impl Translator for CreateTable {
    type Schema = PgSchema;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = CreateTable;

    fn translate(
        &self,
        schema: &mut Self::Schema,
        options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        let created_table = Self {
            columns: self
                .columns
                .iter()
                .map(|c| c.translate(schema, options))
                .collect::<Result<Vec<_>, _>>()?,
            constraints: self
                .constraints
                .iter()
                .map(|c| c.translate(schema, options))
                .collect::<Result<Vec<Option<TableConstraint>>, _>>()?
                .into_iter()
                .flatten()
                .collect(),
            ..self.clone()
        };

        schema.add_table(&created_table);

        Ok(created_table)
    }
}
