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

    fn table_schema(&self) -> Option<&str> {
        let object_name_parts = &self.name.0;
        if object_name_parts.len() > 1 {
            let schema_part = &object_name_parts[0];
            match schema_part {
                sqlparser::ast::ObjectNamePart::Identifier(Ident { value, .. }) => {
                    Some(value.as_str())
                }
                _ => panic!("Unexpected object name part in CreateTable: {:?}", schema_part),
            }
        } else {
            None
        }
    }

    fn columns<'db>(
        &'db self,
        _database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db,
    {
        self.columns.iter()
    }

    fn primary_key_columns<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db,
    {
        for constraint in &self.constraints {
            if let sqlparser::ast::TableConstraint::PrimaryKey(pk_constraint) = constraint {
                let mut primary_key_columns = Vec::new();
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
                    primary_key_columns.extend(self.column_by_name(&column_name.value, database));
                }
                return primary_key_columns.into_iter();
            }
        }

        for column in &self.columns {
            for option in &column.options {
                if let sqlparser::ast::ColumnOption::PrimaryKey(_) = option.option {
                    return vec![column].into_iter();
                }
            }
        }

        Vec::new().into_iter()
    }

    fn unique_indices<'db>(
        &'db self,
        _database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::UniqueIndex>
    where
        Self: 'db,
    {
        self.columns
            .iter()
            .filter_map(|col| {
                col.options.iter().find_map(|opt| {
                    if let sqlparser::ast::ColumnOption::Unique(unique_constraint) = &opt.option {
                        Some(unique_constraint)
                    } else {
                        None
                    }
                })
            })
            .chain(self.constraints.iter().filter_map(move |constraint| {
                if let sqlparser::ast::TableConstraint::Unique(unique_constraint) = constraint {
                    Some(unique_constraint)
                } else {
                    None
                }
            }))
    }

    fn check_constraints<'db>(
        &'db self,
        _database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::CheckConstraint>
    where
        Self: 'db,
    {
        self.columns
            .iter()
            .filter_map(|col| {
                col.options.iter().find_map(|opt| {
                    if let sqlparser::ast::ColumnOption::Check(check_constraint) = &opt.option {
                        Some(check_constraint)
                    } else {
                        None
                    }
                })
            })
            .chain(self.constraints.iter().filter_map(move |constraint| {
                if let sqlparser::ast::TableConstraint::Check(check_constraint) = constraint {
                    Some(check_constraint)
                } else {
                    None
                }
            }))
    }

    fn foreign_keys<'db>(
        &'db self,
        _database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::ForeignKey>
    where
        Self: 'db,
    {
        self.columns
            .iter()
            .filter_map(|col| {
                col.options.iter().find_map(|opt| {
                    if let sqlparser::ast::ColumnOption::ForeignKey(foreign_key_constraint) =
                        &opt.option
                    {
                        Some(foreign_key_constraint)
                    } else {
                        None
                    }
                })
            })
            .chain(self.constraints.iter().filter_map(move |constraint| {
                if let sqlparser::ast::TableConstraint::ForeignKey(fk_constraint) = constraint {
                    Some(fk_constraint)
                } else {
                    None
                }
            }))
    }
}
