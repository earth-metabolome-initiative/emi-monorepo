#![doc = include_str!("../README.md")]

mod from_impls;

#[derive(Debug)]
/// High-level errors that may occur in the Server.
pub enum Error {
    /// An error that occurred while trying to access the redis database.
    RedisError(redis::RedisError),
    /// An error that occurred while trying to access the postgres database with diesel.
    DieselError,
    /// An error that occurred due to some error mis-configuration.
    EnvironmentError,
    /// The user attempted an unauthorized action.
    Unauthorized,
    /// A third-party service returned an error.
    ThirdPartyError,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::RedisError => {
                write!(f, "An error occurred while trying to access the redis database.")
            }
            Error::DieselError => {
                write!(f, "An error occurred while trying to access the database.")
            }
            Error::EnvironmentError => {
                write!(f, "An error occurred due to some error mis-configuration.")
            }
            Error::Unauthorized => write!(f, "The user attempted an unauthorized action."),
            Error::ThirdPartyError => {
                write!(f, "A third-party service returned an error.")
            }
        }
    }
}
