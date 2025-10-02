//! Submodule implementing the [`TableLike`] trait for `sqlparser`'s
//! [`CreateTable`](sqlparser::ast::CreateTable) struct.

use ::sqlparser::ast::{CreateTable, Ident};

use crate::traits::TableLike;

impl TableLike for CreateTable {
    fn table_name(&self) -> &str {
        let object_name_parts = &self.name.0;
        let last_object_name_parts = &object_name_parts[object_name_parts.len() - 1];
        match last_object_name_parts {
            sqlparser::ast::ObjectNamePart::Identifier(Ident { value, .. }) => value.as_str(),
            _ => panic!("Unexpected object name part in CreateTable: {:?}", last_object_name_parts),
        }
    }
}
