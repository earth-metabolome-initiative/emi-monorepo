//! Implementation of the [`Translator`] trait for the
//! [`Column`](sqlparser::ast::Column) type.

use sql_traits::structs::ParserDB;
use sqlparser::ast::{ColumnOption, ColumnOptionDef, Expr, ForeignKeyConstraint};

use crate::prelude::{Pg2SqliteOptions, Translator};

impl Translator for ColumnOptionDef {
    type Schema = ParserDB;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = Option<ColumnOptionDef>;

    fn translate(
        &self,
        schema: &Self::Schema,
        options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        match &self.option {
            ColumnOption::Unique(unique_constraint) => {
                Ok(Some(ColumnOptionDef {
                    name: self.name.clone(),
                    option: unique_constraint.clone().into(),
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
                        // SQLite does not support methods such as `gen_random_uuid()`, therefore
                        // we return `None` if we encounter such a function.
                        if func.name.0.first().and_then(|s| Some(s.as_ident()?.value.as_str()))
                            == Some("gen_random_uuid")
                        {
                            return Ok(None);
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
            ColumnOption::Check(_) => Ok(None),
            ColumnOption::ForeignKey(ForeignKeyConstraint {
                name,
                index_name,
                columns,
                match_kind,
                foreign_table,
                referred_columns,
                on_delete,
                on_update,
                characteristics,
            }) => {
                Ok(Some(Self {
                    name: self.name.clone(),
                    option: ForeignKeyConstraint {
                        name: name.clone(),
                        index_name: index_name.clone(),
                        columns: columns.clone(),
                        match_kind: match_kind.clone(),
                        foreign_table: foreign_table.clone(),
                        referred_columns: referred_columns.clone(),
                        on_delete: on_delete
                            .map(|on_delete| on_delete.translate(schema, options))
                            .transpose()?,
                        on_update: on_update
                            .map(|on_update| on_update.translate(schema, options))
                            .transpose()?,
                        characteristics: characteristics
                            .map(|c| c.translate(schema, options))
                            .transpose()?,
                    }
                    .into(),
                }))
            }
            unimplemented => {
                unimplemented!("The column option {unimplemented:?} is not supported")
            }
        }
    }
}
