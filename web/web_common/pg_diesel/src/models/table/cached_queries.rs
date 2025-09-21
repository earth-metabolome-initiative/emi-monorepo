//! Submodule defining the cached queries methods used in the [`Table`] struct.

use crate::models::{CheckConstraint, Column, KeyColumnUsage, PgIndex, PgTrigger, Table};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, PgConnection,
    QueryDsl, RunQueryDsl, SelectableHelper,
};

#[pg_cached::auto_cached]
pub(super) fn load_all_tables(
    table_catalog: &str,
    table_schema: &str,
    conn: &mut PgConnection,
) -> Result<Vec<Table>, crate::error::Error> {
    use crate::schema::tables;
    Ok(
        tables::table
            .filter(tables::table_catalog.eq(table_catalog))
            .filter(tables::table_schema.eq(table_schema))
            .filter(tables::table_name.ne("__diesel_schema_migrations"))
            .order_by(tables::table_name)
            .select(Table::as_select())
            .load::<Table>(conn)?,
    )
}

#[pg_cached::auto_cached]
pub(super) fn load_table(
    conn: &mut PgConnection,
    table_name: &str,
    table_schema: &str,
    table_catalog: &str,
) -> Result<Table, crate::error::Error> {
    use crate::schema::tables;
    Ok(
        tables::table
            .filter(tables::table_name.eq(table_name))
            .filter(tables::table_schema.eq(table_schema))
            .filter(tables::table_catalog.eq(table_catalog))
            .first::<Table>(conn)?,
    )
}

