//! Submodule defining the errors which may occur during procedure code
//! generation.

mod procedure_error;
mod procedure_template_error;

pub use procedure_error::ProcedureError;
pub use procedure_template_error::ProcedureTemplateError;
use webcodegen::{Column, Table};

use crate::utils::ASSETS_TABLE_NAME;

#[derive(Debug)]
/// Errors which may occur during procedure code generation.
pub enum Error {
    /// An error occurred while interacting with Diesel.
    Diesel(diesel::result::Error),
    /// An error occurred while parsing or formatting code.
    Syn(syn::Error),
    /// An I/O error occurred.
    Io(std::io::Error),
    /// A webcodegen error occurred.
    Webcodegen(webcodegen::errors::WebCodeGenError),
    /// A procedure-related error occurred.
    Procedure(ProcedureError),
    /// A procedure template-related error occurred.
    ProcedureTemplate(ProcedureTemplateError),
    /// An asset column was not characterized in a procedure table.
    UncharacterizedAssetColumn(Box<Column>),
    /// An asset foreign key in a procedure or procedure template table was
    /// cascading.
    CascadingAssetForeignKey(Box<Column>),
    /// An unused foreign procedure template constraint was found.
    UnusedForeignProcedureTemplateConstraint {
        /// The procedure template table.
        table: Box<Table>,
        /// The columns which are part of the unused foreign procedure template
        /// constraint.
        columns: Vec<Column>,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Diesel(err) => write!(f, "Diesel error: {}", err),
            Error::Syn(err) => write!(f, "Syn error: {}", err),
            Error::Io(err) => write!(f, "I/O error: {}", err),
            Error::Webcodegen(err) => write!(f, "Webcodegen error: {}", err),
            Error::Procedure(err) => write!(f, "Procedure error: {}", err),
            Error::ProcedureTemplate(err) => write!(f, "procedure template error: {}", err),
            Error::UncharacterizedAssetColumn(column) => {
                write!(
                    f,
                    "Uncharacterized foreign key to {ASSETS_TABLE_NAME} table: `{}.{}.{}`",
                    column.table_schema, column.table_name, column.column_name
                )
            }
            Error::CascadingAssetForeignKey(column) => {
                write!(
                    f,
                    "Foreign key to `{ASSETS_TABLE_NAME}` table in a procedure or procedure template table should not be cascading: `{}.{}.{}`",
                    column.table_schema, column.table_name, column.column_name
                )
            }
            Error::UnusedForeignProcedureTemplateConstraint { table, columns } => {
                write!(
                    f,
                    "Procedure template table `{}.{}' has same-as foreign keys to a procedure template which are not used in the associated procedure table: Columns: [{}]",
                    table.table_schema,
                    table.table_name,
                    columns.iter().map(|c| c.column_name.as_str()).collect::<Vec<_>>().join(", ")
                )
            }
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Error::Diesel(err) => Some(err),
            Error::Syn(err) => Some(err),
            Error::Io(err) => Some(err),
            Error::Webcodegen(err) => Some(err),
            Error::Procedure(err) => Some(err),
            Error::ProcedureTemplate(err) => Some(err),
            Error::UncharacterizedAssetColumn(_) => None,
            Error::CascadingAssetForeignKey(_) => None,
            Error::UnusedForeignProcedureTemplateConstraint { .. } => None,
        }
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::Diesel(err)
    }
}

impl From<syn::Error> for Error {
    fn from(err: syn::Error) -> Self {
        Error::Syn(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<webcodegen::errors::WebCodeGenError> for Error {
    fn from(err: webcodegen::errors::WebCodeGenError) -> Self {
        Error::Webcodegen(err)
    }
}

impl From<webcodegen::ConstraintError> for Error {
    fn from(err: webcodegen::ConstraintError) -> Self {
        Error::Webcodegen(webcodegen::errors::WebCodeGenError::from(err))
    }
}

impl From<procedure_error::ProcedureError> for Error {
    fn from(err: procedure_error::ProcedureError) -> Self {
        Error::Procedure(err)
    }
}

impl From<procedure_template_error::ProcedureTemplateError> for Error {
    fn from(err: procedure_template_error::ProcedureTemplateError) -> Self {
        Error::ProcedureTemplate(err)
    }
}
