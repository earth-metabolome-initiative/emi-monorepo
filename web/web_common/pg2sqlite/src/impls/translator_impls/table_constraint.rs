//! Implementation of the [`Translator`] trait for the
//! [`TableConstraint`](sqlparser::ast::TableConstraint) type.

use sqlparser::ast::{Function, TableConstraint};

use crate::prelude::{Pg2Sqlite, Schema, Translator};

impl Translator for TableConstraint {
    type Schema = Pg2Sqlite;
    type SQLiteEntry = Option<TableConstraint>;

    fn translate(&self, schema: &Self::Schema) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        match self {
            Self::Check { expr, .. } => {
                match expr.as_ref() {
                    sqlparser::ast::Expr::Function(Function { name, .. }) => {
                        let function_name = name.to_string();
                        if schema.has_function(&function_name) {
                            Ok(Some(self.clone()))
                        } else if schema.should_remove_unsupported_check_constraints() {
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
