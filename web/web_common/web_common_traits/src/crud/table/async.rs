#![cfg(feature = "diesel-async")]
//! Submodule implementing the `AsyncExecuteCrudOperation` trait for the
//! `CrudTableOperation` enum.

use diesel_async::AsyncConnection;

use super::CrudTableOperation;
use crate::{
    crud::AsyncExecuteCrudOperation,
    database::AsyncBoundedReadDispatch,
    prelude::{Rows, Tabular},
};

impl<C, R> AsyncExecuteCrudOperation<C> for CrudTableOperation<R>
where
    C: AsyncConnection,
    R: Rows + AsyncBoundedReadDispatch<C, TableName = <R as Tabular>::TableName>,
{
    async fn execute(self, conn: &mut C) -> Result<Self::Payload, diesel::result::Error> {
        match self {
            CrudTableOperation::Read { table_name, offset, limit } => {
                R::bounded_read(table_name, offset, limit, conn).await
            }
        }
    }
}
