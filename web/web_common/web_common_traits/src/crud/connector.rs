//! Submodule providing the `Connector` trait which defines an interface able to
//! connect the frontend to either the local database or the remote server.

use super::{primary_key::CrudPrimaryKeyOperation, table::CrudTableOperation};
use crate::prelude::{Row, Rows};

/// The `Connector` trait defines an interface for connecting to a data source.
pub trait Connector {
    /// The associated type representing the row type.
    type Row: Row;
    /// The associated type representing the rows type.
    type Rows: Rows;
    /// The associated type representing the message type.
    type ComponentMessage: From<CrudTableOperation<Self::Rows>>
        + From<CrudPrimaryKeyOperation<Self::Row>>;

    /// Sends a message to the connector.
    fn send<M>(&self, message: M)
    where
        M: Into<Self::ComponentMessage>;
}
