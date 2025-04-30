#![doc = include_str!("../README.md")]

pub mod attributes;
pub mod connection;
pub mod crud;
pub mod database;
pub mod filtrable;
pub mod insert_operation;
pub mod operation;
pub mod operation_error;
pub mod outcome;
pub mod session;
pub mod session_operation;

/// Main prelude for the library.
pub mod prelude {
    pub use common_traits::prelude::*;

    #[cfg(feature = "backend")]
    pub use crate::database::BackendInsertableVariant;
    #[cfg(feature = "diesel-async")]
    pub use crate::database::{
        AsyncBoundedRead, AsyncBoundedReadDispatch, AsyncRead, AsyncReadDispatch,
    };
    pub use crate::{
        attributes::*,
        connection::Connection,
        database::{
            BoundedRead, BoundedReadDispatch, Deletable, Foreign, ForeignKeys, HasForeignKeys,
            Insertable, InsertableBuilder, InsertableVariant, ReadDispatch, Row, Rows,
            StaticTabular, Tabular, UpsertVec, Upsertable,
        },
        filtrable::*,
        operation::Operation,
        operation_error::OperationError,
        outcome::Outcome,
        session::Session,
        session_operation::SessionOperation,
    };
}
