//! Submodule providing a struct [`PgConstraint`] representing the
//! `pg_constraint` table.

use cached::{DiskCache, proc_macro::io_cached};
use diesel::{
    ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, Queryable, QueryableByName, RunQueryDsl,
    Selectable, SelectableHelper,
};

use super::{PgOperator, PgProc};
use crate::errors::WebCodeGenError;

/// Represents the `pg_constraint` system catalog table in `PostgreSQL`.
/// This table stores information about constraints on tables and columns.
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::pg_constraint)]
#[allow(clippy::struct_excessive_bools)]
pub struct PgConstraint {
    /// The OID of the constraint.
    pub oid: u32,
    /// The name of the constraint.
    pub conname: String,
    /// The OID of the namespace that contains this constraint.
    pub connamespace: u32,
    /// The type of the constraint: c = check constraint, f = foreign key
    /// constraint, n = not-null constraint (domains only), p = primary key
    /// constraint, u = unique constraint, t = constraint trigger, x = exclusion
    /// constraint
    pub contype: String,
    /// Whether the constraint is deferrable
    pub condeferrable: bool,
    /// Whether the constraint is deferred
    pub condeferred: bool,
    /// Whether the constraint is validated
    pub convalidated: bool,
    /// The OID of the table that the constraint is on
    pub conrelid: u32,
    /// The domain this constraint is on; zero if not a domain constraint
    pub contypid: u32,
    /// The OID of the index that the constraint is on
    pub conindid: u32,
    /// The corresponding constraint of the parent partitioned table, if this is
    /// a constraint on a partition; else zero
    pub conparentid: u32,
    /// If a foreign key, the referenced table; else zero
    pub confrelid: u32,
    /// Foreign key update action code: a = no action, r = restrict, c =
    /// cascade, n = set null, d = set default
    pub confupdtype: String,
    /// Foreign key deletion action code: a = no action, r = restrict, c =
    /// cascade, n = set null, d = set default
    pub confdeltype: String,
    /// Foreign key match type: f = full, p = partial, s = simple
    pub confmatchtype: String,
    /// This constraint is defined locally for the relation. Note that a
    /// constraint can be locally defined and inherited simultaneously.
    pub conislocal: bool,
    /// The number of direct inheritance ancestors this constraint has. A
    /// constraint with a nonzero number of ancestors cannot be dropped nor
    /// renamed.
    pub coninhcount: i16,
    /// This constraint is defined locally for the relation. It is a
    /// non-inheritable constraint.
    pub connoinherit: bool,
    /// If a table constraint (including foreign keys, but not constraint
    /// triggers), list of the constrained columns
    pub conkey: Option<Vec<i16>>,
    /// If a foreign key, list of the referenced columns
    pub confkey: Option<Vec<i16>>,
    /// If a foreign key, list of the equality operators for PK = FK comparisons
    pub conpfeqop: Option<Vec<u32>>,
    /// If a foreign key, list of the equality operators for PK = PK comparisons
    pub conppeqop: Option<Vec<u32>>,
    /// If a foreign key, list of the equality operators for FK = FK comparisons
    pub conffeqop: Option<Vec<u32>>,
    /// If a foreign key with a SET NULL or SET DEFAULT delete action, the
    /// columns that will be updated. If null, all of the referencing columns
    /// will be updated.
    pub confdelsetcols: Option<Vec<i16>>,
    /// If an exclusion constraint, list of the per-column exclusion operators
    pub conexclop: Option<Vec<u32>>,
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("pg_constraints.functions")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "u32",
    convert = r#"{ pg_constraint.oid }"#
)]
fn functions(
    pg_constraint: &PgConstraint,
    conn: &mut PgConnection,
) -> Result<Vec<PgProc>, WebCodeGenError> {
    use crate::schema::{pg_constraint, pg_depend, pg_proc};
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

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("pg_constraints.operators")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "u32",
    convert = r#"{ pg_constraint.oid }"#
)]
fn operators(
    pg_constraint: &PgConstraint,
    conn: &mut PgConnection,
) -> Result<Vec<PgOperator>, WebCodeGenError> {
    use crate::schema::{pg_constraint, pg_depend, pg_operator};
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

impl PgConstraint {
    /// Returns the vector of [`PgProc`] functions that are used in the
    /// constraint.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn functions(&self, conn: &mut PgConnection) -> Result<Vec<PgProc>, WebCodeGenError> {
        functions(self, conn)
    }

    /// Returns the vector of [`PgOperator`] functions that are used in the
    /// constraint.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn operators(&self, conn: &mut PgConnection) -> Result<Vec<PgOperator>, WebCodeGenError> {
        operators(self, conn)
    }
}
