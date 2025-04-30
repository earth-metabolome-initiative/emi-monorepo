#![cfg(feature = "diesel-async")]
//! Submodule implementing the `AsyncExecuteCrudOperation` trait for the
//! `CrudPrimaryKeyOperation` enum.

use diesel_async::AsyncConnection;

use super::CrudPrimaryKeyOperation;
use crate::{crud::AsyncExecuteCrudOperation, database::AsyncReadDispatch, prelude::Row};

impl<C, R> AsyncExecuteCrudOperation<C> for CrudPrimaryKeyOperation<R>
where
    C: AsyncConnection,
    R: Row + AsyncReadDispatch<C, PrimaryKey = <R as Row>::PrimaryKey>,
{
    async fn execute(self, conn: &mut C) -> Result<Self::Payload, diesel::result::Error> {
        match self {
            CrudPrimaryKeyOperation::Read(primary_key) => R::read(primary_key, conn).await,
            CrudPrimaryKeyOperation::Delete(_) => {
                todo!()
            }
        }
    }
}
