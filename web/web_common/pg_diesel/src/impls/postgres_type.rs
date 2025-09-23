//! Submodule providing implementations of the [`PostgresType`] trait for various
//! diesel model structs representing PostgreSQL system catalog tables.

use crate::{
    models::{Column, PgType},
    traits::PostgresType,
};

impl PostgresType for PgType {
    fn postgres_type(
        &self,
        _conn: &mut diesel::PgConnection,
    ) -> Result<PgType, diesel::result::Error> {
        Ok(self.clone())
    }
}

impl PostgresType for Column {
    fn postgres_type(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<PgType, diesel::result::Error> {
        self.pg_type(conn)
    }
}
