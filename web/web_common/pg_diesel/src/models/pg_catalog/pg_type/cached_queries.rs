//! Submodule defining the cached queries methods used in the [`PgType`] struct.

use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};

use crate::models::{PgAttribute, PgEnum, PgExtension, PgType};

pub(super) fn variants(
    pg_type: &PgType,
    conn: &mut PgConnection,
) -> Result<Vec<PgEnum>, diesel::result::Error> {
    use crate::schema::pg_catalog::pg_enum::pg_enum;

    Ok(pg_enum::table
        .filter(pg_enum::enumtypid.eq(pg_type.oid))
        .order_by(pg_enum::enumsortorder)
        .select(PgEnum::as_select())
        .load::<PgEnum>(conn)?)
}

pub(super) fn attributes(
    pg_type: &PgType,
    conn: &mut PgConnection,
) -> Result<Vec<PgAttribute>, diesel::result::Error> {
    use crate::schema::pg_catalog::{pg_attribute::pg_attribute, pg_type::pg_type};

    Ok(pg_attribute::table
        .inner_join(pg_type::table.on(pg_attribute::attrelid.eq(pg_type::typrelid)))
        .filter(pg_type::typname.eq(&pg_type.typname).and(pg_attribute::attisdropped.eq(false)))
        .select(PgAttribute::as_select())
        .load::<PgAttribute>(conn)?)
}

pub(super) fn extension(
    pg_type: &PgType,
    conn: &mut PgConnection,
) -> Result<PgExtension, diesel::result::Error> {
    use crate::schema::pg_catalog::{pg_depend::pg_depend, pg_extension::pg_extension};
    Ok(pg_depend::table
        .filter(pg_depend::objid.eq(pg_type.oid))
        .filter(pg_depend::deptype.eq("e"))
        .inner_join(pg_extension::table.on(pg_extension::oid.eq(pg_depend::refobjid)))
        .select(PgExtension::as_select())
        .first::<PgExtension>(conn)?)
}

pub(super) fn from_oid(oid: u32, conn: &mut PgConnection) -> Result<PgType, diesel::result::Error> {
    use crate::schema::pg_catalog::pg_type::pg_type;
    Ok(pg_type::table.filter(pg_type::oid.eq(oid)).first::<PgType>(conn)?)
}
