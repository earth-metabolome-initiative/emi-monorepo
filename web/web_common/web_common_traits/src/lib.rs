#![doc = include_str!("../README.md")]

pub mod attributes;
pub mod connection;
pub mod database;
pub mod filtrable;
pub mod insert_operation;
pub mod operation;
pub mod operation_error;
pub mod outcome;
pub mod session;
pub mod session_operation;
pub mod types;

/// Main prelude for the library.
pub mod prelude {
    pub use common_traits::prelude::*;

    #[cfg(feature = "backend")]
    pub use crate::types::{DBConn, DBPool};
    pub use crate::{
        attributes::*,
        connection::Connection,
        database::{Deletable, Loadable},
        filtrable::*,
        insert_operation::{InsertableRow, InsertableVariant, InsertableVariantBuilder},
        operation::Operation,
        operation_error::OperationError,
        outcome::Outcome,
        session::Session,
        session_operation::SessionOperation,
    };
}
