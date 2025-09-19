//! Enumeration for the errors that may happen within the webcodegen crate.
use std::fmt::Display;

use cached::DiskCacheError;
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
    /// The column does not have a default value.
    ColumnDoesNotHaveDefaultValue(Box<Column>),
    /// Something failed with the Disk Cache.
    DiskCache(DiskCacheError),
}

impl Display for WebCodeGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebCodeGenError::DieselError(err) => write!(f, "Diesel error: {err}"),
            WebCodeGenError::MissingTable(table) => write!(f, "Missing table: {table}",),
            WebCodeGenError::MissingExtension(extension) => {
                write!(f, "Missing extension: {extension}")
            }
            WebCodeGenError::IllegalTable(table) => write!(f, "Illegal table: {table}",),
            WebCodeGenError::IllegalRolesTable(table) => {
                write!(f, "Illegal roles table: {table}",)
            }
            WebCodeGenError::ConstraintError(err) => write!(f, "Constraint error: {err}"),
            WebCodeGenError::SynError(err) => write!(f, "Syn error: {err}"),
            WebCodeGenError::UnknownColumnType(column) => {
                write!(
                    f,
                    "Unknown column type: `{}.{}` (ID: {})",
                    column.table_name,
                    column.column_name,
                    column.raw_data_type()
                )
            }
            WebCodeGenError::ColumnNotFound(name) => write!(f, "Column not found: {name}"),
            WebCodeGenError::UnknownDieselPostgresType(ty) => {
                write!(f, "Unknown Diesel PostgreSQL type: {ty}")
            }
            WebCodeGenError::UnknownPostgresRustType(ty) => {
                write!(f, "Unknown PostgreSQL Rust type: {ty}")
            }
            WebCodeGenError::UnknownPostgresProc(proc) => {
                write!(f, "Unknown PostgreSQL procedure: {proc}")
            }
            WebCodeGenError::NotUserDefinedType(ty) => {
                write!(f, "Type is not user-defined: {ty}")
            }
            WebCodeGenError::MissingBaseType(ty) => {
                write!(
                    f,
                    "Missing base type for type: `{}.{}` (ID: {})",
                    ty.typnamespace, ty.typname, ty.oid
                )
            }
            WebCodeGenError::SanitizationErrors(errs) => {
                write!(f, "Sanitization errors: {errs}")
            }
            WebCodeGenError::CodeGenerationError(err) => {
                write!(f, "Code generation error: {err}")
            }
            WebCodeGenError::IllegalTableCodegen(reason, table_name, table) => {
                write!(
                    f,
                    "Illegal table codegen for table `{table_name}`: {reason}. Table details: {table}",
                )
            }
            WebCodeGenError::ExcessiveNumberOfColumns(table, count) => {
                write!(
                    f,
                    "Table `{}` has an excessive number of columns: {count}",
                    table.table_name
                )
            }
            WebCodeGenError::MissingUpdateAtColumn(table) => {
                write!(f, "Table `{table}` is missing 'updated_at' column")
            }
            WebCodeGenError::MissingUpdatedByColumn(table) => {
                write!(f, "Table `{table}` is missing 'updated_by' column")
            }
            WebCodeGenError::NoPrimaryKeyColumn(table) => {
                write!(f, "Table `{table}` has no primary key column(s)")
            }
            WebCodeGenError::RolesMechanismIncomplete(table) => {
                write!(f, "Table `{table}` has incomplete roles mechanism tables")
            }
            WebCodeGenError::IoError(err) => write!(f, "I/O error: {err}"),
            WebCodeGenError::EmptyTableName(table) => {
                write!(f, "Table has an empty name: {table}")
            }
            WebCodeGenError::EmptyColumnName(column) => {
                write!(f, "Column has an empty name: {column}")
            }
            WebCodeGenError::ParseIntError(err) => write!(f, "Parse int error: {err}"),
            WebCodeGenError::ParseFloatError(err) => write!(f, "Parse float error: {err}"),
            WebCodeGenError::ParseBoolError(err) => write!(f, "Parse bool error: {err}"),
            WebCodeGenError::UnsupportedTypeCasting(target_type, source_type) => {
                write!(
                    f,
                    "Unsupported type casting from `{}` to type: `{}.{}` (ID: {})",
                    target_type, source_type.typnamespace, source_type.typname, source_type.oid
                )
            }
            WebCodeGenError::ColumnDoesNotHaveDefaultValue(column) => {
                write!(f, "Column `{column}` does not have a default value",)
            }
            WebCodeGenError::DiskCache(err) => write!(f, "Disk cache error: {err}"),
        }
    }
}

