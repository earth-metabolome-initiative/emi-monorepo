//! Implementation of the [`Translator`] trait for the
//! [`Statement`](sqlparser::ast::Statement) type.

use sql_traits::structs::ParserDB;
use sqlparser::ast::Statement;

use crate::prelude::{Pg2SqliteOptions, Translator};

impl Translator for Statement {
    type Schema = ParserDB;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = Vec<Statement>;

    fn translate(
        &self,
        schema: &Self::Schema,
        options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        Ok(match self {
            Self::CreateTable(create_table) => {
                vec![create_table.translate(schema, options)?.into()]
            }
            Self::CreateIndex(create_index) => {
                create_index.translate(schema, options)?.map(Into::into).into_iter().collect()
            }
            Self::CreateFunction(_) => Vec::new(),
            Self::CreateTrigger(create_trigger) => {
                let (maybe_drop_trigger, create_trigger) =
                    create_trigger.translate(schema, options)?;
                let mut statements = vec![];
                if let Some(drop_trigger) = maybe_drop_trigger {
                    statements.push(drop_trigger.into());
                }
                statements.push(create_trigger.into());
                statements
            }
            unsupported_statement => {
                unimplemented!(
                    "Unsupported PostgreSQL statement: `{}` - Parsed as: {unsupported_statement:?}",
                    unsupported_statement.to_string()
                )
            }
        })
    }
}
