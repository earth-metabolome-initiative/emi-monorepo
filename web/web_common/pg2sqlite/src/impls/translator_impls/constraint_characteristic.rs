//! Implementation of the [`Translator`] trait for the
//! [`ConstraintCharacteristics`](sqlparser::ast::ConstraintCharacteristics)
//! type.

use sqlparser::ast::ConstraintCharacteristics;

use crate::prelude::{Pg2Sqlite, Translator};

impl Translator for ConstraintCharacteristics {
    type Schema = Pg2Sqlite;
    type SQLiteEntry = ConstraintCharacteristics;

    fn translate(&self, _schema: &Self::Schema) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        unimplemented!("The constraint characteristic {:?} is not supported", self)
    }
}
