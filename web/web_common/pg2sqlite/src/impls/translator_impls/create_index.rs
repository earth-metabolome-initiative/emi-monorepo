//! Implementation of the [`Translator`] trait for the
//! [`CreateIndex`](sqlparser::ast::CreateIndex) type.

use sql_traits::{
    structs::ParserDB,
    traits::{DatabaseLike, TableLike},
};
use sqlparser::ast::{CreateIndex, IndexType};

use crate::{
    prelude::{Pg2SqliteOptions, Translator},
    traits::translation_options::TranslationOptions,
};

impl Translator for CreateIndex {
    type Schema = ParserDB;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = Option<Self>;

    fn translate(
        &self,
        schema: &Self::Schema,
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
            let table_name = match self.table_name.0.last() {
                Some(sqlparser::ast::ObjectNamePart::Identifier(ident)) => ident.value.as_str(),
                _ => {
                    return Err(crate::errors::Error::UnknownPostgresFeature(
                        "Unsupported table name format in index".to_string(),
                    ));
                }
            };
            let table = schema.table(None, table_name);
            if !table.has_generated_primary_key(schema)
                && table.primary_key_type(schema).as_slice() == ["uuid"]
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
