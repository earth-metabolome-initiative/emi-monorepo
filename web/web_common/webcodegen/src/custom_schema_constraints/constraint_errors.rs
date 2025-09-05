//! Submodule defining an error enumeration for the custom constraints.
use std::fmt::{Display, Formatter};

use sqlparser::ast::CascadeOption;

use crate::{CheckConstraint, Column, KeyColumnUsage, Table};

fn format_column_list(columns: &Vec<Column>) -> String {
    format!("({})", columns.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", "))
}

#[derive(Debug)]
/// Error type for custom schema constraints.
pub enum ConstraintError {
    /// The column is unexpectedly nullable.
    UnexpectedNullableColumn(Box<Column>),
    /// The column is unexpectedly non-nullable.
    UnexpectedNonNullableColumn(Box<Column>),
    /// The column name is unexpectedly uppercase.
    UnexpectedUppercaseColumn(Box<Column>),
    /// Missing column in a table.
    MissingColumn {
        /// The table missing the column.
        table: Box<Table>,
        /// The name of the missing column.
        column_name: String,
    },
    /// A column was found and it was not expected.
    UnexpectedColumn(Box<Column>),
    /// A foreign key was found and it was not expected.
    UnexpectedForeignKey {
        /// The columns that compose the foreign key.
        columns: Vec<Column>,
        /// The referenced columns.
        referenced_columns: Vec<Column>,
    },
    /// Missing unique index on a table.
    MissingUniqueIndex {
        /// The table missing the unique index.
        table: Box<Table>,
        /// The columns that should compose the unique index.
        columns: Vec<Column>,
    },
    /// Missing foreign key constraint on a table.
    MissingForeignKey {
        /// The columns that should compose the foreign key.
        columns: Vec<Column>,
        /// The referenced columns.
        referenced_columns: Vec<Column>,
        /// The foreign key cascade option (e.g., cascade on delete).
        cascade_option: CascadeOption,
    },
    /// The table name is unexpectedly uppercase.
    UnexpectedUppercaseTable(Box<Table>),
    /// A deprecated word is used in a column name.
    ColumnWordDeprecationError {
        /// The column name where the deprecated word was used.
        column: Box<Column>,
        /// The deprecated word that was used.
        deprecated_word: String,
    },
    /// A deprecated word is used in a table name.
    TableWordDeprecationError {
        /// The table name where the deprecated word was used.
        table: Box<Table>,
        /// The deprecated word that was used.
        deprecated_word: String,
    },
    /// The column and foreign column types are incompatible.
    IncompatibleForeignType {
        /// The column
        column: Box<Column>,
        /// The foreign column
        foreign_column: Box<Column>,
    },
    /// The column is not a foreign key column.
    NotForeignKeyColumn {
        /// The name of the table
        table_name: String,
        /// The name of the column
        column_name: String,
    },
    /// The column is not of the expected type.
    NotOfCorrectType {
        /// The column which failed the check constraint
        column: Box<Column>,
        /// The expected type of the column
        expected_column_type: String,
    },
    /// The column does not have the expected name.
    DoesNotHaveExpectedName {
        /// The column which failed the check constraint
        column: Box<Column>,
        /// The expected name of the column
        expected_name: String,
    },
    /// The column does not have the expected suffix.
    DoesNotHaveExpectedSuffix {
        /// The column which failed the check constraint
        column: Box<Column>,
        /// The expected suffix of the column
        expected_suffix: String,
    },
    /// The column does not have the expected prefix.
    DoesNotHaveExpectedPrefix {
        /// The column which failed the check constraint
        column: Box<Column>,
        /// The expected prefix of the column
        expected_prefix: String,
    },
    /// The column does not have a sibling column.
    DoesNotHaveSiblingColumn {
        /// The name of the column
        column: Box<Column>,
        /// The name of the sibling column
        sibling_column_name: String,
    },
    /// The same_as constraint is set to cascade on delete, which is not
    /// allowed.
    SameAsConstraintMustNotCascade(Box<KeyColumnUsage>),
    /// Redundant foreign keys detected.
    RedundantForeignKeys {
        /// The first foreign key
        foreign_key_1: Box<KeyColumnUsage>,
        /// The second foreign key
        foreign_key_2: Box<KeyColumnUsage>,
        /// Local columns of the first foreign key
        local_columns_1: Vec<Column>,
        /// Local columns of the second foreign key
        local_columns_2: Vec<Column>,
        /// Foreign columns of the first foreign key
        foreign_columns_1: Vec<Column>,
        /// Foreign columns of the second foreign key
        foreign_columns_2: Vec<Column>,
    },
    /// Redundant unique indices detected.
    RedundantUniqueIndices {
        /// Table containing the indices.
        table: Box<Table>,
        /// The columns composing the unique index.
        columns: Vec<Column>,
    },
    /// Redoundant check constraints detected.
    DuplicatedCheckConstraint {
        /// The table containing the duplicated check constraint.
        table: Box<Table>,
        /// The duplicated check constraint.
        check_constraint: Box<CheckConstraint>,
    },
    /// A foreign key has unexpected cascading behavior.
    ForeignKeyWithUnexpectedCascadingBehavior {
        /// The columns with the foreign key.
        columns: Vec<Column>,
        /// The expected cascading behavior.
        expected_behavior: CascadeOption,
        /// The found cascading behavior.
        found_behavior: CascadeOption,
    },
}

