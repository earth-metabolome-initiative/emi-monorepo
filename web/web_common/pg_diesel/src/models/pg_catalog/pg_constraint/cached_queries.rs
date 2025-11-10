//! Submodule defining the cached queries methods used in the [`PgConstraint`]
//! struct.

use diesel::{ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::models::{PgConstraint, PgOperator, PgProc};

pub(super) fn functions(
    pg_constraint: &PgConstraint,
    conn: &mut PgConnection,
) -> Result<Vec<PgProc>, diesel::result::Error> {
    use crate::schema::pg_catalog::{
        pg_constraint::pg_constraint, pg_depend::pg_depend, pg_proc::pg_proc,
    };
    Ok(pg_constraint::table
        // Join to pg_depend where the constraint's OID is recorded as the dependent.
        .inner_join(pg_depend::table.on(pg_constraint::oid.eq(pg_depend::objid)))
        // Then join to pg_proc using the referenced function OID.
        .inner_join(pg_proc::table.on(pg_depend::refobjid.eq(pg_proc::oid)))
        // Filter for this specific constraint.
        .filter(pg_constraint::oid.eq(pg_constraint.oid))
        // Select all columns from pg_proc.
        .select(PgProc::as_select())
        .load::<PgProc>(conn)?)
}

pub(super) fn operators(
    pg_constraint: &PgConstraint,
    conn: &mut PgConnection,
) -> Result<Vec<PgOperator>, diesel::result::Error> {
    use crate::schema::pg_catalog::{
        pg_constraint::pg_constraint, pg_depend::pg_depend, pg_operator::pg_operator,
    };
    Ok(pg_constraint::table
        // Join to pg_depend where the constraint's OID is recorded as the dependent.
        .inner_join(pg_depend::table.on(pg_constraint::oid.eq(pg_depend::objid)))
        // Then join to pg_operator using the referenced operator OID.
        .inner_join(pg_operator::table.on(pg_depend::refobjid.eq(pg_operator::oid)))
        // Filter for this specific constraint.
        .filter(pg_constraint::oid.eq(pg_constraint.oid))
        // Select all columns from pg_operator.
        .select(PgOperator::as_select())
        .load::<PgOperator>(conn)?)
}
