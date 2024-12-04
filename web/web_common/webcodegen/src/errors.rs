//! Enumeration for the errors that may happen within the webcodegen crate.
use diesel::result::Error as DieselError;

#[derive(Debug)]
pub enum WebCodeGenError {
    DieselError(DieselError),
    MissingTable(String),
    IllegalTable(String),
    IllegalRolesTable(String),
}

impl From<DieselError> for WebCodeGenError {
    fn from(e: DieselError) -> Self {
        WebCodeGenError::DieselError(e)
    }
}
