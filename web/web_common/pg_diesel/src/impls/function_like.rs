//! Submodule implementing the
//! [`FunctionLike`](sql_traits::prelude::FunctionLike) trait for the
//! [`PgProc`](crate::models::PgProc) struct.

use sql_traits::traits::FunctionLike;

use crate::models::PgProc;

impl FunctionLike for PgProc {
    fn name(&self) -> &str {
        &self.proname
    }
}
