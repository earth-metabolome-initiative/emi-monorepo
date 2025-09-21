//! Submodule defining the cached queries methods used in the [`PgType`] struct.

use crate::models::{PgAttribute, PgEnum, PgExtension, PgType};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};

#[pg_cached::oid_auto_cached]
pub(super) fn variants(
    pg_type: &PgType,
    conn: &mut PgConnection,
) -> Result<Vec<PgEnum>, crate::error::Error> {
    use crate::schema::pg_enum;

    Ok(pg_enum::table
        .filter(pg_enum::enumtypid.eq(pg_type.oid))
        .order_by(pg_enum::enumsortorder)
        .select(PgEnum::as_select())
        .load::<PgEnum>(conn)?)
}

#[pg_cached::oid_auto_cached]
pub(super) fn attributes(
    pg_type: &PgType,
    conn: &mut PgConnection,
) -> Result<Vec<PgAttribute>, crate::error::Error> {
    use crate::schema::{pg_attribute, pg_type};

    Ok(pg_attribute::table
        .inner_join(pg_type::table.on(pg_attribute::attrelid.eq(pg_type::typrelid)))
        .filter(pg_type::typname.eq(&pg_type.typname).and(pg_attribute::attisdropped.eq(false)))
        .select(PgAttribute::as_select())
        .load::<PgAttribute>(conn)?)
}

#[pg_cached::oid_auto_cached]
pub(super) fn extension(
    pg_type: &PgType,
    conn: &mut PgConnection,
) -> Result<PgExtension, crate::error::Error> {
    use crate::schema::{pg_depend, pg_extension};
    Ok(pg_depend::table
        .filter(pg_depend::objid.eq(pg_type.oid))
        .filter(pg_depend::deptype.eq("e"))
        .inner_join(pg_extension::table.on(pg_extension::oid.eq(pg_depend::refobjid)))
        .select(PgExtension::as_select())
        .first::<PgExtension>(conn)?)
}

#[pg_cached::auto_cached]
pub(super) fn from_oid(
    oid: u32,
    conn: &mut PgConnection,
) -> Result<PgType, crate::error::Error> {
    use crate::schema::pg_type;
    Ok(pg_type::table.filter(pg_type::oid.eq(oid)).first::<PgType>(conn)?)
}
