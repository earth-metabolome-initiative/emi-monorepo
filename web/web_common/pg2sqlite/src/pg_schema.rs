//! Submodule defining a simplified schema struct for PostgreSQL.

use sqlparser::ast::CreateFunction;

use crate::traits::Schema;

#[derive(Debug, Clone, Default)]
/// Simplified schema struct for PostgreSQL.
pub struct PgSchema {
    /// The set of functions defined in the schema.
    functions: Vec<CreateFunction>,
}

impl Schema for PgSchema {
    fn has_function(&self, name: &str) -> Option<&CreateFunction> {
        self.functions.iter().find(|f| f.name.to_string().eq_ignore_ascii_case(name))
    }

    fn add_function(&mut self, function: &CreateFunction) {
        if self.has_function(&function.name.to_string()).is_none() {
            self.functions.push(function.clone());
        }
    }
}
