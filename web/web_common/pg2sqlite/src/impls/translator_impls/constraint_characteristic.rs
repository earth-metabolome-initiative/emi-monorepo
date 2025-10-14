//! Implementation of the [`Translator`] trait for the
//! [`ConstraintCharacteristics`](sqlparser::ast::ConstraintCharacteristics)
//! type.

use sql_traits::structs::ParserDB;
use sqlparser::ast::ConstraintCharacteristics;

use crate::prelude::{Pg2SqliteOptions, Translator};

impl Translator for ConstraintCharacteristics {
    type Schema = ParserDB;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = ConstraintCharacteristics;

    fn translate(
        &self,
        _schema: &Self::Schema,
        _options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        unimplemented!("The constraint characteristic {:?} is not supported", self)
    }
}
