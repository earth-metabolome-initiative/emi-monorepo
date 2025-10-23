//! Submodule defining the cached queries methods used in the [`KeyColumnUsage`]
//! struct.

use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};

use crate::models::{Column, KeyColumnUsage, ReferentialConstraint, Table};

#[pg_cached::auto_cached]
pub(super) fn referential_constraint(
    key_column_usage: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<ReferentialConstraint, diesel::result::Error> {
    use crate::schema::information_schema::referential_constraints::referential_constraints;
    Ok(referential_constraints::table
        .filter(referential_constraints::constraint_name.eq(&key_column_usage.constraint_name))
        .filter(referential_constraints::constraint_schema.eq(&key_column_usage.constraint_schema))
        .filter(
            referential_constraints::constraint_catalog.eq(&key_column_usage.constraint_catalog),
        )
        .select(ReferentialConstraint::as_select())
        .first::<ReferentialConstraint>(conn)?)
}

#[pg_cached::auto_cached]
pub(super) fn referenced_columns(
    key_column_usage: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<Vec<Column>, diesel::result::Error> {
    use crate::schema::information_schema::{
        columns::columns, constraint_column_usage::constraint_column_usage,
    };

    // Find the referential constraint for this key_column_usage
    let referential_constraint = referential_constraint(key_column_usage, conn)?;

    // Find the columns in the referenced (unique) constraint
    constraint_column_usage::table
        .filter(
            constraint_column_usage::constraint_catalog.eq(referential_constraint
                .unique_constraint_catalog
                .ok_or(diesel::result::Error::NotFound)?),
        )
        .filter(
            constraint_column_usage::constraint_schema.eq(referential_constraint
                .unique_constraint_schema
                .ok_or(diesel::result::Error::NotFound)?),
        )
        .filter(constraint_column_usage::constraint_name.eq(
            referential_constraint.unique_constraint_name.ok_or(diesel::result::Error::NotFound)?,
        ))
        .inner_join(
            columns::table.on(columns::table_name
                .eq(constraint_column_usage::table_name)
                .and(columns::table_schema.eq(constraint_column_usage::table_schema))
                .and(columns::table_catalog.eq(constraint_column_usage::table_catalog))
                .and(columns::column_name.eq(constraint_column_usage::column_name))),
        )
        .order_by(columns::ordinal_position.asc())
        .select(Column::as_select())
        .load::<Column>(conn)
}

#[pg_cached::auto_cached]
pub(crate) fn host_columns(
    key_column_usage: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<Vec<Column>, diesel::result::Error> {
    use crate::schema::information_schema::{columns::columns, key_column_usage::key_column_usage};
    Ok(key_column_usage::table
        .filter(key_column_usage::constraint_name.eq(&key_column_usage.constraint_name))
        .filter(key_column_usage::constraint_schema.eq(&key_column_usage.constraint_schema))
        .filter(key_column_usage::constraint_catalog.eq(&key_column_usage.constraint_catalog))
        .inner_join(
            columns::table.on(columns::table_name
                .eq(key_column_usage::table_name)
                .and(columns::table_schema.eq(key_column_usage::table_schema))
                .and(columns::table_catalog.eq(key_column_usage::table_catalog))
                .and(columns::column_name.eq(key_column_usage::column_name))),
        )
        .order_by(key_column_usage::ordinal_position.asc())
        .select(Column::as_select())
        .load::<Column>(conn)?)
}

#[pg_cached::auto_cached]
/// Returns the referenced table associated with this key column usage
///
/// # Arguments
///
/// * `conn` - A mutable reference to a `PgConnection`
///
/// # Errors
///
/// * If an error occurs while loading the referenced table from the database
pub(super) fn referenced_table(
    key_column_usage: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<Table, diesel::result::Error> {
    use crate::schema::information_schema::{
        constraint_table_usage::constraint_table_usage, tables::tables,
    };

    let constraint = referential_constraint(key_column_usage, conn)?;

    Ok(constraint_table_usage::table
        .inner_join(
            tables::table.on(tables::table_name
                .eq(constraint_table_usage::table_name)
                .and(tables::table_schema.eq(constraint_table_usage::table_schema))
                .and(tables::table_catalog.eq(constraint_table_usage::table_catalog))),
        )
        .filter(constraint_table_usage::constraint_name.eq(&constraint.constraint_name))
        .filter(constraint_table_usage::constraint_schema.eq(&constraint.constraint_schema))
        .filter(constraint_table_usage::constraint_catalog.eq(&constraint.constraint_catalog))
        .select(Table::as_select())
        .first::<Table>(conn)?)
}
