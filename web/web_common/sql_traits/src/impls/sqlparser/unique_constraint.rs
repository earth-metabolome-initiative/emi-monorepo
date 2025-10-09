//! Implement the [`UniqueConstraint`] trait for the `sqlparser` crate's

use sqlparser::{
    ast::{ColumnDef, CreateTable, Expr, UniqueConstraint},
    parser::Parser,
};

use crate::{impls::SqlParserDatabase, traits::UniqueIndexLike};

impl UniqueIndexLike for UniqueConstraint {
    type Table = CreateTable;
    type Database = SqlParserDatabase;
    type Column = ColumnDef;

    fn expression(&self, _database: &Self::Database) -> Expr {
        let expression_string = format!(
            "({})",
            self.columns
                .iter()
                .map(|ident| ident.column.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        Parser::new(&sqlparser::dialect::GenericDialect {})
            .try_with_sql(expression_string.as_str())
            .expect("Failed to parse unique constraint expression")
            .parse_expr()
            .expect("No expression found in parsed unique constraint")
    }
}
