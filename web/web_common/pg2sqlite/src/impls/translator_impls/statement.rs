//! Implementation of the [`Translator`] trait for the
//! [`Statement`](sqlparser::ast::Statement) type.

use sqlparser::ast::Statement;

use crate::{
    impls::translator_impls::create_trigger::translate_create_trigger,
    prelude::{Pg2SqliteOptions, PgSchema, Schema, Translator},
};

impl Translator for Statement {
    type Schema = PgSchema;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = Vec<Statement>;

    fn translate(
        &self,
        schema: &mut Self::Schema,
        options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        Ok(match self {
            Self::CreateTable(create_table) => {
                vec![Self::CreateTable(create_table.translate(schema, options)?)]
            }
            Self::CreateIndex(create_index) => {
                vec![Statement::CreateIndex(create_index.translate(schema, options)?)]
            }
            Self::CreateFunction(create_function) => {
                schema.add_function(create_function);
                Vec::new()
            }
            Self::CreateTrigger {
                or_alter,
                or_replace,
                is_constraint,
                name,
                period,
                events,
                table_name,
                referenced_table_name,
                referencing,
                trigger_object,
                include_each,
                condition,
                exec_body,
                statements,
                characteristics,
            } => {
                vec![translate_create_trigger(
                    *or_alter,
                    *or_replace,
                    *is_constraint,
                    name,
                    *period,
                    events,
                    table_name,
                    referenced_table_name,
                    referencing,
                    *trigger_object,
                    *include_each,
                    condition,
                    exec_body,
                    statements,
                    characteristics,
                    schema,
                    options,
                )?]
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
