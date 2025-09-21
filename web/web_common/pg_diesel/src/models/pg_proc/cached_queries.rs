//! Submodule defining the cached queries methods used in the [`PgProc`] struct.

use crate::models::{PgExtension, PgProc};
use diesel::{ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

#[pg_cached::oid_auto_cached]
pub(super) fn extension(
    pg_proc: &PgProc,
    conn: &mut PgConnection,
) -> Result<PgExtension, crate::error::Error> {
    use crate::schema::{pg_depend, pg_extension};
    Ok(pg_extension::table
        .inner_join(pg_depend::table.on(pg_extension::oid.eq(pg_depend::refobjid)))
        .filter(pg_depend::objid.eq(pg_proc.oid))
        .select(PgExtension::as_select())
        .first::<PgExtension>(conn)?)
}
