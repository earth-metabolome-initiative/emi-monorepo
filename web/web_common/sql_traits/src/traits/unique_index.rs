//! Submodule definining the `UniqueIndexLike` trait for SQL unique
//! indexes.

use sqlparser::ast::{Expr, Ident};

use crate::traits::{ColumnLike, DatabaseLike, TableLike};

pub(crate) fn collect_column_idents(expr: Expr) -> Vec<Ident> {
    match expr {
        Expr::Identifier(ident) => vec![ident],
        Expr::CompoundIdentifier(idents) => idents,
        Expr::BinaryOp { left, right, .. } => {
            let mut cols = collect_column_idents(*left);
            cols.extend(collect_column_idents(*right));
            cols
        }
        Expr::UnaryOp { expr, .. } => collect_column_idents(*expr),
        Expr::Nested(inner) => collect_column_idents(*inner),
        _ => unimplemented!("Unhandled expression type in unique index: {:?}", expr),
    }
}

/// A unique index is a rule that specifies that the values in a column
/// (or a group of columns) must be unique across all rows in a table.
/// This trait represents such a unique index in a database-agnostic way.
pub trait UniqueIndexLike {
    /// The table type the unique index belongs to.
    type Table: TableLike<Database = Self::Database, UniqueIndex = Self, Column = Self::Column>;
    /// The column type the unique index belongs to.
    type Column: ColumnLike;
    /// The database type the unique index belongs to.
    type Database: DatabaseLike<Table = Self::Table, Column = Self::Column>;

    /// Returns the expression of the unique index as an SQL AST node.
    fn expression(&self, database: &Self::Database) -> Expr;

    /// Returns whether the unique index is defined using simply columns
    /// and no other expressions.
    fn is_simple(&self, database: &Self::Database) -> bool {
        let Expr::Nested(inner) = self.expression(database) else {
            unreachable!("Unique index expression must always be nested");
        };
        matches!(*inner, Expr::Identifier(_) | Expr::CompoundIdentifier(_))
    }

    /// Returns the columns which appear in the unique index.
    fn columns<'db>(
        &'db self,
        database: &'db Self::Database,
        table: &'db Self::Table,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db,
    {
        let Expr::Nested(inner) = self.expression(database) else {
            unreachable!("Unique index expression must always be nested");
        };
        collect_column_idents(*inner)
            .into_iter()
            .filter_map(|ident| table.column_by_name(&ident.value, database))
    }
}
