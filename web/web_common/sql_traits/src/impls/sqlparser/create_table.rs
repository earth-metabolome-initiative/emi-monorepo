//! Submodule implementing the [`TableLike`] trait for `sqlparser`'s
//! [`CreateTable`](sqlparser::ast::CreateTable) struct.

use ::sqlparser::ast::{CreateTable, Ident};
use sqlparser::ast::{CheckConstraint, ColumnDef};

use crate::{impls::SqlParserDatabase, traits::TableLike};

impl TableLike for CreateTable {
    type Database = SqlParserDatabase;
    type Column = ColumnDef;
    type CheckConstraint = CheckConstraint;

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
}