impl Display for ConstraintError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ConstraintError::UnexpectedNullableColumn(column) => {
                write!(f, "Unexpected nullable column: {column}")
            }
            ConstraintError::UnexpectedNonNullableColumn(column) => {
                write!(f, "Unexpected non-nullable column: {column}")
            }
            ConstraintError::ColumnWordDeprecationError { column, deprecated_word } => {
                write!(f, "The column {column} uses the deprecated word '{deprecated_word}'",)
            }
            ConstraintError::MissingColumn { table, column_name } => {
                write!(f, "The table {table} is missing the expected column `{column_name}`.",)
            }
            ConstraintError::UnexpectedColumn(column) => {
                write!(f, "Unexpected column found: {column}, you may want to remove it.")
            }
            ConstraintError::UnexpectedForeignKey { columns, referenced_columns } => {
                write!(
                    f,
                    "Unexpected foreign key found in table {}: FOREIGN KEY ({}) REFERENCES {}({})",
                    columns[0].table_name,
                    columns
                        .iter()
                        .map(|c| c.column_name.clone())
                        .collect::<Vec<String>>()
                        .join(", "),
                    referenced_columns[0].table_name,
                    referenced_columns
                        .iter()
                        .map(|c| c.column_name.clone())
                        .collect::<Vec<String>>()
                        .join(", "),
                )
            }
            ConstraintError::MissingUniqueIndex { table, columns } => {
                write!(
                    f,
                    "The table {table} is missing a unique index on columns: {}",
                    format_column_list(columns)
                )
            }
            ConstraintError::MissingForeignKey { columns, referenced_columns, cascade_option } => {
                write!(
                    f,
                    "In table {}, you are missing: FOREIGN KEY ({}) REFERENCES {}({}){}",
                    columns[0].table_name,
                    columns
                        .iter()
                        .map(|c| c.column_name.clone())
                        .collect::<Vec<String>>()
                        .join(", "),
                    referenced_columns[0].table_name,
                    referenced_columns
                        .iter()
                        .map(|c| c.column_name.clone())
                        .collect::<Vec<String>>()
                        .join(", "),
                    if *cascade_option != CascadeOption::Restrict {
                        format!(" ON DELETE {}", cascade_option)
                    } else {
                        "".to_string()
                    }
                )
            }
            ConstraintError::TableWordDeprecationError { table, deprecated_word } => {
                write!(
                    f,
                    "The table `{}` uses the deprecated word '{}'",
                    table.table_name, deprecated_word
                )
            }
            ConstraintError::UnexpectedUppercaseColumn(column_name) => {
                write!(f, "Unexpected uppercase column: {column_name}")
            }
            ConstraintError::UnexpectedUppercaseTable(table_name) => {
                write!(f, "Unexpected uppercase table: {table_name}")
            }
            ConstraintError::IncompatibleForeignType { column, foreign_column } => {
                write!(
                    f,
                    "Column {column} is of type {column_type}, foreign column {foreign_column} is of type {foreign_column_type}",
                    column_type = column.raw_data_type(),
                    foreign_column_type = foreign_column.raw_data_type(),
                )
            }
            ConstraintError::NotForeignKeyColumn { table_name, column_name } => {
                write!(f, "Column {column_name} in table {table_name} is not a foreign key column",)
            }
            ConstraintError::NotOfCorrectType { column, expected_column_type } => {
                write!(
                    f,
                    "Column {column} is of type {}, expected {expected_column_type}",
                    column.raw_data_type(),
                )
            }
            ConstraintError::DoesNotHaveExpectedName { column, expected_name } => {
                write!(f, "Column {column} does not have the expected name \"{expected_name}\"",)
            }
            ConstraintError::DoesNotHaveExpectedSuffix { column, expected_suffix } => {
                write!(f, "Column {column} does not have the expected suffix \"{expected_suffix}\"",)
            }
            ConstraintError::DoesNotHaveExpectedPrefix { column, expected_prefix } => {
                write!(f, "Column {column} does not have the expected prefix \"{expected_prefix}\"",)
            }
            ConstraintError::DoesNotHaveSiblingColumn { column, sibling_column_name } => {
                write!(
                    f,
                    "Column {column} does not have the expected sibling column `{}.{}.{sibling_column_name}`",
                    column.table_schema, column.table_name,
                )
            }
            ConstraintError::SameAsConstraintMustNotCascade(same_as_constraint) => {
                write!(
                    f,
                    "The same_as constraint `{}.{}` on table `{}` must not be set to cascade on delete.",
                    same_as_constraint.constraint_catalog,
                    same_as_constraint.constraint_name,
                    same_as_constraint.table_name
                )
            }
            ConstraintError::RedundantForeignKeys {
                foreign_key_1,
                foreign_key_2,
                local_columns_1,
                local_columns_2,
                foreign_columns_1,
                foreign_columns_2,
            } => {
                let local_cols_1 = format_column_list(local_columns_1);
                let local_cols_2 = format_column_list(local_columns_2);
                let foreign_cols_1 = format_column_list(foreign_columns_1);
                let foreign_cols_2 = format_column_list(foreign_columns_2);
                write!(
                    f,
                    "Redundant foreign keys detected: `{}.{}` [{local_cols_1} -> {foreign_cols_1}] and `{}.{}` [{local_cols_2} -> {foreign_cols_2}]",
                    foreign_key_1.table_name,
                    foreign_key_1.constraint_name,
                    foreign_key_2.table_name,
                    foreign_key_2.constraint_name,
                )
            }
            ConstraintError::RedundantUniqueIndices { table, columns } => {
                let cols = format_column_list(columns);
                write!(f, "Redundant unique indices detected on table {table} for columns {cols}")
            }
            ConstraintError::DuplicatedCheckConstraint { table, check_constraint } => {
                write!(
                    f,
                    "Duplicated check constraint detected on table {table}: clause `{}`",
                    check_constraint.check_clause
                )
            }
            ConstraintError::ForeignKeyWithUnexpectedCascadingBehavior {
                columns,
                expected_behavior,
                found_behavior,
            } => {
                write!(
                    f,
                    "Foreign key on {} has unexpected cascading behavior. Expected: `{expected_behavior}`, found: `{found_behavior}`",
                    format_column_list(columns),
                )
            }
        }
    }
}

impl std::error::Error for ConstraintError {}
