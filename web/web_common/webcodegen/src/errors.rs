//! Enumeration for the errors that may happen within the webcodegen crate.
use crate::{custom_schema_constraints::ConstraintError, Column, PgType, Table};
use diesel::result::Error as DieselError;
use snake_case_sanitizer::SanitizationErrors;
use crate::codegen::CodeGenerationError;

#[derive(Debug)]
pub enum WebCodeGenError {
    DieselError(DieselError),
    MissingTable(String),
    IllegalTable(String),
    IllegalRolesTable(String),
    ConstraintError(ConstraintError),
    UnknownColumnType(Column),
    NotUserDefinedType(String),
    MissingBaseType(PgType),
    SanitizationErrors(SanitizationErrors),
    CodeGenerationError(CodeGenerationError),
    IllegalTableCodegen(String, String, Table),
    ExcessiveNumberOfColumns(Table, usize),
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