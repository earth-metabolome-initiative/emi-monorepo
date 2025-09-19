//! Submodule defining a simplified schema struct for `PostgreSQL`.

use sqlparser::ast::{CreateFunction, CreateTable};

use crate::traits::Schema;

#[derive(Debug, Clone, Default)]
/// Simplified schema struct for `PostgreSQL`.
pub struct PgSchema {
    /// The set of functions defined in the schema.
    functions: Vec<CreateFunction>,
    /// The set of tables defined in the schema.
    tables: Vec<CreateTable>,
}

impl Schema for PgSchema {
    fn function(&self, name: &str) -> Option<&CreateFunction> {
        self.functions.iter().find(|f| f.name.to_string().eq_ignore_ascii_case(name))
    }

    fn table(&self, name: &str) -> Option<&CreateTable> {
        self.tables.iter().find(|t| t.name.to_string().eq_ignore_ascii_case(name))
    }

    fn add_function(&mut self, function: &CreateFunction) {
        if self.function(&function.name.to_string()).is_none() {
            self.functions.push(function.clone());
        }
    }

    fn add_table(&mut self, table: &CreateTable) {
        if self.table(&table.name.to_string()).is_none() {
            self.tables.push(table.clone());
        }
    }
}
