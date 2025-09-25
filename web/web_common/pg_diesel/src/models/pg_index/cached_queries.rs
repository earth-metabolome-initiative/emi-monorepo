//! Submodule defining the cached queries methods used in the [`PgIndex`]
//! struct.


use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};

use crate::models::{Column, PgIndex, Table};

#[pg_cached::oid_auto_cached]
pub(super) fn columns(
    index: &PgIndex,
    conn: &mut PgConnection,
) -> Result<Vec<Column>, diesel::result::Error> {
    use crate::schema::{columns, pg_attribute, pg_class, pg_index};

    Ok(
        pg_index::table
            .inner_join(pg_class::table.on(pg_class::oid.eq(pg_index::indrelid)))
            .inner_join(pg_attribute::table.on(pg_attribute::attrelid.eq(pg_class::oid)))
            .inner_join(
                columns::table.on(columns::table_name
                    .eq(pg_class::relname)
                    .and(columns::column_name.eq(pg_attribute::attname))),
            )
            .filter(
                pg_index::indexrelid
                    .eq(index.indexrelid)
                    .and(pg_attribute::attnum.eq_any(&index.indkey)),
            )
            .select(Column::as_select())
            .load::<Column>(conn)?,
    )
}

#[pg_cached::oid_auto_cached]
pub(super) fn table(
    index: &PgIndex,
    conn: &mut PgConnection,
) -> Result<crate::models::Table, diesel::result::Error> {
    use crate::schema::{pg_class, tables};

    Ok(pg_class::table
        .inner_join(tables::table.on(tables::table_name.eq(pg_class::relname)))
        .filter(pg_class::oid.eq(index.indrelid))
        .select(Table::as_select())
        .first(conn)?)
}
