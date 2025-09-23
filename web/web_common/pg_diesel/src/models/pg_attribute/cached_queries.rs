//! Submodule defining the cached queries methods used in the [`PgAttribute`] struct.

use crate::models::{PgAttribute, PgType};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

#[pg_cached::auto_cached]
pub(super) fn pg_type(
    pg_attribute: &PgAttribute,
    conn: &mut PgConnection,
) -> Result<PgType, diesel::result::Error> {
    use crate::schema::pg_type;
    Ok(pg_type::table.filter(pg_type::oid.eq(pg_attribute.atttypid)).first(conn)?)
}
