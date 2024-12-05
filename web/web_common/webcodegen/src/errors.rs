//! Enumeration for the errors that may happen within the webcodegen crate.
use diesel::result::Error as DieselError;
use crate::custom_schema_constraints::ConstraintError;

#[derive(Debug)]
pub enum WebCodeGenError {
    DieselError(DieselError),
    MissingTable(String),
    IllegalTable(String),
    IllegalRolesTable(String),
    ConstraintError(ConstraintError),

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