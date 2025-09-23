//! Submodule defining the cached queries methods used in the [`CheckConstraint`] struct.

use crate::models::{CheckConstraint, Column, PgConstraint, TableConstraint};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};

#[pg_cached::auto_cached]
pub(super) fn columns(
    check_constraint: &CheckConstraint,
    conn: &mut PgConnection,
) -> Result<Vec<Column>, diesel::result::Error> {
    use diesel::RunQueryDsl;

    use crate::schema::{columns, constraint_column_usage};
    Ok(columns::table
        .inner_join(
            constraint_column_usage::table.on(constraint_column_usage::constraint_name
                .eq(&check_constraint.constraint_name)
                .and(
                    constraint_column_usage::constraint_catalog
                        .eq(&check_constraint.constraint_catalog)
                        .and(
                            constraint_column_usage::constraint_schema
                                .eq(&check_constraint.constraint_schema),
                        ),
                )),
        )
        .filter(
            constraint_column_usage::column_name.eq(&columns::column_name).and(
                constraint_column_usage::table_catalog.eq(&columns::table_catalog).and(
                    constraint_column_usage::table_schema
                        .eq(&columns::table_schema)
                        .and(constraint_column_usage::table_name.eq(&columns::table_name)),
                ),
            ),
        )
        .select(Column::as_select())
        .load(conn)?)
}

#[pg_cached::auto_cached]
pub fn table_constraint(
    check_constraint: &CheckConstraint,
    conn: &mut PgConnection,
) -> Result<TableConstraint, diesel::result::Error> {
    use crate::schema::table_constraints;

    Ok(table_constraints::table
        .filter(
            table_constraints::constraint_name
                .eq(&check_constraint.constraint_name)
                .and(table_constraints::constraint_catalog.eq(&check_constraint.constraint_catalog))
                .and(table_constraints::constraint_schema.eq(&check_constraint.constraint_schema)),
        )
        .select(TableConstraint::as_select())
        .first(conn)?)
}

#[pg_cached::auto_cached]
pub fn pg_constraint(
    check_constraint: &CheckConstraint,
    conn: &mut PgConnection,
) -> Result<PgConstraint, diesel::result::Error> {
    use crate::schema::{pg_constraint, pg_namespace};
    Ok(pg_constraint::table
        .inner_join(pg_namespace::table.on(pg_constraint::connamespace.eq(pg_namespace::oid)))
        .filter(
            pg_constraint::conname
                .eq(&check_constraint.constraint_name)
                .and(pg_constraint::contype.eq("c")),
        )
        .filter(pg_namespace::nspname.eq(&check_constraint.constraint_schema))
        .select(PgConstraint::as_select())
        .first(conn)?)
}
