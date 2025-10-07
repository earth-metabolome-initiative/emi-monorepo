//! Submodule implementing the [`DatabaseLike`] trait.
use std::path::Path;

use sqlparser::ast::{ColumnDef, CreateTable, Statement};

use crate::traits::DatabaseLike;

#[derive(Debug, Default)]
/// Struct representing a SQL schema parsed using the `sqlparser` crate.
pub struct SqlParserDatabase {
    /// Vector of tables in the schema.
    tables: Vec<CreateTable>,
}

impl SqlParserDatabase {
    /// Recursively visits a directory, parsing all `.sql` files found
    /// and integrating their SQL statements into the schema.
    pub fn from_directory(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut schema = SqlParserDatabase::default();

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().extension().map(|e| e == "sql").unwrap_or(false) {
                let content = std::fs::read_to_string(entry.path())?;
                schema.parse_sql(&content)?;
            }

            if entry.path().is_dir() {
                let sub_schema = SqlParserDatabase::from_directory(&entry.path())?;
                schema.tables.extend(sub_schema.tables);
            }
        }

        Ok(schema)
    }

    /// Creates a new `SqlParserDatabase` by parsing SQL statements from a
    /// string.
    pub fn from_sql(sql: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut schema = SqlParserDatabase::default();
        schema.parse_sql(sql)?;
        Ok(schema)
    }

    /// Parses and integrates SQL statements from a string into the schema.
    pub fn parse_sql(&mut self, sql: &str) -> Result<(), Box<dyn std::error::Error>> {
        let dialect = sqlparser::dialect::GenericDialect {};
        let statements = sqlparser::parser::Parser::parse_sql(&dialect, sql)?;
        for statement in &statements {
            if let Statement::CreateTable(create_table) = statement {
                self.tables.push(create_table.clone());
            }
        }

        Ok(())
    }
}

impl DatabaseLike for SqlParserDatabase {
    type Table = CreateTable;
    type Column = ColumnDef;

    fn tables(&self) -> impl Iterator<Item = &Self::Table> {
        self.tables.iter()
    }
}
