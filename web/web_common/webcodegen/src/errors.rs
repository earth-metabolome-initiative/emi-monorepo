//! Enumeration for the errors that may happen within the webcodegen crate.
use crate::{custom_schema_constraints::ConstraintError, Column, Table, PgType};
use diesel::result::Error as DieselError;

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
    UnexpectedCompositePrimaryKey(Table)
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
