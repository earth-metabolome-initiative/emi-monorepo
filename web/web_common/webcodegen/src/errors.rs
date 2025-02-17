//! Enumeration for the errors that may happen within the webcodegen crate.
use diesel::result::Error as DieselError;
use snake_case_sanitizer::SanitizationErrors;

use crate::{
    codegen::CodeGenerationError, custom_schema_constraints::ConstraintError, Column, PgType, Table,
};

#[derive(Debug)]
/// Enumeration for the errors that may happen within the webcodegen crate.
pub enum WebCodeGenError {
    /// A Diesel error, such as a connection error.
    DieselError(DieselError),
    /// The table is missing.
    MissingTable(String),
    /// A table should not exist.
    IllegalTable(String),
    /// A table should not have an associated roles table.
    IllegalRolesTable(String),
    /// A constraint is missing or invalid.
    ConstraintError(ConstraintError),
    /// A Syn Error occurred.
    SynError(syn::Error),
    /// A column is of an unknown type.
    UnknownColumnType(Box<Column>),
    /// Unknown PostgreSQL Diesel type.
    UnknownDieselPostgresType(String),
    /// Unknown PostgreSQL Rust type.
    UnknownPostgresRustType(String),
    /// A type is not user-defined.
    NotUserDefinedType(String),
    /// A base type is missing.
    MissingBaseType(Box<PgType>),
    /// Failed to sanitize a name.
    SanitizationErrors(SanitizationErrors),
    /// An error occurred during code generation.
    CodeGenerationError(CodeGenerationError),
    /// A table cannot be codegened.
    IllegalTableCodegen(String, String, Box<Table>),
    /// A table has an excessive number of columns (> 64)
    ExcessiveNumberOfColumns(Box<Table>, usize),
    /// A table should have an update_at column.
    MissingUpdateAtColumn(Box<Table>),
    /// A table should have an updated_by column.
    MissingUpdatedByColumn(Box<Table>),
    /// A table has no primary key column(s).
    NoPrimaryKeyColumn(Box<Table>),
    /// The tables necessary for the roles mechanism are incomplete.
    RolesMechanismIncomplete(Box<Table>),
}

impl From<DieselError> for WebCodeGenError {
    fn from(e: DieselError) -> Self {
        WebCodeGenError::DieselError(e)
    }
}

impl From<ConstraintError> for WebCodeGenError {
    fn from(value: ConstraintError) -> Self {
        WebCodeGenError::ConstraintError(value)
    }
}

impl From<SanitizationErrors> for WebCodeGenError {
    fn from(value: SanitizationErrors) -> Self {
        WebCodeGenError::SanitizationErrors(value)
    }
}

impl From<CodeGenerationError> for WebCodeGenError {
    fn from(value: CodeGenerationError) -> Self {
        WebCodeGenError::CodeGenerationError(value)
    }
}

impl From<syn::Error> for WebCodeGenError {
    fn from(value: syn::Error) -> Self {
        WebCodeGenError::SynError(value)
    }
}
