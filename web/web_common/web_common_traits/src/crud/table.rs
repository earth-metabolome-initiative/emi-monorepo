//! Crud operations which have a Tabular entry as a payload.

mod sync;

use super::{CRUD, CrudOperation};
use crate::prelude::{Rows, Tabular};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
/// Struct representing a CRUD operation with a Tabular entry as a payload.
pub enum CrudTableOperation<R: Rows> {
    /// A Read operation with a payload of type `P`.
    Read {
        /// The table name for the read operation.
        table_name: R::TableName,
        /// The offset from which to start reading.
        offset: u16,
        /// The limit on the number of rows to read.
        limit: u16,
    },
}

impl<R: Rows> AsRef<CRUD> for CrudTableOperation<R> {
    fn as_ref(&self) -> &CRUD {
        match self {
            CrudTableOperation::Read { .. } => &CRUD::Read,
        }
    }
}

impl<R: Rows> CrudOperation for CrudTableOperation<R> {
    type Payload = R;
}

impl<R> Tabular for CrudTableOperation<R>
where
    R: Rows,
{
    type TableName = R::TableName;

    fn table_name(&self) -> Self::TableName {
        match self {
            Self::Read { table_name, .. } => *table_name,
        }
    }
}
