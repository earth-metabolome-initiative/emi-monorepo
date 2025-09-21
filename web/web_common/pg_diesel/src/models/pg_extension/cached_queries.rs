//! Submodule defining the cached queries methods used in the [`PgExtension`] struct.

use diesel::{ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::models::{PgEnum, PgExtension, PgProc, PgType};

#[pg_cached::oid_auto_cached]
pub(super) fn enums(
    pg_extension: &PgExtension,
    conn: &mut PgConnection,
) -> Result<Vec<PgEnum>, crate::error::Error> {
    use crate::schema::{pg_depend, pg_enum, pg_type};

    Ok(pg_enum::table
        .inner_join(pg_type::table.on(pg_enum::enumtypid.eq(pg_type::oid)))
        .inner_join(pg_depend::table.on(pg_depend::objid.eq(pg_type::oid)))
        .filter(pg_depend::refobjid.eq(pg_extension.oid))
        .select(PgEnum::as_select())
        .load(conn)?)
}

#[pg_cached::oid_auto_cached]
pub(super) fn types(
    pg_extension: &PgExtension,
    conn: &mut PgConnection,
) -> Result<Vec<PgType>, crate::error::Error> {
    use crate::schema::{pg_depend, pg_type};
    Ok(pg_depend::table
        .inner_join(pg_type::table.on(pg_depend::objid.eq(pg_type::oid)))
        .filter(pg_depend::refobjid.eq(pg_extension.oid))
        .select(PgType::as_select())
        .load(conn)?)
}

#[pg_cached::oid_auto_cached]
pub(super) fn functions(
    pg_extension: &PgExtension,
    conn: &mut PgConnection,
) -> Result<Vec<PgProc>, crate::error::Error> {
    use crate::schema::{pg_depend, pg_proc};
    Ok(pg_depend::table
        .inner_join(pg_proc::table.on(pg_depend::objid.eq(pg_proc::oid)))
        .filter(pg_depend::refobjid.eq(pg_extension.oid))
        .select(PgProc::as_select())
        .load(conn)?)
}
