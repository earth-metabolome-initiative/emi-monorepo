//! Implementation of the [`Translator`] trait for the
//! [`TableConstraint`](sqlparser::ast::TableConstraint) type.

use sqlparser::ast::{Function, TableConstraint};

use crate::{
    options::Pg2SqliteOptions,
    prelude::{PgSchema, TranslationOptions, Translator},
};

impl Translator for TableConstraint {
    type Schema = PgSchema;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = Option<TableConstraint>;

    fn translate(
        &self,
        schema: &mut Self::Schema,
        options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        match self {
            Self::Check { expr, .. } => {
                match expr.as_ref() {
                    sqlparser::ast::Expr::Function(Function { name, .. }) => {
                        let function_name = name.to_string();
                        if options.should_remove_unsupported_check_constraints() {
                            Ok(None)
                        } else {
                            Err(crate::errors::Error::UndefinedFunction(function_name))
                        }
                    }
                    _ => Ok(Some(self.clone())),
                }
            }
            other => Ok(Some(other.clone())),
        }
    }
}
