//! Submodule defining the cached queries methods used in the
//! [`CheckConstraint`] struct.

use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};

use crate::models::{CheckConstraint, TableConstraint};

pub fn table_constraint(
    check_constraint: &CheckConstraint,
    conn: &mut PgConnection,
) -> Result<TableConstraint, diesel::result::Error> {
    use crate::schema::information_schema::table_constraints::table_constraints;

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
