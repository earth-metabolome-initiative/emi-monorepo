//! Crud operations which have a Primary Key as a payload.

mod r#async;
mod sync;

use super::{CRUD, CrudOperation};
use crate::prelude::{Row, Tabular};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
/// Struct representing a CRUD operation with a Primary Key as a payload.
pub enum CrudPrimaryKeyOperation<R: Row> {
    /// A Read operation with a payload of type `R::PrimaryKey`.
    Read(R::PrimaryKey),
    /// A delete operation with a payload of type `R::PrimaryKey`.
    Delete(R::PrimaryKey),
}

impl<R: Row> AsRef<CRUD> for CrudPrimaryKeyOperation<R> {
    fn as_ref(&self) -> &CRUD {
        match self {
            CrudPrimaryKeyOperation::Read(_) => &CRUD::Read,
            CrudPrimaryKeyOperation::Delete(_) => &CRUD::Delete,
        }
    }
}

impl<R: Row> CrudOperation for CrudPrimaryKeyOperation<R> {
    type Payload = Option<R>;
}

impl<R: Row> Tabular for CrudPrimaryKeyOperation<R> {
    type TableName = R::TableName;

    fn table_name(&self) -> Self::TableName {
        match self {
            CrudPrimaryKeyOperation::Read(pk) | CrudPrimaryKeyOperation::Delete(pk) => {
                pk.table_name()
            }
        }
    }
}

impl<R: Row> Row for CrudPrimaryKeyOperation<R> {
    type PrimaryKey = R::PrimaryKey;

    fn primary_key(&self) -> Self::PrimaryKey {
        match self {
            CrudPrimaryKeyOperation::Read(pk) | CrudPrimaryKeyOperation::Delete(pk) => *pk,
        }
    }
}
