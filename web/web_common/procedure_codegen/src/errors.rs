//! Submodule defining the errors which may occur during procedure code
//! generation.

mod procedure_error;
mod procedure_model_error;

pub use procedure_error::ProcedureError;
pub use procedure_model_error::ProcedureModelError;

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
    /// A procedure model-related error occurred.
    ProcedureModel(ProcedureModelError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Diesel(err) => write!(f, "Diesel error: {}", err),
            Error::Syn(err) => write!(f, "Syn error: {}", err),
            Error::Io(err) => write!(f, "I/O error: {}", err),
            Error::Webcodegen(err) => write!(f, "Webcodegen error: {}", err),
            Error::Procedure(err) => write!(f, "Procedure error: {}", err),
            Error::ProcedureModel(err) => write!(f, "Procedure model error: {}", err),
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
            Error::ProcedureModel(err) => Some(err),
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

impl From<procedure_error::ProcedureError> for Error {
    fn from(err: procedure_error::ProcedureError) -> Self {
        Error::Procedure(err)
    }
}

impl From<procedure_model_error::ProcedureModelError> for Error {
    fn from(err: procedure_model_error::ProcedureModelError) -> Self {
        Error::ProcedureModel(err)
    }
}
