//! Submodule defining the cached queries methods used in the [`PgProc`] struct.

use diesel::{ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::models::{PgExtension, PgProc};

#[pg_cached::oid_auto_cached]
pub(super) fn extension(
    pg_proc: &PgProc,
    conn: &mut PgConnection,
) -> Result<PgExtension, diesel::result::Error> {
    use crate::schema::pg_catalog::{pg_depend::pg_depend, pg_extension::pg_extension};
    Ok(pg_extension::table
        .inner_join(pg_depend::table.on(pg_extension::oid.eq(pg_depend::refobjid)))
        .filter(pg_depend::objid.eq(pg_proc.oid))
        .select(PgExtension::as_select())
        .first::<PgExtension>(conn)?)
}

pub(super) fn load_all(conn: &mut PgConnection) -> Result<Vec<PgProc>, diesel::result::Error> {
    use crate::schema::pg_catalog::pg_proc::pg_proc;
    Ok(pg_proc::table
        .filter(pg_proc::prokind.ne_all(vec!["p", "a"])) // Exclude procedures and aggregates
        .filter(pg_proc::proisstrict.eq(true)) // Exclude non-strict functions
        .filter(pg_proc::proretset.eq(false)) // Exclude set-returning functions
        .filter(pg_proc::prorettype.ne(0)) // Exclude functions returning "void"
        .order_by(pg_proc::proname.asc())
        .then_order_by(pg_proc::oid.asc())
        .select(PgProc::as_select())
        .load::<PgProc>(conn)?)
}
