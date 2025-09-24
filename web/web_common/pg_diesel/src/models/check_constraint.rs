use std::fmt::{Debug, Display};

use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, Queryable,
    QueryableByName, Selectable, SelectableHelper,
};

use super::{Column, PgConstraint, PgOperator, PgProc};
use crate::models::{Table, TableConstraint};

mod cached_queries;

#[derive(
    Queryable, QueryableByName, Debug, Clone, Selectable, Ord, PartialEq, Eq, Hash, PartialOrd,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::check_constraints)]
/// A struct representing a check constraint
pub struct CheckConstraint {
    /// The name of the constraint catalog
    pub constraint_catalog: String,
    /// The name of the constraint schema
    pub constraint_schema: String,
    /// The name of the constraint
    pub constraint_name: String,
    /// The check clause of the constraint
    pub check_clause: String,
}

impl Display for CheckConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.constraint_catalog, self.constraint_schema, self.constraint_name)
    }
}

const POSTGIS_CONSTRAINTS: [&str; 1] = ["spatial_ref_sys_srid_check"];

impl CheckConstraint {
    #[must_use]
    /// Returns whether the current [`CheckConstraint`] is known to come
    /// from Postgis and therefore should most likely be ignored
    pub fn is_postgis_constraint(&self) -> bool {
        POSTGIS_CONSTRAINTS.contains(&self.constraint_name.as_str())
    }

    /// Returns the vector of [`PgProc`] functions that are used in the check
    /// clause
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn functions(&self, conn: &mut PgConnection) -> Result<Vec<PgProc>, diesel::result::Error> {
        self.pg_constraint(conn)?.functions(conn)
    }

    /// Returns the vector of [`PgOperator`] operators that are used in the
    /// check clause
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn operators(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgOperator>, diesel::result::Error> {
        self.pg_constraint(conn)?.operators(conn)
    }

    /// Returns the [`PgConstraint`] that corresponds to this check constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn pg_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<PgConstraint, diesel::result::Error> {
        use diesel::RunQueryDsl;

        use crate::schema::{pg_constraint, pg_namespace};
        pg_constraint::table
            .inner_join(pg_namespace::table.on(pg_constraint::connamespace.eq(pg_namespace::oid)))
            .filter(
                pg_constraint::conname
                    .eq(&self.constraint_name)
                    .and(pg_constraint::contype.eq("c")),
            )
            .filter(pg_namespace::nspname.eq(&self.constraint_schema))
            .select(PgConstraint::as_select())
            .first(conn)
            .map_err(diesel::result::Error::from)
    }

    /// Returns the table constraint associated with this check constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn table_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TableConstraint, diesel::result::Error> {
        cached_queries::table_constraint(self, conn)
    }

    /// Returns the table that this check constraint belongs to
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn table(&self, conn: &mut PgConnection) -> Result<Table, diesel::result::Error> {
        self.table_constraint(conn)?.table(conn)
    }

    /// Returns all the columns associated to this check constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If their is an error while querying the database.
    pub fn columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, diesel::result::Error> {
        cached_queries::columns(self, conn)
    }
}
