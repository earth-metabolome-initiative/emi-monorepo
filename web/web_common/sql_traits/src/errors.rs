//! Error enumeration used in the `sql_traits` crate.

use sqlparser::parser::ParserError;

#[derive(Debug, thiserror::Error)]
/// Defines the `Error` enum representing various error types
pub enum Error {
    #[error("Unknown column `{column_name}` in table `{table_name}`.")]
    /// A check constraint contained columns which do not exist in the table.
    UnknownColumnInCheckConstraint {
        /// Name of the unknown column.
        column_name: String,
        /// Name of the table the check constraint belongs to.
        table_name: String,
    },
    #[error(
        "Referenced table `{referenced_table}` not found for foreign key in table `{host_table}`."
    )]
    /// Error indicating that a foreign key references a table that does not
    /// exist.
    ReferencedTableNotFoundForForeignKey {
        /// Name of the referenced table.
        referenced_table: String,
        /// Name of the host table containing the foreign key.
        host_table: String,
    },
    #[error(
        "Referenced column `{referenced_column}` not found in table `{referenced_table}` for foreign key in table `{host_table}`."
    )]
    /// Error indicating that a foreign key references a column that does not
    /// exist.
    ReferencedColumnNotFoundForForeignKey {
        /// Name of the referenced column.
        referenced_column: String,
        /// Name of the referenced table.
        referenced_table: String,
        /// Name of the host table containing the foreign key.
        host_table: String,
    },
    #[error("Host column `{host_column}` not found in table `{host_table}` for foreign key.")]
    /// Error indicating that a foreign key references a host column that does
    /// not exist.
    HostColumnNotFoundForForeignKey {
        /// Name of the host column.
        host_column: String,
        /// Name of the host table containing the foreign key.
        host_table: String,
    },
    /// Wrapper around SQL parser errors.
    #[error("SQL parser error: {0}")]
    SqlParserError(#[from] ParserError),
}
