//! Submodule implementing the [`DatabaseLike`] trait.
use std::path::Path;

use sqlparser::ast::{ColumnDef, CreateTable, ForeignKeyConstraint, Statement};

use crate::traits::{DatabaseLike, table::TableLike};

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

        // We sort all tables by their schema and name to ensure a consistent order.
        schema.tables.sort_by_key(|table| {
            (table.table_schema().unwrap_or("").to_owned(), table.table_name().to_owned())
        });

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

        // We sort all tables by their schema and name to ensure a consistent order.
        self.tables.sort_by_key(|table| {
            (table.table_schema().unwrap_or("").to_owned(), table.table_name().to_owned())
        });

        Ok(())
    }
}

impl DatabaseLike for SqlParserDatabase {
    type Table = CreateTable;
    type Column = ColumnDef;
    type ForeignKey = ForeignKeyConstraint;

    fn tables(&self) -> impl Iterator<Item = &Self::Table> {
        self.tables.iter()
    }

    fn table(&self, schema: Option<&str>, table_name: &str) -> &Self::Table {
        // The tables are sorted by schema and name, so we can use binary search.
        let key = (schema, table_name);
        match self
            .tables
            .binary_search_by_key(&key, |table| (table.table_schema(), table.table_name()))
        {
            Ok(index) => &self.tables[index],
            Err(_) => panic!("Table not found: {:?}.{:?}", schema, table_name),
        }
    }
}
