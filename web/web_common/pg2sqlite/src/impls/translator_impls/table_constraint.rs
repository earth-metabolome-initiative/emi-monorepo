//! Implementation of the [`Translator`] trait for the
//! [`TableConstraint`](sqlparser::ast::TableConstraint) type.

use sql_traits::structs::ParserDB;
use sqlparser::ast::{Function, TableConstraint};

use crate::{
    options::Pg2SqliteOptions,
    prelude::{TranslationOptions, Translator},
};

impl Translator for TableConstraint {
    type Schema = ParserDB;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = Option<TableConstraint>;

    fn translate(
        &self,
        _schema: &Self::Schema,
        options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        match self {
            Self::Check(check_constraint) => {
                match check_constraint.expr.as_ref() {
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
