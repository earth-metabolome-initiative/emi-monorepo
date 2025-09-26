//! Submodule implementing traits from `sql_constraints` crate for the [`CreateTable`](sqlparser::ast::CreateTable) struct from the `sqlparser` crate.

use sqlparser::ast::{CreateTable, Ident};

use crate::traits::{Constrainable, ConstrainableTable};

impl Constrainable for CreateTable {}

impl ConstrainableTable for CreateTable {
    fn name(&self) -> &str {
        let object_name_parts = &self.name.0;
        let last_object_name_parts = &object_name_parts[object_name_parts.len() - 1];
        match last_object_name_parts {
            sqlparser::ast::ObjectNamePart::Identifier(Ident { value, .. }) => value.as_str(),
            _ => panic!("Unexpected object name part in CreateTable: {:?}", last_object_name_parts),
        }
    }
}
