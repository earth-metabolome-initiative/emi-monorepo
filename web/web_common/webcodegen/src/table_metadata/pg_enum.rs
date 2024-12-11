//! Submodule providing the `PgEnum` struct, which represents a PostgreSQL enum type.
use diesel::{Queryable, QueryableByName, Selectable};

#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::pg_enum)]
pub struct PgEnum {
    pub oid: u32,
    pub enumtypid: u32,
    pub enumsortorder: f32,
    pub enumlabel: String,
}
