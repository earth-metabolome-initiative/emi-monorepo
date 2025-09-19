//! Implementation of the [`Translator`] trait for the
//! [`CreateIndex`](sqlparser::ast::CreateIndex) type.

use sqlparser::ast::{CreateIndex, IndexType};

use crate::{
    prelude::{Pg2SqliteOptions, PgSchema, Translator},
    traits::{Schema, translation_options::TranslationOptions},
};

impl Translator for CreateIndex {
    type Schema = PgSchema;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = Option<Self>;

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

        // If the option to drop indexes without UUID primary key tables is set,
        // we need to check if the index is on a table with a UUID primary key.
        if options.should_drop_indexes_without_uuid_pk_tables() {
            let table_name = self.table_name.to_string();
            if !schema
                .table_has_uuid_pk(&table_name)
                .unwrap_or_else(|| panic!("Table `{table_name}` not found in schema"))
            {
                // Drop the index by returning None.
                return Ok(None);
            }
        }

        Ok(Some(CreateIndex {
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
        }))
    }
}
