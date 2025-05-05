//! Implementation of the [`Translator`] trait for the
//! [`CreateIndex`](sqlparser::ast::CreateIndex) type.

use sqlparser::ast::{CreateIndex, IndexType, Statement};

use crate::prelude::{Pg2Sqlite, Translator};

impl Translator for CreateIndex {
    type Schema = Pg2Sqlite;
    type SQLiteEntry = Vec<Statement>;

    fn translate(&self, _schema: &Self::Schema) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        // If the index is a GIN or GiST index, we need to translate it into a table
        // with a FTS5 virtual table. This is because SQLite does not support
        // GIN or GiST indexes.
        if let Some(IndexType::GIN | IndexType::GiST) = self.using {
            todo!("Translate GIN/GiST index into FTS5 table");
            // let _fts5_table = create_fts5_from_index(self);
        }

        unimplemented!("CreateIndex translation for definition `{}` is not yet implemented.", self)
    }
}
