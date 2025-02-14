#![doc = include_str!("../README.md")]

pub mod authenticated_operation;
pub mod operation;
pub mod operation_error;
pub mod outcome;
pub mod session;
pub mod connection;
pub mod tabular;

/// Main prelude for the library.
pub mod prelude {
    pub use crate::authenticated_operation::AuthenticatedOperation;
    pub use crate::operation::Operation;
    pub use crate::operation_error::OperationError;
    pub use crate::outcome::Outcome;
    pub use crate::session::Session;
    pub use crate::connection::Connection;
    pub use crate::tabular::Tabular;
}