#[pg_cached::auto_cached]
pub(super) fn columns(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Vec<Column>, crate::error::Error> {
    use crate::schema::columns;
    Ok(
        columns::table
            .filter(columns::table_name.eq(&table.table_name))
            .filter(columns::table_schema.eq(&table.table_schema))
            .filter(columns::table_catalog.eq(&table.table_catalog))
            .order_by(columns::ordinal_position)
            .load::<Column>(conn)?,
    )
}

#[pg_cached::auto_cached]
pub(super) fn primary_key_columns(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Vec<Column>, crate::error::Error> {
    use crate::schema::{columns, key_column_usage, table_constraints};
    Ok(
        key_column_usage::table
            .inner_join(
                columns::table.on(key_column_usage::table_name
                    .nullable()
                    .eq(columns::table_name.nullable())
                    .and(
                        key_column_usage::table_schema
                            .nullable()
                            .eq(columns::table_schema.nullable()),
                    )
                    .and(
                        key_column_usage::table_catalog
                            .nullable()
                            .eq(columns::table_catalog.nullable()),
                    )
                    .and(
                        key_column_usage::column_name
                            .nullable()
                            .eq(columns::column_name.nullable()),
                    )),
            )
            .inner_join(
                table_constraints::table.on(key_column_usage::constraint_name
                    .nullable()
                    .eq(table_constraints::constraint_name.nullable())
                    .and(
                        key_column_usage::constraint_schema
                            .nullable()
                            .eq(table_constraints::constraint_schema.nullable()),
                    )
                    .and(
                        key_column_usage::constraint_catalog
                            .nullable()
                            .eq(table_constraints::constraint_catalog.nullable()),
                    )
                    .and(
                        key_column_usage::table_name
                            .nullable()
                            .eq(table_constraints::table_name.nullable()),
                    )
                    .and(
                        key_column_usage::table_schema
                            .nullable()
                            .eq(table_constraints::table_schema.nullable()),
                    )
                    .and(
                        key_column_usage::table_catalog
                            .nullable()
                            .eq(table_constraints::table_catalog.nullable()),
                    )),
            )
            .filter(key_column_usage::table_name.eq(&table.table_name))
            .filter(key_column_usage::table_schema.eq(&table.table_schema))
            .filter(key_column_usage::table_catalog.eq(&table.table_catalog))
            .filter(table_constraints::constraint_type.eq("PRIMARY KEY"))
            .select(Column::as_select())
            .load::<Column>(conn)?,
    )
}

#[pg_cached::auto_cached]
pub(super) fn foreign_keys(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Vec<KeyColumnUsage>, crate::error::Error> {
    use crate::schema::{key_column_usage, referential_constraints};
    Ok(
        key_column_usage::table
            .inner_join(
                referential_constraints::table.on(key_column_usage::constraint_name
                    .eq(referential_constraints::constraint_name)
                    .and(
                        key_column_usage::constraint_schema
                            .eq(referential_constraints::constraint_schema),
                    )
                    .and(
                        key_column_usage::constraint_catalog
                            .eq(referential_constraints::constraint_catalog),
                    )),
            )
            .filter(key_column_usage::table_name.eq(&table.table_name))
            .filter(key_column_usage::table_schema.eq(&table.table_schema))
            .filter(key_column_usage::table_catalog.eq(&table.table_catalog))
            .filter(key_column_usage::ordinal_position.eq(1))
            .order_by((
                key_column_usage::constraint_catalog,
                key_column_usage::constraint_schema,
                key_column_usage::constraint_name,
                key_column_usage::ordinal_position,
            ))
            .select(KeyColumnUsage::as_select())
            .load::<KeyColumnUsage>(conn)?,
    )
}

#[pg_cached::auto_cached]
pub(super) fn unique_indices(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Vec<PgIndex>, crate::error::Error> {
    use crate::schema::{pg_class, pg_index};

    let (pg_class1, pg_class2) = diesel::alias!(pg_class as pg_class1, pg_class as pg_class2);

    Ok(pg_index::table
        .inner_join(pg_class1.on(pg_class1.field(pg_class::oid).eq(pg_index::indexrelid)))
        .inner_join(pg_class2.on(pg_class2.field(pg_class::oid).eq(pg_index::indrelid)))
        .filter(pg_class2.field(pg_class::relname).eq(&table.table_name).and(
            pg_class2.field(pg_class::relnamespace).eq(pg_class1.field(pg_class::relnamespace)),
        ))
        .filter(pg_index::indisunique.eq(true))
        .select(PgIndex::as_select())
        .load::<PgIndex>(conn)?)
}

#[pg_cached::auto_cached]
pub(super) fn indices(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Vec<PgIndex>, crate::error::Error> {
    use crate::schema::{pg_class, pg_index};

    let (pg_class1, pg_class2) = diesel::alias!(pg_class as pg_class1, pg_class as pg_class2);

    Ok(pg_index::table
        .inner_join(pg_class1.on(pg_class1.field(pg_class::oid).eq(pg_index::indexrelid)))
        .inner_join(pg_class2.on(pg_class2.field(pg_class::oid).eq(pg_index::indrelid)))
        .filter(pg_class2.field(pg_class::relname).eq(&table.table_name).and(
            pg_class2.field(pg_class::relnamespace).eq(pg_class1.field(pg_class::relnamespace)),
        ))
        .select(PgIndex::as_select())
        .load::<PgIndex>(conn)?)
}

#[pg_cached::auto_cached]
pub(super) fn triggers(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Vec<PgTrigger>, crate::error::Error> {
    use crate::schema::{pg_class, pg_namespace, pg_trigger};
    Ok(pg_trigger::table
        .inner_join(pg_class::table.on(pg_trigger::tgrelid.eq(pg_class::oid)))
        .inner_join(pg_namespace::table.on(pg_class::relnamespace.eq(pg_namespace::oid)))
        .filter(pg_class::relname.eq(&table.table_name))
        .filter(pg_namespace::nspname.eq(&table.table_schema))
        .select(PgTrigger::as_select())
        .load::<PgTrigger>(conn)?)
}

#[pg_cached::auto_cached]
pub(super) fn check_constraints(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Vec<CheckConstraint>, crate::error::Error> {
    use crate::schema::{check_constraints, table_constraints};

    Ok(check_constraints::table
        .inner_join(
            table_constraints::table.on(check_constraints::constraint_name
                .eq(table_constraints::constraint_name)
                .and(check_constraints::constraint_schema.eq(table_constraints::constraint_schema))
                .and(
                    check_constraints::constraint_catalog.eq(table_constraints::constraint_catalog),
                )),
        )
        .filter(table_constraints::table_name.eq(&table.table_name))
        .filter(table_constraints::table_schema.eq(&table.table_schema))
        .filter(table_constraints::table_catalog.eq(&table.table_catalog))
        .select(CheckConstraint::as_select())
        .load::<CheckConstraint>(conn)?)
}

#[pg_cached::auto_cached]
pub(super) fn column_by_name(
    table: &Table,
    column_name: &str,
    conn: &mut PgConnection,
) -> Result<Column, crate::error::Error> {
    use crate::schema::columns;
    Ok(columns::table
        .filter(columns::table_name.eq(&table.table_name))
        .filter(columns::table_schema.eq(&table.table_schema))
        .filter(columns::table_catalog.eq(&table.table_catalog))
        .filter(columns::column_name.eq(column_name))
        .first::<Column>(conn)?)
}
