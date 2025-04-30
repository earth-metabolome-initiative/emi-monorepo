//! Submodule implementing the `ExecuteCrudOperation` trait for the
//! `CrudTableOperation` enum.

use diesel::Connection;

use super::CrudTableOperation;
use crate::{
    crud::ExecuteCrudOperation,
    database::BoundedReadDispatch,
    prelude::{Rows, Tabular},
};

impl<C, R> ExecuteCrudOperation<C> for CrudTableOperation<R>
where
    C: Connection,
    R: Rows + BoundedReadDispatch<C, TableName = <R as Tabular>::TableName>,
{
    fn execute(self, conn: &mut C) -> Result<Self::Payload, diesel::result::Error> {
        match self {
            CrudTableOperation::Read { table_name, offset, limit } => {
                R::bounded_read(table_name, offset, limit, conn)
            }
        }
    }
}
