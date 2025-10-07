//! Submodule implementing the [`TableLike`] trait for `sqlparser`'s
//! [`CreateTable`](sqlparser::ast::CreateTable) struct.

use ::sqlparser::ast::{CreateTable, Ident};
use sqlparser::ast::{CheckConstraint, ColumnDef, Expr, ForeignKeyConstraint, UniqueConstraint};

use crate::{impls::SqlParserDatabase, traits::TableLike};

impl TableLike for CreateTable {
    type Database = SqlParserDatabase;
    type Column = ColumnDef;
    type CheckConstraint = CheckConstraint;
    type UniqueIndex = UniqueConstraint;
    type ForeignKey = ForeignKeyConstraint;

    fn table_name(&self) -> &str {
        let object_name_parts = &self.name.0;
        let last_object_name_parts = &object_name_parts[object_name_parts.len() - 1];
        match last_object_name_parts {
            sqlparser::ast::ObjectNamePart::Identifier(Ident { value, .. }) => value.as_str(),
            _ => panic!("Unexpected object name part in CreateTable: {:?}", last_object_name_parts),
        }
    }

    fn columns(&self, _database: &Self::Database) -> impl Iterator<Item = Self::Column> {
        self.columns.clone().into_iter()
    }

    fn primary_key_columns(
        &self,
        _database: &Self::Database,
    ) -> impl Iterator<Item = Self::Column> {
        for constraint in &self.constraints {
            if let sqlparser::ast::TableConstraint::PrimaryKey(pk_constraint) = constraint {
                for col_name in &pk_constraint.columns {
                    let column_name = match &col_name.column.expr {
                        Expr::Identifier(ident) => ident,
                        _ => {
                            unreachable!(
                                "Unexpected expression in primary key column: {:?}",
                                col_name
                            )
                        }
                    };
                    let mut primary_key_columns = Vec::new();
                    if let Some(column) = self.columns.iter().find(|col| &col.name == column_name) {
                        primary_key_columns.push(column.clone());
                    }
                    return primary_key_columns.into_iter();
                }
            }
        }

        for column in &self.columns {
            for option in &column.options {
                if let sqlparser::ast::ColumnOption::Unique { is_primary: true, .. } = option.option
                {
                    return vec![column.clone()].into_iter();
                }
            }
        }

        Vec::new().into_iter()
    }

    fn unique_indexes(
        &self,
        _database: &Self::Database,
    ) -> impl Iterator<Item = Self::UniqueIndex> {
        self.constraints.iter().filter_map(move |constraint| {
            if let sqlparser::ast::TableConstraint::Unique(unique_constraint) = constraint {
                Some(unique_constraint.clone())
            } else {
                None
            }
        })
    }

    fn check_constraints(
        &self,
        _database: &Self::Database,
    ) -> impl Iterator<Item = Self::CheckConstraint> {
        self.constraints.iter().filter_map(move |constraint| {
            if let sqlparser::ast::TableConstraint::Check(check_constraint) = constraint {
                Some(check_constraint.clone())
            } else {
                None
            }
        })
    }

    fn foreign_keys(&self, _database: &Self::Database) -> impl Iterator<Item = Self::ForeignKey> {
        self.constraints
            .iter()
            .filter_map(move |constraint| {
                if let sqlparser::ast::TableConstraint::ForeignKey(fk_constraint) = constraint {
                    Some(fk_constraint.clone())
                } else {
                    None
                }
            })
            .chain(self.columns.iter().filter_map(|col| {
                col.options.iter().find_map(|opt| {
                    if let sqlparser::ast::ColumnOption::ForeignKey {
                        foreign_table,
                        referred_columns,
                        on_delete,
                        on_update,
                        characteristics,
                    } = opt.option.clone()
                    {
                        let fk = ForeignKeyConstraint {
                            name: None,
                            index_name: None,
                            columns: vec![col.name.clone()],
                            foreign_table,
                            referred_columns,
                            on_delete,
                            on_update,
                            characteristics,
                        };
                        Some(fk)
                    } else {
                        None
                    }
                })
            }))
    }
}
