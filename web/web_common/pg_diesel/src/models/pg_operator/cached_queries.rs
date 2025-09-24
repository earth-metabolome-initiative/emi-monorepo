//! Submodule defining the cached queries methods used in the [`PgOperator`]
//! struct.

use diesel::{ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::models::{PgExtension, PgOperator, PgProc, PgType};

#[pg_cached::oid_auto_cached]
pub(super) fn function(
    pg_operator: &PgOperator,
    conn: &mut PgConnection,
) -> Result<PgProc, diesel::result::Error> {
    use crate::schema::pg_proc;
    Ok(pg_proc::table
        .filter(pg_proc::oid.eq(pg_operator.oprcode))
        .select(PgProc::as_select())
        .first::<PgProc>(conn)?)
}

#[pg_cached::oid_auto_cached]
pub(super) fn extension(
    pg_operator: &PgOperator,
    conn: &mut PgConnection,
) -> Result<PgExtension, diesel::result::Error> {
    use crate::schema::{pg_depend, pg_extension};
    Ok(pg_extension::table
        .inner_join(pg_depend::table.on(pg_extension::oid.eq(pg_depend::refobjid)))
        .filter(pg_depend::objid.eq(pg_operator.oid))
        .select(PgExtension::as_select())
        .first::<PgExtension>(conn)?)
}

#[pg_cached::oid_auto_cached]
pub(super) fn left_operand_type(
    pg_operator: &PgOperator,
    conn: &mut PgConnection,
) -> Result<PgType, diesel::result::Error> {
    use crate::schema::pg_type;
    Ok(pg_type::table
        .filter(pg_type::oid.eq(pg_operator.oprleft))
        .select(PgType::as_select())
        .first::<PgType>(conn)?)
}

#[pg_cached::oid_auto_cached]
pub(super) fn right_operand_type(
    pg_operator: &PgOperator,
    conn: &mut PgConnection,
) -> Result<PgType, diesel::result::Error> {
    use crate::schema::pg_type;
    Ok(pg_type::table
        .filter(pg_type::oid.eq(pg_operator.oprright))
        .select(PgType::as_select())
        .first::<PgType>(conn)?)
}

#[pg_cached::oid_auto_cached]
pub(super) fn result_type(
    pg_operator: &PgOperator,
    conn: &mut PgConnection,
) -> Result<PgType, diesel::result::Error> {
    use crate::schema::pg_type;
    Ok(pg_type::table
        .filter(pg_type::oid.eq(pg_operator.oprresult))
        .select(PgType::as_select())
        .first::<PgType>(conn)?)
}