impl core::error::Error for WebCodeGenError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            WebCodeGenError::DieselError(err) => Some(err),
            WebCodeGenError::ConstraintError(err) => Some(err),
            WebCodeGenError::SynError(err) => Some(err),
            WebCodeGenError::SanitizationErrors(err) => Some(err),
            WebCodeGenError::CodeGenerationError(err) => Some(err),
            WebCodeGenError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

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

impl Display for CodeGenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeGenerationError::GenerationDirectoryNotProvided => {
                write!(f, "Generation directory was not provided")
            }
            CodeGenerationError::UserTableNotProvided => {
                write!(f, "User table was not provided")
            }
            CodeGenerationError::ProjectTableNotProvided => {
                write!(f, "Project table was not provided")
            }
            CodeGenerationError::TeamsTableNotProvided => {
                write!(f, "Teams table was not provided")
            }
            CodeGenerationError::TeamMembersTableNotProvided => {
                write!(f, "Team members table was not provided")
            }
            CodeGenerationError::TeamProjectsTableNotProvided => {
                write!(f, "Team projects table was not provided")
            }
            CodeGenerationError::CheckConstraintError(err) => {
                write!(f, "Check constraint error: {err}")
            }
        }
    }
}

impl core::error::Error for CodeGenerationError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            CodeGenerationError::CheckConstraintError(err) => Some(err),
            _ => None,
        }
    }
}

#[derive(Debug)]
/// Error type for code generation.
pub enum CheckConstraintError {
    /// When one of the functions is not from the provided extensions.
    FunctionNotFromProvidedExtensions(Box<PgProc>, Box<CheckConstraint>),
    /// When the check constraint does not contain any function calls, and
    /// therefore it is not clear how to convert it into a rust function.
    NoFunctionCalls(Box<CheckConstraint>),
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

impl Display for CheckConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FunctionNotFromProvidedExtensions(proc, constraint) => {
                write!(
                    f,
                    "Function `{}` in check constraint `{}.{}` is not from the provided extensions",
                    proc.proname, constraint.constraint_schema, constraint.constraint_name
                )
            }
            Self::NoFunctionCalls(constraint) => {
                write!(
                    f,
                    "Check constraint `{}.{}` does not contain any function calls",
                    constraint.constraint_schema, constraint.constraint_name
                )
            }
            Self::UnsupportedSyntax(constraint, syntax_error) => {
                write!(
                    f,
                    "Unsupported syntax in check constraint `{}.{}`: {}",
                    constraint.constraint_schema, constraint.constraint_name, syntax_error
                )
            }
            Self::OperatorsNotSupported => {
                write!(f, "Some operators are not supported")
            }
            Self::NoInvolvedColumns(column, constraint) => {
                write!(
                    f,
                    "Column `{}.{}` is not involved in the check constraint `{}.{}`",
                    column.table_name,
                    column.column_name,
                    constraint.constraint_schema,
                    constraint.constraint_name
                )
            }
            Self::TopLevelExpressionNotResult(constraint) => {
                write!(
                    f,
                    "Top-level expression in check constraint `{}.{}` cannot be reduced to a Result-returning expression",
                    constraint.constraint_schema, constraint.constraint_name
                )
            }
        }
    }
}

impl core::error::Error for CheckConstraintError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            CheckConstraintError::UnsupportedSyntax(_, syntax_error) => Some(syntax_error),
            _ => None,
        }
    }
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

impl Display for UnsupportedCheckConstraintErrorSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnsupportedCheckConstraintErrorSyntax::ExpectedNoScopedColumn(count) => {
                write!(f, "Expected no scoped columns, but found {count}")
            }
            UnsupportedCheckConstraintErrorSyntax::ExpectedSingleScopedColumn(count) => {
                write!(f, "Expected a single scoped column, but found {count}")
            }
            UnsupportedCheckConstraintErrorSyntax::ExpectedScopedColumn => {
                write!(f, "Expected a scoped column")
            }
        }
    }
}

impl core::error::Error for UnsupportedCheckConstraintErrorSyntax {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        None
    }
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

impl From<DiskCacheError> for WebCodeGenError {
    fn from(value: DiskCacheError) -> Self {
        WebCodeGenError::DiskCache(value)
    }
}
