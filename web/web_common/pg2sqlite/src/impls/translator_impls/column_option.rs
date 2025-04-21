//! Implementation of the [`Translator`] trait for the
//! [`Column`](sqlparser::ast::Column) type.

use sqlparser::ast::{ColumnOption, ColumnOptionDef, Expr, Function};

use crate::prelude::{Pg2Sqlite, Schema, Translator};

impl Translator for ColumnOptionDef {
    type Schema = Pg2Sqlite;
    type SQLiteEntry = Option<ColumnOptionDef>;

    fn translate(&self, schema: &Self::Schema) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        match &self.option {
            ColumnOption::Unique { is_primary, characteristics } => {
                Ok(Some(ColumnOptionDef {
                    name: self.name.clone(),
                    option: ColumnOption::Unique {
                        is_primary: *is_primary,
                        characteristics: *characteristics,
                    },
                }))
            }
            ColumnOption::Default(expr) => {
                match expr {
                    Expr::Function(func) => {
                        if let Some("CURRENT_TIMESTAMP") =
                            func.name.0.first().and_then(|s| Some(s.as_ident()?.value.as_str()))
                        {
                            return Ok(Some(ColumnOptionDef {
                                name: self.name.clone(),
                                option: ColumnOption::Default(Expr::Function(func.clone())),
                            }));
                        }
                        unimplemented!("The default expression function {func:?} is not supported",)
                    }
                    Expr::Value(value) => {
                        Ok(Some(ColumnOptionDef {
                            name: self.name.clone(),
                            option: ColumnOption::Default(Expr::Value(value.clone())),
                        }))
                    }
                    unimplemented => {
                        unimplemented!(
                            "The default expression {:?} is not supported",
                            unimplemented
                        )
                    }
                }
            }
            ColumnOption::NotNull => Ok(Some(self.clone())),
            ColumnOption::Check(check) => {
                match check {
                    Expr::Function(Function { name, .. }) => {
                        let function_name = name.to_string();
                        if schema.has_function(&function_name) {
                            Ok(Some(self.clone()))
                        } else if schema.should_remove_unsupported_check_constraints() {
                            Ok(None)
                        } else {
                            Err(crate::errors::Error::UndefinedFunction(function_name))
                        }
                    }
                    unimplemented => {
                        unimplemented!("The check expression {unimplemented:?} is not supported")
                    }
                }
            }
            ColumnOption::ForeignKey {
                foreign_table,
                referred_columns,
                on_delete,
                on_update,
                characteristics,
            } => {
                Ok(Some(Self {
                    name: self.name.clone(),
                    option: ColumnOption::ForeignKey {
                        foreign_table: foreign_table.clone(),
                        referred_columns: referred_columns.clone(),
                        on_delete: on_delete
                            .map(|on_delete| on_delete.translate(schema))
                            .transpose()?,
                        on_update: on_update
                            .map(|on_update| on_update.translate(schema))
                            .transpose()?,
                        characteristics: characteristics
                            .map(|c| c.translate(schema))
                            .transpose()?,
                    },
                }))
            }
            unimplemented => {
                unimplemented!("The column option {unimplemented:?} is not supported")
            }
        }
    }
}
