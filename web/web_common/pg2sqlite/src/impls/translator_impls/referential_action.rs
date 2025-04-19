//! Implementation of the [`Translator`] trait for the
//! [`ReferentialAction`](sqlparser::ast::ReferentialAction) type.

use sqlparser::ast::ReferentialAction;

use crate::prelude::{Pg2Sqlite, Translator};

impl Translator for ReferentialAction {
    type Schema = Pg2Sqlite;
    type SQLiteEntry = ReferentialAction;

    fn translate(&self, _schema: &Self::Schema) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        match self {
            ReferentialAction::NoAction => Ok(ReferentialAction::NoAction),
            ReferentialAction::Restrict => Ok(ReferentialAction::Restrict),
            ReferentialAction::SetNull => Ok(ReferentialAction::SetNull),
            ReferentialAction::SetDefault => Ok(ReferentialAction::SetDefault),
            ReferentialAction::Cascade => Ok(ReferentialAction::Cascade),
        }
    }
}
