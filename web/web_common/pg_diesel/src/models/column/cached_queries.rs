//! Submodule defining the cached queries methods used in the [`Column`] struct.

use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, OptionalExtension, PgConnection, QueryDsl,
    RunQueryDsl, SelectableHelper,
};

use crate::models::{
    CheckConstraint, Column, GeographyColumn, GeometryColumn, KeyColumnUsage, PgType, Table,
};

#[pg_cached::auto_cached]
pub(super) fn foreign_keys(
    column: &Column,
    conn: &mut PgConnection,
) -> Result<Vec<KeyColumnUsage>, diesel::result::Error> {
    use crate::schema::{
        columns, constraint_column_usage, key_column_usage, table_constraints, tables,
    };
    Ok(
        table_constraints::table
            .inner_join(
                key_column_usage::table.on(table_constraints::constraint_name
                    .eq(key_column_usage::constraint_name)
                    .and(
                        table_constraints::constraint_schema
                            .eq(key_column_usage::constraint_schema),
                    )
                    .and(
                        table_constraints::constraint_catalog
                            .eq(key_column_usage::constraint_catalog),
                    )
                    .and(table_constraints::table_name.eq(key_column_usage::table_name))
                    .and(table_constraints::table_schema.eq(key_column_usage::table_schema))
                    .and(table_constraints::table_catalog.eq(key_column_usage::table_catalog))),
            )
            .inner_join(constraint_column_usage::table.on(
                constraint_column_usage::constraint_name.eq(table_constraints::constraint_name),
            ))
            .inner_join(
                tables::table.on(tables::table_name
                    .eq(constraint_column_usage::table_name)
                    .and(tables::table_schema.eq(constraint_column_usage::table_schema))
                    .and(tables::table_catalog.eq(constraint_column_usage::table_catalog))),
            )
            .inner_join(
                columns::table.on(columns::table_name
                    .eq(constraint_column_usage::table_name)
                    .and(columns::table_schema.eq(constraint_column_usage::table_schema))
                    .and(columns::table_catalog.eq(constraint_column_usage::table_catalog))
                    .and(columns::column_name.eq(constraint_column_usage::column_name))),
            )
            .filter(table_constraints::constraint_type.eq("FOREIGN KEY"))
            .filter(table_constraints::table_name.eq(&column.table_name))
            .filter(table_constraints::table_schema.eq(&column.table_schema))
            .filter(table_constraints::table_catalog.eq(&column.table_catalog))
            .filter(key_column_usage::column_name.eq(&column.column_name))
            .order_by(key_column_usage::constraint_name.asc())
            .select(KeyColumnUsage::as_select())
            .distinct()
            .load::<KeyColumnUsage>(conn)?,
    )
}

#[pg_cached::auto_cached]
pub(super) fn check_constraints(
    column: &Column,
    conn: &mut PgConnection,
) -> Result<Vec<CheckConstraint>, diesel::result::Error> {
    use crate::schema::{check_constraints, constraint_column_usage};
    Ok(check_constraints::table
        .inner_join(
            constraint_column_usage::table.on(constraint_column_usage::constraint_name
                .eq(check_constraints::constraint_name)
                .and(
                    constraint_column_usage::constraint_catalog
                        .eq(check_constraints::constraint_catalog)
                        .and(
                            constraint_column_usage::constraint_schema
                                .eq(check_constraints::constraint_schema),
                        ),
                )),
        )
        .filter(
            constraint_column_usage::column_name.eq(&column.column_name).and(
                constraint_column_usage::table_catalog.eq(&column.table_catalog).and(
                    constraint_column_usage::table_schema
                        .eq(&column.table_schema)
                        .and(constraint_column_usage::table_name.eq(&column.table_name)),
                ),
            ),
        )
        .select(CheckConstraint::as_select())
        .load(conn)?)
}

#[pg_cached::auto_cached]
pub(super) fn table(
    column: &Column,
    conn: &mut PgConnection,
) -> Result<Table, diesel::result::Error> {
    use crate::schema::tables;
    Ok(tables::table
        .filter(tables::table_name.eq(&column.table_name))
        .filter(tables::table_schema.eq(&column.table_schema))
        .filter(tables::table_catalog.eq(&column.table_catalog))
        .first::<Table>(conn)?)
}

#[pg_cached::auto_cached]
pub(super) fn geometry(
    column: &Column,
    conn: &mut PgConnection,
) -> Result<Option<GeometryColumn>, diesel::result::Error> {
    use crate::schema::geometry_columns;

    Ok(geometry_columns::table
        .filter(geometry_columns::f_table_name.eq(&column.table_name))
        .filter(geometry_columns::f_table_schema.eq(&column.table_schema))
        .filter(geometry_columns::f_geometry_column.eq(&column.column_name))
        .first::<GeometryColumn>(conn)
        .optional()?)
}

#[pg_cached::auto_cached]
pub(super) fn geography(
    column: &Column,
    conn: &mut PgConnection,
) -> Result<Option<GeographyColumn>, diesel::result::Error> {
    use crate::schema::geography_columns;

    Ok(geography_columns::table
        .filter(geography_columns::f_table_name.eq(&column.table_name))
        .filter(geography_columns::f_table_schema.eq(&column.table_schema))
        .filter(geography_columns::f_geography_column.eq(&column.column_name))
        .first::<GeographyColumn>(conn)
        .optional()?)
}

#[pg_cached::auto_cached]
pub(super) fn pg_type(
    column: &Column,
    conn: &mut PgConnection,
) -> Result<PgType, diesel::result::Error> {
    use crate::schema::{pg_attribute, pg_class, pg_namespace, pg_type};

    Ok(pg_type::table
        .inner_join(pg_attribute::table.on(pg_attribute::atttypid.eq(pg_type::oid)))
        .inner_join(pg_class::table.on(pg_attribute::attrelid.eq(pg_class::oid)))
        .inner_join(pg_namespace::table.on(pg_class::relnamespace.eq(pg_namespace::oid)))
        .filter(pg_class::relname.eq(&column.table_name))
        .filter(pg_namespace::nspname.eq(&column.table_schema))
        .filter(pg_attribute::attname.eq(&column.column_name))
        .select(PgType::as_select())
        .first::<PgType>(conn)?)
}
