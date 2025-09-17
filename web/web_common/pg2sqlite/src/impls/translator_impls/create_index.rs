//! Implementation of the [`Translator`] trait for the
//! [`CreateIndex`](sqlparser::ast::CreateIndex) type.

use sqlparser::ast::{CreateIndex, IndexType};

use crate::prelude::{Pg2SqliteOptions, PgSchema, Translator};

impl Translator for CreateIndex {
    type Schema = PgSchema;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = Self;

    fn translate(
        &self,
        schema: &mut Self::Schema,
        options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        // If the index is a GIN or GiST index, we need to translate it into a table
        // with a FTS5 virtual table. This is because SQLite does not support
        // GIN or GiST indexes.
        if let Some(IndexType::GIN | IndexType::GiST) = self.using {
            todo!("Translate GIN/GiST index into FTS5 table");
            // let _fts5_table = create_fts5_from_index(self);
        }

        Ok(CreateIndex {
            columns: self
                .columns
                .iter()
                .map(|col| col.translate(schema, options))
                .collect::<Result<_, _>>()?,
            predicate: self
                .predicate
                .as_ref()
                .map(|predicate| predicate.translate(schema, options))
                .transpose()?,
            ..self.clone()
        })
    }
}
