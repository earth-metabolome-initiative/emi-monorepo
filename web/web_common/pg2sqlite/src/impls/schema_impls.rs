//! Submodule implementing the schema for the translation between `PostgreSQL` and
//! `SQLite`.

use crate::prelude::{Pg2Sqlite, Schema};

impl Schema for Pg2Sqlite {
    fn has_function(&self, name: &str) -> bool {
        for statement in &self.pg_statements {
            match statement {
                sqlparser::ast::Statement::CreateFunction(create_function) => {
                    if create_function.name.to_string().to_lowercase() == name.to_lowercase()
                        && create_function
                            .language
                            .as_ref()
                            .is_none_or(|language| language.value.to_lowercase() != "sql")
                    {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn should_remove_unsupported_check_constraints(&self) -> bool {
        self.remove_unsupported_check_constraints
    }
}
