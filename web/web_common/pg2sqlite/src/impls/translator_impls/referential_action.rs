//! Implementation of the [`Translator`] trait for the
//! [`ReferentialAction`](sqlparser::ast::ReferentialAction) type.

use sqlparser::ast::ReferentialAction;

use crate::prelude::{Pg2SqliteOptions, PgSchema, Translator};

impl Translator for ReferentialAction {
    type Schema = PgSchema;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = ReferentialAction;

    fn translate(
        &self,
        _schema: &mut Self::Schema,
        _options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        match self {
            ReferentialAction::NoAction => Ok(ReferentialAction::NoAction),
            ReferentialAction::Restrict => Ok(ReferentialAction::Restrict),
            ReferentialAction::SetNull => Ok(ReferentialAction::SetNull),
            ReferentialAction::SetDefault => Ok(ReferentialAction::SetDefault),
            ReferentialAction::Cascade => Ok(ReferentialAction::Cascade),
        }
    }
}
