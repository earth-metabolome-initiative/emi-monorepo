//! Submodule implementing the `ExecuteCrudOperation` trait for the
//! `CrudPrimaryKeyOperation` enum.

use diesel::Connection;

use super::CrudPrimaryKeyOperation;
use crate::{crud::ExecuteCrudOperation, database::ReadDispatch, prelude::Row};

impl<C, R> ExecuteCrudOperation<C> for CrudPrimaryKeyOperation<R>
where
    C: Connection,
    R: Row + ReadDispatch<C, PrimaryKey = <R as Row>::PrimaryKey>,
{
    fn execute(self, conn: &mut C) -> Result<Self::Payload, diesel::result::Error> {
        match self {
            CrudPrimaryKeyOperation::Read(primary_key) => R::read(primary_key, conn),
            CrudPrimaryKeyOperation::Delete(_) => {
                todo!()
            }
        }
    }
}
