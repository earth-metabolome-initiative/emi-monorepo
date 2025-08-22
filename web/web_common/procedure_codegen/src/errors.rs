//! Submodule defining the errors which may occur during procedure code
//! generation.

#[derive(Debug)]
/// Errors which may occur during procedure code generation.
pub enum Error {
    /// An error occurred while interacting with Diesel.
    Diesel(diesel::result::Error),
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::Diesel(err)
    }
}
