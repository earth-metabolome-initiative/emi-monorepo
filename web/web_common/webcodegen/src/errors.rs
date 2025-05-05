//! Enumeration for the errors that may happen within the webcodegen crate.
use diesel::result::Error as DieselError;
use snake_case_sanitizer::SanitizationErrors;

use crate::{
    CheckConstraint, Column, PgProc, PgType, Table, custom_schema_constraints::ConstraintError,
};

#[derive(Debug)]
/// Enumeration for the errors that may happen within the webcodegen crate.
pub enum WebCodeGenError {
    /// A Diesel error, such as a connection error.
    DieselError(DieselError),
    /// The table is missing.
    MissingTable(String),
    /// The extension is missing.
    MissingExtension(String),
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
    /// A column requested by name could not be found.
    ColumnNotFound(String),
    /// Unknown `PostgreSQL` Diesel type.
    UnknownDieselPostgresType(String),
    /// Unknown `PostgreSQL` Rust type.
    UnknownPostgresRustType(String),
    /// Unknown `PostgreSQL` [`PgProc`].
    UnknownPostgresProc(String),
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
    /// A table should have an `update_at` column.
    MissingUpdateAtColumn(Box<Table>),
    /// A table should have an `updated_by` column.
    MissingUpdatedByColumn(Box<Table>),
    /// A table has no primary key column(s).
    NoPrimaryKeyColumn(Box<Table>),
    /// The tables necessary for the roles mechanism are incomplete.
    RolesMechanismIncomplete(Box<Table>),
    /// When an io error occurs.
    IoError(std::io::Error),
    /// Empty table name.
    EmptyTableName(Box<Table>),
    /// Empty column name.
    EmptyColumnName(Box<Column>),
    /// Error attempting to convert a string to an integer.
    ParseIntError(core::num::ParseIntError),
    /// Error attempting to convert a string to a float.
    ParseFloatError(core::num::ParseFloatError),
    /// Error attempting to convert a string to a boolean.
    ParseBoolError(core::str::ParseBoolError),
    /// An unsupported type casting operation.
    UnsupportedTypeCasting(String, Box<PgType>),
}

unsafe impl Send for WebCodeGenError {}

#[derive(Debug)]
/// Error type for code generation.
pub enum CodeGenerationError {
    /// When the generation directory was not provided.
    GenerationDirectoryNotProvided,
    /// When the user table has not been specified.
    UserTableNotProvided,
    /// When the project table has not been specified.
    ProjectTableNotProvided,
    /// When the teams table has not been specified.
    TeamsTableNotProvided,
    /// When the team members table has not been specified.
    TeamMembersTableNotProvided,
    /// When the team projects table has not been specified.
    TeamProjectsTableNotProvided,
    /// When the check constraints failed to generate.
    CheckConstraintError(CheckConstraintError),
}

#[derive(Debug)]
/// Error type for code generation.
pub enum CheckConstraintError {
    /// When one of the functions is not from the provided extensions.
    FunctionNotFromProvidedExtensions(Box<PgProc>, Box<CheckConstraint>),
    /// When some of the syntax of the [`CheckConstraint`] is not supported.
    UnsupportedSyntax(Box<CheckConstraint>, UnsupportedCheckConstraintErrorSyntax),
    /// When one of the operators is not a plain Rust operator.
    OperatorsNotSupported,
    /// When none of the provided columns are involved in the check clause.
    NoInvolvedColumns(Box<Column>, Box<CheckConstraint>),
    /// When the top-level expression cannot be reduced to a Result-returning
    /// expression.
    TopLevelExpressionNotResult(Box<CheckConstraint>),
}

#[derive(Debug)]
/// Error type for unsupported syntax in a check constraint.
pub enum UnsupportedCheckConstraintErrorSyntax {
    /// When no scoped columns are expected but are found.
    ExpectedNoScopedColumn(usize),
    /// When a column is expected to be a single scoped column but is not.
    ExpectedSingleScopedColumn(usize),
    /// When a column is expected to be a scoped column but is not.
    ExpectedScopedColumn,
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

impl From<std::io::Error> for WebCodeGenError {
    fn from(value: std::io::Error) -> Self {
        WebCodeGenError::IoError(value)
    }
}

impl From<CheckConstraintError> for CodeGenerationError {
    fn from(value: CheckConstraintError) -> Self {
        CodeGenerationError::CheckConstraintError(value)
    }
}

impl From<CheckConstraintError> for WebCodeGenError {
    fn from(value: CheckConstraintError) -> Self {
        let codegen_error: CodeGenerationError = value.into();
        codegen_error.into()
    }
}

impl From<core::num::ParseIntError> for WebCodeGenError {
    fn from(value: core::num::ParseIntError) -> Self {
        WebCodeGenError::ParseIntError(value)
    }
}

impl From<core::num::ParseFloatError> for WebCodeGenError {
    fn from(value: core::num::ParseFloatError) -> Self {
        WebCodeGenError::ParseFloatError(value)
    }
}

impl From<core::str::ParseBoolError> for WebCodeGenError {
    fn from(value: core::str::ParseBoolError) -> Self {
        WebCodeGenError::ParseBoolError(value)
    }
}
