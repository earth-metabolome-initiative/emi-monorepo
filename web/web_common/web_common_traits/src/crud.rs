//! Submodule providing traits for the CRUD operations.

pub mod connector;
use std::fmt::Display;

pub use connector::Connector;
use diesel::Connection;
pub mod primary_key;
pub use primary_key::CrudPrimaryKeyOperation;
pub mod table;
pub use table::CrudTableOperation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
/// Enumeration of the CRUD operations.
pub enum CRUD {
    /// Create operation.
    Create,
    /// Read operation.
    Read,
    /// Update operation.
    Update,
    /// Delete operation.
    Delete,
}

impl Display for CRUD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CRUD::Create => write!(f, "Create"),
            CRUD::Read => write!(f, "Read"),
            CRUD::Update => write!(f, "Update"),
            CRUD::Delete => write!(f, "Delete"),
        }
    }
}

/// Trait representing a CRUD operation.
pub trait CrudOperation: AsRef<CRUD> {
    /// The type of the expected answer payload.
    type Payload;
}

/// Trait representing an executable CRUD operation.
pub trait ExecuteCrudOperation<C: Connection>: CrudOperation {
    /// Executes the operation using the provided connection.
    fn execute(self, conn: &mut C) -> Result<Self::Payload, diesel::result::Error>;
}

#[cfg(feature = "diesel-async")]
/// Trait representing an asynchronously executable CRUD operation.
pub trait AsyncExecuteCrudOperation<C: diesel_async::AsyncConnection>: CrudOperation {
    /// Executes the operation using the provided connection.
    fn execute(
        self,
        conn: &mut C,
    ) -> impl Future<Output = Result<Self::Payload, diesel::result::Error>>;
}
