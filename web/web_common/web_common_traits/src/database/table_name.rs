//! Submodule providing the `TableName` trait for struct tables.

/// Trait defining the name of a table in the database.
pub trait TableName {
    /// Constant representing the name of the table.
    const TABLE_NAME: &'static str;
}
